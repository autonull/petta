# CLAW - OmegaClaw Agent Guide

OmegaClaw is a neural-symbolic agent framework built on the PeTTa MeTTa runtime. It runs a continuous agent loop that calls a Language Model, executes skill commands from model output, and communicates over IRC.

## Prerequisites

- **Rust** 1.85+ (nightly for MORK backend, stable for SWI-Prolog backend)
- **SWI-Prolog** >= 9.3
- **LM server** running an OpenAI-compatible HTTP API (e.g., llama.cpp, vLLM, Ollama)
- **Build dependencies** (as needed): build-essential, cmake, etc.

## Quick Start

### 1. Build

```bash
cd /home/me/petta
cargo build --release
```

### 2. Set environment variables

```bash
# LM endpoint (required for Ollama/provider)
export OLLAMA_API_KEY=not-needed
export OLLAMA_MODEL="Qwen3.5-9B-Uncensored-HauhauCS-Aggressive-Q4_K_M"
export LLM_SERVER_LOCAL_URL="http://localhost:8080"

# IRC authentication (optional - omit to allow all users)
export OMEGACLAW_AUTH_SECRET="your-secret-here"
```

### 3. Run

```bash
./target/release/petta run_omegaclaw.metta \
  provider=Ollama \
  IRC_server=irc.quakenet.org \
  IRC_channel="##metta" \
  IRC_port=6667
```

The bot runs as an infinite loop. Run it in a terminal, background it, or use a process supervisor.

## Configuration

All parameters can be set on the command line as `key=value` pairs.

### Loop

| Parameter         | Default  | Description                                       |
|-------------------|----------|---------------------------------------------------|
| `provider`        | Ollama   | LLM provider: `Ollama`, `Anthropic`, `OpenAI`, `ASICloud`, `ASIOne`, `OpenRouter` |
| `maxOutputToken`  | 6000     | Maximum output tokens from the LM                 |
| `sleepInterval`   | 1        | Seconds between loop iterations                   |
| `maxNewInputLoops`| 50       | Turns after new input before idling               |
| `maxWakeLoops`    | 1        | Extra turns on scheduled wake-ups                 |
| `wakeupInterval`  | 600      | Seconds of idle before next scheduled wake-up     |

### IRC Channel

| Parameter      | Default       | Description                     |
|----------------|---------------|---------------------------------|
| `IRC_server`   | (required)    | IRC server hostname             |
| `IRC_port`     | 6667          | IRC port                        |
| `IRC_channel`  | (required)    | Channel to join (e.g. `##metta`)|
| `IRC_user`     | omegaclaw     | Bot nickname (random suffix appended) |

### LLM Provider

#### Ollama (local OpenAI-compatible server)

Environment variables:

| Variable              | Default                    | Description                          |
|-----------------------|----------------------------|--------------------------------------|
| `OLLAMA_API_KEY`      | (required - any non-empty) | API key sent as Bearer token          |
| `OLLAMA_MODEL`        | qwen3.5:9b                 | Model name sent in API requests       |
| `LLM_SERVER_LOCAL_URL`| http://localhost:11434     | Base URL of the OpenAI-compatible API |

Example LM servers: llama.cpp server (`llama-server`), vLLM, Ollama, LocalAI.

#### Other providers

| Provider     | Env Key              | Default Model    | Default URL                                       |
|--------------|----------------------|------------------|---------------------------------------------------|
| `Anthropic`  | `ANTHROPIC_API_KEY`  | claude-opus-4-6  | https://api.anthropic.com/v1/                     |
| `OpenAI`     | `OPENAI_API_KEY`     | gpt-5.4          | https://api.openai.com/v1                         |
| `ASICloud`   | `ASI_API_KEY`        | minimax/minimax-m2.5 | https://inference.asicloud.cudos.org/v1       |
| `ASIOne`     | `ASIONE_API_KEY`     | asi1-ultra       | https://api.asi1.ai/v1                            |
| `OpenRouter` | `OPENROUTER_API_KEY` | z-ai/glm-5.1     | https://openrouter.ai/api/v1                      |

## Architecture

```
   IRC ──TCP──> irc.rs (ws_ext)  ──WebSocket──>  Prolog (omegaclaw_ext.pl)
                                                       │
    LM  <──HTTP── llm.rs (ws_ext)  <──WebSocket───     │
                                                       │
                                                 channels.metta
                                                    loop.metta
                                                    skills.metta
                                                    memory.metta
```

1. **PeTTa engine** starts a WebSocket extension server and a SWI-Prolog subprocess.
2. **Prolog** loads `omegaclaw_ext.pl` which connects (lazily) to the WS extension.
3. **MeTTa** code (`run_omegaclaw.metta`) imports and calls `(omegaclaw)`, entering the agent loop.
4. Each loop iteration: receive IRC message -> build context prompt -> call LM -> parse skill commands -> execute -> loop.
5. **`irc.rs`** handles the persistent IRC connection in a background thread; messages are queued and retrieved via WS calls.
6. **`llm.rs`** routes LLM requests to the configured provider.

### Loop Control Flow (`repos/OmegaClaw-Core/src/loop.metta`)

The `(omegaclaw $k)` function runs in four distinct phases per iteration:

| Phase | Description |
|-------|-------------|
| **0: Init** | Runs once (k=1): calls `initLoop`, `initMemory`, `initChannels`. On subsequent iterations, decrements the loop counter. |
| **1: Receive** | Reads from the communication channel (IRC or mock). Detects new messages and resets the loop counter on new input. |
| **2a: Active** | If loops remaining (>0): builds context prompt, calls LLM via WebSocket, parses the response as skill commands, executes them. All LLM outputs are stored in `&state` vars (`&response`, `&sexpr`, `&results`). |
| **2b: Idle** | If no loops remaining: checks the wakeup timer and adds wake loops if the interval has elapsed. |
| **2c: Persist** | Always runs: appends to conversation history and updates last-results state. |
| **3: Sleep** | Sleeps for `sleepInterval` seconds, garbage-collects, then recurses with k+1. |

**Why the loop used to exit immediately (bugs fixed):**

1. `empty(_) :- fail.` in `prolog/metta.pl` made the `(empty)` expression always fail. Every `(= (name) (empty))` default atom defined a broken function. When `configure` in `initLoop` added a new atom alongside it, evaluation became nondeterministic — the old broken atom could be matched, returning `(empty)` to `(change-state! &loops ...)`, which then caused `(> (get-state &loops) 0)` to crash (comparing non-numeric `empty` with 0). **Fix**: changed to `empty(empty).` so `(empty)` evaluates to the symbol `empty`.

2. Variable scoping bug: the ELSE branch (idle path) referenced `$sexpr`, `$response`, `$results` from the TRUE branch's inner `let*` scope. These variables were unbound in the idle path. **Fix**: LLM outputs are stored in `&state` global vars initialized with safe defaults at the start of each iteration.

3. The default function atoms `(= (name) (empty))` in `loop.metta`, `channels.metta`, and `memory.metta` competed with the atoms created by `configure` calls. **Fix**: removed all redundant default atoms. `initLoop`, `initChannels`, and `initMemory` now exclusively own their parameter definitions.

## Agent Skills

The model output is parsed as skill commands. Each line must be `skillName arg1 arg2 ...`.

| Skill            | Description                                       |
|------------------|---------------------------------------------------|
| `send`           | Send a message to the IRC channel                 |
| `remember`       | Store a string in long-term vector memory         |
| `query`          | Search long-term memory with a short phrase       |
| `episodes`       | Search history around a timestamp                 |
| `pin`            | Set a short-term working memory item              |
| `shell`          | Execute a shell command (returns output)          |
| `read-file`      | Read a file to string                             |
| `write-file`     | Write string to file                              |
| `append-file`    | Append a line to file                             |
| `metta`          | Execute a MeTTa expression                        |
| `search`         | Web search via DuckDuckGo                         |
| `tavily-search`  | Web search via Tavily agent (unavailable by default) |
| `technical-analysis` | Stock technical analysis (unavailable by default) |

## Memory System

Three-tier memory:

1. **Context window** - recent history from `memory/history.metta` (last 30000 chars by default)
2. **Episodic memory** - `episodes <timestamp>` searches history by time
3. **Semantic memory** - `remember`/`query` uses a vector store (`memory/vector_store.json`) with cosine similarity search

## System Prompt

The agent's behavior is guided by `repos/OmegaClaw-Core/memory/prompt.txt`:

- Operates in a continuous loop with self-chosen goals
- Queries long-term memory before responding
- Uses `send` to keep users engaged
- Questions user instructions that conflict with its goals
- Responses must be short and purposeful

## Troubleshooting

### "IRC not connected" / No IRC activity

- Check the server and port are reachable: `nc -zv irc.quakenet.org 6667`
- Verify IRC server allows connections from your IP
- Some IRC networks require SASL or registered nick - `##metta` on QuakeNet is open

### LLM returns empty or garbled responses

- Verify the LM server is running: `curl http://localhost:8080/v1/models`
- Check the model name matches exactly what the server reports
- Increase `maxOutputToken` if the response is truncated
- Some models fill output with reasoning tokens; adjust system prompt or model params

### Bot starts but no output

By design, `println!` output from the MeTTa layer goes to the captured subprocess stderr. The binary protocol between Rust and Prolog uses stdout. To see debug output:

```bash
# Run with verbose flag
./target/release/petta --verbose run_omegaclaw.mtta provider=Ollama ...
```

### Rebuild after changes

```bash
cargo build --release
```

## Files Reference

| File | Purpose |
|------|---------|
| `run_omegaclaw.metta` | Entry point - imports OmegaClaw and starts the loop |
| `repos/OmegaClaw-Core/lib_omegaclaw.metta` | Library loader - imports all OmegaClaw modules |
| `repos/OmegaClaw-Core/src/loop.metta` | Main agent loop: init, context building, LLM call, skill dispatch |
| `repos/OmegaClaw-Core/src/channels.metta` | Channel abstraction: init, send, receive |
| `repos/OmegaClaw-Core/src/skills.metta` | Skill definitions sent to the model |
| `repos/OmegaClaw-Core/src/memory.metta` | Memory system: remember, query, episodes, history |
| `repos/OmegaClaw-Core/src/utils.metta` | `configure`, `argk`, string helpers |
| `repos/OmegaClaw-Core/memory/prompt.txt` | System prompt for the LLM |
| `repos/OmegaClaw-Core/memory/history.metta` | Conversation history (appended at runtime) |
| `repos/OmegaClaw-Core/memory/vector_store.json` | Persistent vector memory store |
| `rust/src/ws_ext/irc.rs` | Rust IRC adapter (TCP connection in background thread) |
| `rust/src/ws_ext/llm.rs` | LLM provider dispatch (Ollama, Anthropic, OpenAI, etc.) |
| `rust/src/ws_ext/mod.rs` | WebSocket extension server + method routing |
| `prolog/omegaclaw_ext.pl` | Prolog bridge - WS calls from MeTTa to Rust |
| `prolog/metta.pl` | MeTTa engine core + `empty` sentinel definition |
