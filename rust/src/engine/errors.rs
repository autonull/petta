//! Unified error handling for PeTTa
//!
//! Streamlined error types with rich context and automatic suggestions.

use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

/// Main error type for PeTTa
#[derive(Debug)]
pub enum PeTTaError {
    FileNotFound(PathBuf),
    SpawnError(String),
    PathError(String),
    WriteError(String),
    Backend(BackendError),
    Mork(String),
    SwiplVersion(String),
    Protocol(String),
    Io(std::io::Error),
    Timeout(Duration),
    Crash { restarts: u32 },
}

/// Backend error kinds
#[derive(Debug, Clone)]
pub enum BackendError {
    /// For backward compatibility
    UndefinedFunction { name: String, arity: usize, suggestion: Option<String> },
    Undefined { name: String, arity: usize, suggestion: Option<String> },
    TypeMismatch { expected: String, found: String },
    Syntax { detail: String },
    UnboundVar,
    Uninstantiated,
    Permission { op: String, target: String },
    Existence { kind: String, term: String },
    StackOverflow,
    Evaluation(String),
    Generic(String),
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendError::Undefined { name, arity, suggestion } |
            BackendError::UndefinedFunction { name, arity, suggestion } => {
                write!(f, "undefined function `{name}/{arity}`")?;
                if let Some(s) = suggestion {
                    write!(f, " (did you mean `{s}`?)")?;
                }
                Ok(())
            }
            BackendError::TypeMismatch { expected, found } => {
                write!(f, "type error: expected {expected}, got {found}")
            }
            BackendError::Syntax { detail } => write!(f, "syntax error: {detail}"),
            BackendError::UnboundVar => write!(f, "unbound variable"),
            BackendError::Uninstantiated => write!(f, "uninstantiated argument"),
            BackendError::Permission { op, target } => {
                write!(f, "permission denied: {op} on {target}")
            }
            BackendError::Existence { kind, term } => {
                write!(f, "{kind} {term} does not exist")
            }
            BackendError::StackOverflow => write!(f, "stack overflow"),
            BackendError::Evaluation(msg) => write!(f, "evaluation error: {msg}"),
            BackendError::Generic(msg) => write!(f, "{msg}"),
        }
    }
}

impl fmt::Display for PeTTaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeTTaError::FileNotFound(p) => write!(f, "file not found: {}", p.display()),
            PeTTaError::SpawnError(m) => write!(f, "spawn error: {m}"),
            PeTTaError::PathError(m) => write!(f, "path error: {m}"),
            PeTTaError::WriteError(m) => write!(f, "write error: {m}"),
            PeTTaError::Backend(e) => write!(f, "{e}"),
            PeTTaError::Mork(m) => write!(f, "MORK error: {m}"),
            PeTTaError::SwiplVersion(m) => write!(f, "SWI-Prolog version error: {m}"),
            PeTTaError::Protocol(m) => write!(f, "protocol error: {m}"),
            PeTTaError::Io(e) => write!(f, "IO error: {e}"),
            PeTTaError::Timeout(d) => write!(f, "timeout after {d:?}"),
            PeTTaError::Crash { restarts } => write!(f, "crashed after {restarts} restart(s)"),
        }
    }
}

impl std::error::Error for PeTTaError {}
impl From<std::io::Error> for PeTTaError {
    fn from(e: std::io::Error) -> Self { PeTTaError::Io(e) }
}
impl From<BackendError> for PeTTaError {
    fn from(e: BackendError) -> Self { PeTTaError::Backend(e) }
}

/// Backward compatibility alias
pub type BackendErrorKind = BackendError;

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
