use std::path::PathBuf;
use std::time::Duration;

/// SWI-Prolog-specific error kinds parsed from raw error messages.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SwiplErrorKind {
    #[error("MeTTa function '{name}/{arity}' is not defined{}", .suggestion.as_ref().map(|s| format!(". Did you mean '{}'?", s)).unwrap_or_default())]
    UndefinedFunction {
        name: String,
        arity: usize,
        suggestion: Option<String>,
    },

    #[error("Type error: expected {expected}, got {found}{}", .context.as_ref().map(|c| format!(" ({})", c)).unwrap_or_default())]
    TypeMismatch {
        expected: String,
        found: String,
        context: Option<String>,
    },

    #[error("Syntax error: {detail}")]
    SyntaxError {
        line: Option<u32>,
        column: Option<u32>,
        detail: String,
    },

    #[error("Argument is not sufficiently instantiated{}", .location.as_ref().map(|l| format!(" at {}", l)).unwrap_or_default())]
    UninstantiatedArgument { location: Option<String> },

    #[error("Permission denied: {operation} on {target}")]
    PermissionDenied { operation: String, target: String },

    #[error("{error_type} {term} does not exist")]
    ExistenceError { error_type: String, term: String },

    #[error("Stack overflow{}", .limit.map(|l| format!(" (limit: {})", l)).unwrap_or_default())]
    StackOverflow { limit: Option<u32> },

    #[error("{0}")]
    Generic(String),
}

/// Top-level error type for PeTTa operations.
#[derive(Debug, thiserror::Error)]
pub enum PeTTaError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Failed to spawn swipl: {0}")]
    SpawnSwipl(String),

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Write error: {0}")]
    WriteError(String),

    #[error(transparent)]
    SwiplError(#[from] SwiplErrorKind),

    #[error("SWI-Prolog version: {0}")]
    SwiplVersionError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Query timed out after {0:?}")]
    Timeout(Duration),

    #[error("Subprocess crashed after {restarts} restart(s)")]
    SubprocessCrashed { restarts: u32 },
}

pub(crate) fn parse_swipl_error(raw: &str) -> SwiplErrorKind {
    if raw.contains("existence_error")
        && raw.contains("procedure")
        && let Some((name, arity)) = extract_name_arity(raw)
    {
        return SwiplErrorKind::UndefinedFunction {
            name,
            arity,
            suggestion: None,
        };
    }
    if raw.contains("type_error") {
        return SwiplErrorKind::TypeMismatch {
            expected: "unknown".into(),
            found: "unknown".into(),
            context: None,
        };
    }
    if raw.contains("syntax_error") {
        return SwiplErrorKind::SyntaxError {
            line: None,
            column: None,
            detail: extract_between(raw, "syntax_error(", ")").unwrap_or_else(|| "unknown".into()),
        };
    }
    if raw.contains("instantiation_error") {
        return SwiplErrorKind::UninstantiatedArgument {
            location: extract_between(raw, "in ", " at line"),
        };
    }
    if raw.contains("Stack depth") || raw.contains("stack_limit") {
        return SwiplErrorKind::StackOverflow { limit: None };
    }
    SwiplErrorKind::Generic(raw.lines().next().unwrap_or(raw).trim().to_string())
}

fn extract_name_arity(raw: &str) -> Option<(String, usize)> {
    for t in raw.split(&['(', ')', ',', '/']) {
        let t = t.trim();
        let p: Vec<&str> = t.split_whitespace().collect();
        if p.len() == 2
            && let Ok(a) = p[1].parse::<usize>()
        {
            return Some((p[0].to_string(), a));
        }
    }
    None
}

fn extract_between(s: &str, start: &str, end: &str) -> Option<String> {
    let si = s.find(start)?;
    let rest = &s[si + start.len()..];
    let ei = rest.find(end)?;
    Some(rest[..ei].to_string())
}
