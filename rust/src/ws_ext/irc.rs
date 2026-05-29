use std::collections::VecDeque;
use std::io::{BufRead, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

const INITIAL_RECONNECT_DELAY_MS: u64 = 1000;
const MAX_RECONNECT_DELAY_MS: u64 = 60000;
const SEND_DELAY_MS: u64 = 500;
const MAX_MSG_CHUNK_SIZE: usize = 400;
const MAX_TOTAL_RECONNECT_ATTEMPTS: u32 = 100;
const NICK_COLLISION_MAX_RETRIES: u32 = 3;

struct IrcState {
    writer: Option<Arc<Mutex<TcpStream>>>,
    messages: VecDeque<String>,
    connected: bool,
    channel: String,
    auth_secret: String,
    authed_nick: Option<String>,
    running: bool,
    reconnect_attempts: u32,
}

static STATE: OnceLock<Arc<Mutex<IrcState>>> = OnceLock::new();

fn state() -> &'static Arc<Mutex<IrcState>> {
    STATE.get_or_init(|| {
        Arc::new(Mutex::new(IrcState {
            writer: None,
            messages: VecDeque::new(),
            connected: false,
            channel: String::new(),
            auth_secret: String::new(),
            authed_nick: None,
            running: false,
            reconnect_attempts: 0,
        }))
    })
}

fn normalize_nick(nick: &str) -> String {
    nick.trim().to_lowercase()
}

fn is_allowed_msg(secret: &str, authed: &Option<String>, nick: &str, text: &str) -> &'static str {
    if secret.is_empty() {
        return "allow";
    }
    let lower = text.trim().to_lowercase();
    let candidate = if lower.starts_with("auth ") {
        text.trim()[5..].trim().to_string()
    } else if lower.starts_with("/auth ") {
        text.trim()[6..].trim().to_string()
    } else {
        return if authed.is_none() {
            "ignore"
        } else if normalize_nick(nick) == *authed.as_deref().unwrap_or("") {
            "allow"
        } else {
            "ignore"
        };
    };
    if candidate == secret {
        if authed.is_none() {
            return "auth_bound";
        }
        return "ignore";
    }
    "ignore"
}

fn irc_send_raw(writer: &Arc<Mutex<TcpStream>>, cmd: &str) -> bool {
    eprintln!("[IRC] >>> {}", cmd);
    if let Ok(mut w) = writer.lock() {
        let _ = w.write_all(format!("{}\r\n", cmd).as_bytes());
        let _ = w.flush();
        std::thread::sleep(Duration::from_millis(SEND_DELAY_MS));
        true
    } else {
        false
    }
}

fn irc_loop(server: String, port: u16, mut nick: String, channel: String, _auth_secret: String) {
    eprintln!("[IRC] irc_loop ENTERED srv={} port={} nick={} ch={}", server, port, nick, channel);

    let mut reconnect_delay = INITIAL_RECONNECT_DELAY_MS;

    loop {
        {
            let s = state().lock().unwrap();
            if !s.running {
                eprintln!("[IRC] running=false, exiting irc_loop");
                break;
            }
            if s.reconnect_attempts >= MAX_TOTAL_RECONNECT_ATTEMPTS {
                eprintln!("[IRC] Max total reconnect attempts reached, exiting");
                break;
            }
        }

        eprintln!("[IRC] Attempting connection (attempt {})...", {
            let s = state().lock().unwrap();
            s.reconnect_attempts + 1
        });

        let sock = match TcpStream::connect(format!("{}:{}", server, port)) {
            Ok(s) => {
                eprintln!("[IRC] TCP connected");
                s
            }
            Err(e) => {
                eprintln!("[IRC] Connect failed: {}", e);
                let delay = {
                    let mut s = state().lock().unwrap();
                    if !s.running {
                        break;
                    }
                    s.reconnect_attempts += 1;
                    eprintln!("[IRC] Retrying in {}ms", reconnect_delay);
                    reconnect_delay
                };
                std::thread::sleep(Duration::from_millis(delay));
                reconnect_delay = (reconnect_delay * 2).min(MAX_RECONNECT_DELAY_MS);
                continue;
            }
        };

        let _ = sock.set_read_timeout(Some(Duration::from_secs(120)));
        let _ = sock.set_write_timeout(Some(Duration::from_secs(10)));
        let _ = sock.set_nodelay(true).ok();
        let writer = Arc::new(Mutex::new(sock.try_clone().unwrap()));

        {
            let mut s = state().lock().unwrap();
            s.writer = Some(writer.clone());
            s.reconnect_attempts = 0;
        }

        let irc_send = |cmd: &str| irc_send_raw(&writer, cmd);

        irc_send(&format!("NICK {}", nick));
        irc_send(&format!("USER {} 0 * :{}", nick, nick));

        let reader = std::io::BufReader::new(sock);
        let mut nick_attempts = 0;
        let mut registered = false;
        let mut disconnect_detected = false;

        eprintln!("[IRC] Waiting for registration...");
        for line_exp in reader.lines() {
            match line_exp {
                Ok(line) => {
                    eprintln!("[IRC] <<< {}", line);
                    {
                        let s = state().lock().unwrap();
                        if !s.running {
                            break;
                        }
                    }

                    if line.starts_with("PING") {
                        let token = line.split_whitespace().nth(1).unwrap_or("");
                        irc_send(&format!("PONG {}", token));
                        continue;
                    }

                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        match parts[1] {
                            "001" => {
                                let ch = {
                                    let mut s = state().lock().unwrap();
                                    s.connected = true;
                                    s.channel.clone()
                                };
                                eprintln!("[IRC] Registered! Joining channel");
                                irc_send(&format!("JOIN {}", ch));
                                registered = true;
                                reconnect_delay = INITIAL_RECONNECT_DELAY_MS;
                            }
                            "403" | "405" | "471" | "473" | "474" | "475" => {
                                eprintln!("[IRC] Join failed: {}", line);
                                disconnect_detected = true;
                            }
                            "433" => {
                                eprintln!("[IRC] Nickname in use: {}", line);
                                if nick_attempts < NICK_COLLISION_MAX_RETRIES {
                                    nick_attempts += 1;
                                    nick = format!("{}{}", nick.trim_end_matches(|c: char| c.is_ascii_digit()), rand_suffix());
                                    eprintln!("[IRC] Retrying with nick: {}", nick);
                                    irc_send(&format!("NICK {}", nick));
                                } else {
                                    eprintln!("[IRC] Max nick collision attempts reached");
                                    disconnect_detected = true;
                                }
                            }
                            _ => {}
                        }
                    }

                    if let Some((prefix, trailing)) = line.split_once(" PRIVMSG ") {
                        let nick_from = prefix.split('!').next().unwrap_or("");
                        if let Some((_, msg_text)) = trailing.split_once(" :") {
                            let mut s = state().lock().unwrap();
                            let result =
                                is_allowed_msg(&s.auth_secret, &s.authed_nick, nick_from, msg_text);
                            match result {
                                "allow" => {
                                    s.messages.push_back(format!("{}: {}", nick_from, msg_text));
                                }
                                "auth_bound" => {
                                    irc_send(&format!(
                                        "PRIVMSG {} :Authentication successful for {}.",
                                        channel, nick_from
                                    ));
                                    s.authed_nick = Some(normalize_nick(nick_from));
                                }
                                _ => {}
                            }
                        }
                    }

                    if disconnect_detected {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("[IRC] Read error: {}", e);
                    break;
                }
            }
        }

        if registered {
            eprintln!("[IRC] Server closed connection, will reconnect");
        }
        {
            let mut s = state().lock().unwrap();
            s.connected = false;
            s.writer = None;
        }

        {
            let s = state().lock().unwrap();
            if !s.running {
                eprintln!("[IRC] Stopping (running=false)");
                break;
            }
            if s.reconnect_attempts >= MAX_TOTAL_RECONNECT_ATTEMPTS {
                eprintln!("[IRC] Max total reconnect attempts reached, exiting");
                break;
            }
        }

        eprintln!("[IRC] Reconnecting in {}ms...", reconnect_delay);
        std::thread::sleep(Duration::from_millis(reconnect_delay));
        reconnect_delay = (reconnect_delay * 2).min(MAX_RECONNECT_DELAY_MS);
    }

    {
        let mut s = state().lock().unwrap();
        s.connected = false;
        s.writer = None;
        s.running = false;
    }
    eprintln!("[IRC] Disconnected");
}

fn rand_suffix() -> String {
    let n = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos()
        % 9000
        + 1000) as u16;
    n.to_string()
}

pub fn connect(
    server: &str,
    port: u16,
    nick: &str,
    channel: &str,
    auth_secret: &str,
) -> Result<String, String> {
    eprintln!("[IRC] connect called: srv={} port={} nick={} ch={}", server, port, nick, channel);

    let nick = format!("{}{}", nick, rand_suffix());
    let ch = if channel.starts_with('#') { channel.to_string() } else { format!("#{}", channel) };

    {
        let mut s = state().lock().unwrap();
        s.channel = ch.clone();
        s.auth_secret = auth_secret.to_string();
        s.authed_nick = None;
        s.connected = false;
        s.running = true;
        s.messages.clear();
        s.writer = None;
    }

    let srv = server.to_string();
    let n = nick.clone();
    let c = ch.clone();
    let a = auth_secret.to_string();

    std::thread::Builder::new()
        .name("irc-connection".into())
        .spawn(move || irc_loop(srv, port, n, c, a))
        .map_err(|e| format!("Failed to spawn IRC thread: {}", e))?;

    eprintln!("[IRC] Thread spawned, returning: {} as {} on {}", ch, nick, server);
    Ok(format!("Connecting to {} as {} on {}", ch, nick, server))
}

pub fn send(msg: &str) -> Result<String, String> {
    let (channel, writer) = {
        let s = state().lock().unwrap();
        if !s.connected {
            return Err("IRC not connected".into());
        }
        (s.channel.clone(), s.writer.clone())
    };

    let writer = writer.ok_or("IRC writer unavailable")?;

    let segments: Vec<&str> = msg.split("\\n").collect();
    for segment in segments {
        let clean = segment.replace('\n', " ");
        let mut chars = clean.chars().peekable();
        while let Some(_) = chars.peek() {
            let mut byte_pos = 0;
            let mut chunk_chars: Vec<char> = Vec::new();
            while let Some(c) = chars.peek() {
                let char_len = c.len_utf8();
                if byte_pos + char_len > MAX_MSG_CHUNK_SIZE && !chunk_chars.is_empty() {
                    break;
                }
                chunk_chars.push(*c);
                byte_pos += char_len;
                chars.next();
            }
            let chunk_str: String = chunk_chars.into_iter().collect();
            if chunk_str.is_empty() {
                continue;
            }
            let cmd = format!("PRIVMSG {} :{}", channel, chunk_str);
            if let Ok(mut wlock) = writer.lock() {
                let _ = wlock.write_all(format!("{}\r\n", cmd).as_bytes());
                let _ = wlock.flush();
                std::thread::sleep(Duration::from_millis(SEND_DELAY_MS));
            } else {
                return Err("Failed to acquire writer lock".into());
            }
        }
    }
    Ok("ok".into())
}

pub fn recv() -> Result<String, String> {
    let mut s = state().lock().unwrap();
    let mut msgs = Vec::new();
    while let Some(msg) = s.messages.pop_front() {
        msgs.push(msg);
    }
    Ok(msgs.join(" | "))
}

pub fn stop() -> Result<String, String> {
    state().lock().unwrap().running = false;
    Ok("stopped".into())
}
