//! Protocol client for PeTTa ↔ Prolog communication

use std::io::{BufReader, Read, Write};
use std::time::Instant;

use super::config::EngineConfig;
use super::errors::{PeTTaError, parse_backend_error};
use crate::values::MettaResult;

fn send_query(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut BufReader<std::process::ChildStdout>,
    query_type: u8,
    payload: &str,
    config: &EngineConfig,
) -> Result<Vec<MettaResult>, PeTTaError> {
    let start = Instant::now();
    let pb = payload.as_bytes();
    
    stdin.write_all(&[query_type])
        .map_err(|e| PeTTaError::WriteError(e.to_string()))?;
    stdin.write_all(&(pb.len() as u32).to_be_bytes())
        .map_err(|e| PeTTaError::WriteError(e.to_string()))?;
    stdin.write_all(pb)
        .map_err(|e| PeTTaError::WriteError(e.to_string()))?;
    stdin.flush().map_err(|e| PeTTaError::WriteError(e.to_string()))?;
    
    check_timeout(start, config)?;
    
    // Read status byte, skipping any stray ready signals (0xFF)
    let mut b = [0u8; 1];
    loop {
        read_exact(stdout, &mut b)?;
        if b[0] != 0xFF {
            break;
        }
    }
    
    match b[0] {
        0 => {
            let count = read_u32(stdout)?;
            let mut results = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let len = read_u32(stdout)?;
                let mut buf = vec![0u8; len as usize];
                read_exact(stdout, &mut buf)?;
                let value = String::from_utf8(buf)
                    .map_err(|e| PeTTaError::Protocol(e.to_string()))?;
                results.push(MettaResult { value });
            }
            Ok(results)
        }
        1 => {
            let len = read_u32(stdout)?;
            let mut buf = vec![0u8; len as usize];
            read_exact(stdout, &mut buf)?;
            let msg = String::from_utf8_lossy(&buf);
            Err(PeTTaError::Backend(parse_backend_error(&msg)))
        }
        s => Err(PeTTaError::Protocol(format!("unknown status: {s}"))),
    }
}

fn read_exact<R: Read>(r: &mut R, buf: &mut [u8]) -> Result<(), PeTTaError> {
    r.read_exact(buf).map_err(|e| {
        if e.kind() == std::io::ErrorKind::UnexpectedEof {
            PeTTaError::Protocol("child closed".into())
        } else {
            PeTTaError::Protocol(e.to_string())
        }
    })
}

fn read_u32<R: Read>(r: &mut R) -> Result<u32, PeTTaError> {
    let mut b = [0u8; 4];
    read_exact(r, &mut b)?;
    Ok(u32::from_be_bytes(b))
}

fn check_timeout(start: Instant, config: &EngineConfig) -> Result<(), PeTTaError> {
    if let Some(timeout) = config.timeout {
        if start.elapsed() >= timeout {
            return Err(PeTTaError::Timeout(timeout));
        }
    }
    Ok(())
}

pub fn load_metta_file(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut BufReader<std::process::ChildStdout>,
    path: &std::path::Path,
    config: &EngineConfig,
) -> Result<Vec<MettaResult>, PeTTaError> {
    let abs = path.canonicalize().map_err(|e| PeTTaError::PathError(e.to_string()))?;
    if !abs.exists() { return Err(PeTTaError::FileNotFound(abs)); }
    send_query(stdin, stdout, b'F', &abs.to_string_lossy(), config)
}

pub fn load_metta_files(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut BufReader<std::process::ChildStdout>,
    paths: &[&std::path::Path],
    config: &EngineConfig,
) -> Result<Vec<MettaResult>, PeTTaError> {
    if paths.is_empty() { return Ok(Vec::new()); }
    let combined: String = paths.iter().map(|p| {
        let abs = p.canonicalize().map_err(|e| PeTTaError::PathError(e.to_string()))?;
        if !abs.exists() { return Err(PeTTaError::FileNotFound(abs)); }
        std::fs::read_to_string(&abs).map_err(|e| PeTTaError::PathError(e.to_string()))
    }).collect::<Result<Vec<_>, PeTTaError>>()?.join("\n");
    send_query(stdin, stdout, b'S', &combined, config)
}

pub fn process_metta_string(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut BufReader<std::process::ChildStdout>,
    code: &str,
    config: &EngineConfig,
) -> Result<Vec<MettaResult>, PeTTaError> {
    send_query(stdin, stdout, b'S', code, config)
}
