//! Unified error handling for PeTTa
//!
//! Streamlined error types with rich context and automatic suggestions.

use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;

/// Main error type for PeTTa
#[derive(Error, Debug)]
pub enum PeTTaError {
 #[error("file not found: {0}")]
 FileNotFound(PathBuf),
 
 #[error("spawn error: {0}")]
 SpawnError(String),
 
 #[error("path error: {0}")]
 PathError(String),
 
 #[error("write error: {0}")]
 WriteError(String),
 
 #[error("backend error: {0}")]
 Backend(#[from] BackendError),
 
 #[error("MORK error: {0}")]
 Mork(String),
 
 #[error("SWI-Prolog version error: {0}")]
 SwiplVersion(String),
 
 #[error("protocol error: {0}")]
 Protocol(String),
 
 #[error("IO error: {0}")]
 Io(#[from] std::io::Error),
 
 #[error("timeout after {0:?}")]
 Timeout(Duration),
 
 #[error("crashed after {restarts} restart(s)", restarts = restarts)]
 Crash { restarts: u32 },
}

/// Backend error kinds with rich context
#[derive(Error, Debug, Clone)]
pub enum BackendError {
 #[error("undefined function `{name}/{arity}`{suggestion}", 
        suggestion = if let Some(s) = suggestion { format!(" (did you mean `{s}`?)") } else { String::new() })]
 Undefined { 
  name: String, 
  arity: usize, 
  suggestion: Option<String> 
 },
 
 #[error("type error: expected {expected}, found {found}")]
 TypeMismatch { 
  expected: String, 
  found: String 
 },
 
 #[error("syntax error: {detail}")]
 Syntax { 
  detail: String 
 },
 
 #[error("unbound variable")]
 UnboundVar,
 
 #[error("uninstantiated argument")]
 Uninstantiated,
 
 #[error("permission denied: {op} on {target}")]
 Permission { 
  op: String, 
  target: String 
 },
 
 #[error("{kind} '{term}' does not exist")]
 Existence { 
  kind: String, 
  term: String 
 },
 
 #[error("stack overflow")]
 StackOverflow,
 
 #[error("evaluation error: {0}")]
 Evaluation(String),
 
 #[error("{0}")]
 Generic(String),
}



// ============================================================================
// Error Analysis & Helpers
// ============================================================================

/// Backward compatibility alias
pub type BackendErrorKind = BackendError;

/// Parse backend error from Prolog/JSON output

/// Parse backend error from Prolog/JSON output
pub fn parse_backend_error(raw: &str) -> BackendError {
    if raw.trim_start().starts_with('{') {
        if let Some(e) = parse_json(raw) {
            return e;
        }
    }
    parse_error_kind(raw).unwrap_or(BackendError::Generic(raw.trim().into()))
}

fn parse_json(raw: &str) -> Option<BackendError> {
    let v = serde_json::from_str::<serde_json::Value>(raw).ok()?;
    let obj = v.as_object()?;
    
    let msg = |f: &str| obj.get(f).and_then(|v| v.as_str()).map(String::from);
    
    if let Some(kind) = obj.get("kind").and_then(|k| k.as_str()) {
        if matches!(kind, "swipl" | "prolog") {
            if let (Some(n), Some(a)) = (msg("name"), msg("name_arity")) {
                if let Ok(arity) = a.parse() {
                    return Some(BackendError::Undefined {
                        name: n, arity, suggestion: msg("suggestion")
                    });
                }
            }
            if let Some(f) = msg("functor") {
                if f.contains("syntax_error") {
                    return Some(BackendError::Syntax { detail: msg("message").unwrap_or_default() });
                }
                if f.contains("existence_error") {
                    return Some(BackendError::Existence {
                        kind: f, term: msg("raw").unwrap_or_default()
                    });
                }
            }
        }
    }
    
    if let Some(formal) = msg("formal") {
        return parse_error_kind(&formal);
    }
    
    parse_error_kind(msg("raw").as_ref()?.as_str())
}

fn parse_error_kind(raw: &str) -> Option<BackendError> {
    if raw.contains("existence_error") && raw.contains("procedure") {
        return Some(extract_undefined(raw));
    }
    if raw.contains("type_error") {
        return Some(extract_type_error(raw));
    }
    if raw.contains("syntax_error") {
        return Some(BackendError::Syntax { detail: raw.into() });
    }
    if raw.contains("instantiation_error") || raw.contains("uninstantiated") {
        return Some(BackendError::Uninstantiated);
    }
    if raw.contains("Stack depth") || raw.contains("stack_limit") {
        return Some(BackendError::StackOverflow);
    }
    if raw.contains("permission_error") {
        return Some(BackendError::Permission {
            op: "unknown".into(), target: "unknown".into()
        });
    }
    if raw.contains("existence_error") {
        return Some(BackendError::Existence {
            kind: "unknown".into(), term: "unknown".into()
        });
    }
    None
}

fn extract_undefined(raw: &str) -> BackendError {
    let (name, arity) = extract_name_arity(raw).unwrap_or_else(|| ("unknown".into(), 0));
    BackendError::Undefined {
        name, arity, suggestion: None
    }
}

fn extract_type_error(raw: &str) -> BackendError {
    let expected = extract_between(raw, "expected ", ",").unwrap_or_else(|| "unknown".into());
    let found = extract_between(raw, "got ", ")").unwrap_or_else(|| "unknown".into());
    BackendError::TypeMismatch { expected, found }
}

fn extract_name_arity(s: &str) -> Option<(String, usize)> {
    s.split(&['(', ')', ',', '/'])
    .map(|t| t.trim())
    .filter(|t| !t.is_empty())
    .filter_map(|t| {
        let parts: Vec<&str> = t.split_whitespace().collect();
        if parts.len() == 2 {
            parts[1].parse::<usize>().ok().map(|a| (parts[0].to_string(), a))
        } else {
            None
        }
    })
    .next()
}

fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    let si = s.find(start)?;
    let rest = &s[si + start.len()..];
    let ei = rest.find(end)?;
    Some(rest[..ei].to_string())
}
