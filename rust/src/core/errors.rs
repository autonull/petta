//! Unified error handling for PeTTa with rich context and source locations.
//!
//! This module provides a comprehensive error handling system with:
//! - Actionable error messages
//! - Source location tracking for parse errors
//! - Automatic error suggestions
//! - Type-safe error conversions

use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;

/// Main error type for PeTTa execution engine
#[derive(Error, Debug)]
pub enum Error {
    /// File not found
    #[error("file not found: {0}")]
    FileNotFound(PathBuf),

    /// Permission denied
    #[error("permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// Failed to spawn backend
    #[error("failed to spawn backend: {0}")]
    SpawnError(String),

    /// Path error
    #[error("path error: {0}")]
    PathError(String),

    /// Write error
    #[error("write error: {0}")]
    WriteError(String),

    /// Backend error
    #[error("backend error: {0}")]
    Backend(#[from] BackendError),

    /// MORK backend error
    #[error("MORK backend error: {0}")]
    Mork(String),

    /// SWI-Prolog version error
    #[error("SWI-Prolog version error: {0}")]
    SwiplVersion(String),

    /// Protocol error
    #[error("protocol error: {0}")]
    Protocol(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Operation timed out
    #[error("operation timed out after {0:?}")]
    Timeout(Duration),

    /// Backend crashed
    #[error("backend crashed after {restarts} restart attempt(s)")]
    Crash { restarts: u32 },

    /// Configuration error
    #[error("configuration error: {0}")]
    Config(String),

    /// Parse error with source location
    #[error("parse error at {location}: {message}")]
    Parse {
        message: String,
        location: SourceLocation,
    },

    /// Type error with context
    #[error("type error: expected {expected}, found {found} in {context}")]
    Type {
        expected: String,
        found: String,
        context: String,
    },

    /// Execution failed
    #[error("execution failed: {0}")]
    Execution(String),
}

/// Backend-specific error types
#[derive(Error, Debug, Clone)]
pub enum BackendError {
    /// Undefined function or predicate
    #[error("undefined function `{name}/{arity}`{suggestion}",
        suggestion = if let Some(s) = suggestion {
            format!(" (did you mean `{s}`?)")
        } else { String::new() })]
    Undefined {
        name: String,
        arity: usize,
        suggestion: Option<String>,
    },

    /// Type mismatch
    #[error("type error: expected {expected}, found {found}")]
    TypeMismatch {
        expected: String,
        found: String,
    },

    /// Syntax error
    #[error("syntax error: {detail}")]
    Syntax {
        detail: String,
    },

    /// Unbound variable in query
    #[error("unbound variable in query")]
    UnboundVar,

    /// Uninstantiated argument
    #[error("uninstantiated argument")]
    Uninstantiated,

    /// Permission denied
    #[error("permission denied: {op} on {target}")]
    Permission {
        op: String,
        target: String,
    },

    /// Existence error
    #[error("{kind} '{term}' does not exist")]
    Existence {
        kind: String,
        term: String,
    },

    /// Stack overflow
    #[error("stack overflow: maximum stack depth exceeded")]
    StackOverflow,

    /// Evaluation error
    #[error("evaluation error: {0}")]
    Evaluation(String),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

/// Source location for error reporting
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
}

impl SourceLocation {
    /// Create new source location
    pub fn new(file: PathBuf, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
