//! PeTTa - Production MeTTa Runtime
//!
//! A production-grade MeTTa language implementation with dual backends:
//! - **Prolog WAM**: Mature, stable backend using SWI-Prolog
//! - **MORK**: High-performance native Rust backend
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use petta::{PeTTaEngine, EngineConfig};
//! use std::path::Path;
//!
//! // Create engine with default configuration
//! let config = EngineConfig::new(Path::new("."));
//! let mut engine = PeTTaEngine::with_config(&config).unwrap();
//!
//! // Evaluate MeTTa expressions
//! let result = engine.eval("!(+ 1 2)").unwrap();
//! assert_eq!(result, "3");
//! ```
//!
//! # Features
//!
//! - `mork`: Enable high-performance MORK backend (requires nightly Rust)
//! - `repl`: Interactive REPL mode
//! - `clap`: Command-line argument parsing
//! - `fast-hasher`: Use GXHash for faster hashing (requires AES/SSE2)
//! - `parallel`: Parallel execution support
//! - `async`: Async/await support with Tokio
//!
//! # Architecture
//!
//! PeTTa provides a unified interface to multiple backends through the [`PeTTaEngine`] struct.
//! The engine handles:
//! - Backend lifecycle management
//! - Automatic crash recovery
//! - Error handling with suggestions
//! - Output formatting
//!
//! # Example: Loading Files
//!
//! ```rust,no_run
//! use petta::PeTTaEngine;
//!
//! let mut engine = PeTTaEngine::new(std::path::Path::new("."), false).unwrap();
//!
//! // Load and execute MeTTa files
//! engine.load_files(&["lib.metta", "main.metta"]).unwrap();
//!
//! // Query the engine
//! let result = engine.eval("!(your-query)").unwrap();
//! println!("Result: {}", result);
//! ```
//!
//! # Example: Using MORK Backend
//!
//! ```rust,no_run
//! # #[cfg(feature = "mork")]
//! # {
//! use petta::{PeTTaEngine, EngineConfig, Backend};
//!
//! let config = EngineConfig::builder()
//!     .backend(Backend::Mork)
//!     .build();
//!
//! let mut engine = PeTTaEngine::with_config(&config).unwrap();
//! # }
//! ```

#![cfg_attr(feature = "mork", feature(core_intrinsics))]
#![cfg_attr(feature = "mork", feature(portable_simd))]
#![cfg_attr(feature = "mork", feature(allocator_api))]

pub mod engine;
pub mod parser;
pub mod utils;

#[cfg(feature = "mork")]
pub mod mork;

mod cli;
mod repl;
mod profiler;
mod observability;
mod reliability;
pub mod differential;
mod viz;
mod gxhash;

// ============================================================================
// Core Engine Exports
// ============================================================================

pub use engine::{
    Backend, BackendError, BackendErrorKind, EngineConfig, EngineConfigBuilder,
    MettaResult, MettaValue, PeTTaError, PeTTaEngine,
};

// ============================================================================
// Output Formatting
// ============================================================================

pub use engine::{
    create_formatter, CompactFormatter, JsonFormatter, OutputFormatter,
    PrettyFormatter, SExprFormatter,
};

// ============================================================================
// CLI and REPL
// ============================================================================

pub use cli::{BackendArg, Cli, OutputFormat};
pub use repl::{run_repl, ReplConfig};

// ============================================================================
// Profiling and Monitoring
// ============================================================================

pub use profiler::{ProfileStats, QueryProfile};
pub use observability::{HealthStatus, Metrics, ObservabilityConfig, ServiceStatus};
pub use reliability::{CircuitBreaker, CircuitState, ReliabilityConfig};

// ============================================================================
// Differential Testing
// ============================================================================

pub use differential::{
    assert_backend_parity, compare_results, BackendResult, DifferentialTest,
    ParityTestSuite, TestSuiteResult,
};

// ============================================================================
// Visualization
// ============================================================================

pub use viz::{format_stats_table, visualize_expression};

// ============================================================================
// Optional Exports
// ============================================================================

/// GXHash hasher (available with `fast-hasher` feature)
#[cfg(feature = "fast-hasher")]
pub use gxhash::GxHasher;

// ============================================================================
// Re-exports for convenience
// ============================================================================

/// Parser utilities for MeTTa S-expressions
pub mod parse {
    pub use crate::parser::{parse_metta, serialize_metta};
}

/// Utility functions
pub mod util {
    pub use crate::utils::{format_duration_ms, find_best_match, levenshtein, truncate};
}
