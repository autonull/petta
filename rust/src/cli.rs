//! CLI argument parsing with clap
//!
//! Ergonomic command-line interface configuration

use clap::{Parser, ValueEnum};
use petta::Backend;

/// PeTTa CLI arguments
#[derive(Parser, Debug, Clone)]
#[command(
    name = "petta",
    author = "Patrick Hammer",
    version = "0.5.0",
    about = "🦀 PeTTa v0.5.0 - Production MeTTa Runtime",
    long_about = "PeTTa is a production-grade MeTTa runtime with dual backends (Prolog and MORK)"
)]
pub struct Cli {
    /// MeTTa files to execute
    #[arg(required = false, value_name = "FILES")]
    pub files: Vec<String>,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Enable timing information
    #[arg(short, long, default_value_t = false)]
    pub time: bool,

    /// Backend to use (auto, mork, prolog)
    #[arg(short, long, default_value = "prolog", value_name = "BACKEND")]
    pub backend: BackendArg,

    /// Interactive REPL mode
    #[arg(short = 'i', long, default_value_t = false)]
    pub interactive: bool,

    /// Output format (pretty, compact, json, sexpr)
    #[arg(short = 'O', long, default_value = "pretty", value_name = "FORMAT")]
    pub output_format: OutputFormat,

    /// Enable profiling
    #[arg(long, default_value_t = false)]
    pub profile: bool,

    /// Enable trace output
    #[arg(long, default_value_t = false)]
    pub trace: bool,

    /// Show statistics
    #[arg(long, default_value_t = false)]
    pub stats: bool,
}

/// Backend selection
#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
#[value(rename_all = "lowercase")]
pub enum BackendArg {
    Mork,
    #[default]
    Prolog,
}

impl std::fmt::Display for BackendArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendArg::Mork => write!(f, "Mork"),
            BackendArg::Prolog => write!(f, "Swipl"),
        }
    }
}

impl BackendArg {
    pub fn to_backend(&self) -> Backend {
        match self {
            BackendArg::Mork => Backend::Mork,
            BackendArg::Prolog => Backend::Swipl,
        }
    }
}

/// Output format selection
#[derive(ValueEnum, Clone, Debug, Default, PartialEq)]
#[value(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Pretty,
    Compact,
    Json,
    SExpr,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Pretty => write!(f, "pretty"),
            OutputFormat::Compact => write!(f, "compact"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::SExpr => write!(f, "sexpr"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_arg_display() {
        assert_eq!(BackendArg::Mork.to_string(), "Mork");
        assert_eq!(BackendArg::Prolog.to_string(), "Swipl");
    }

    #[test]
    fn test_output_format_display() {
        assert_eq!(OutputFormat::Pretty.to_string(), "pretty");
        assert_eq!(OutputFormat::Json.to_string(), "json");
    }

    #[test]
    fn test_backend_conversion() {
        assert_eq!(BackendArg::Mork.to_backend(), Backend::Mork);
        assert_eq!(BackendArg::Prolog.to_backend(), Backend::Swipl);
    }
}
