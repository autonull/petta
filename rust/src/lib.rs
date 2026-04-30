//! PeTTa - Production MeTTa Runtime
//!
//! A production-grade MeTTa language implementation with dual backends:
//! - **Prolog WAM**: Mature, stable backend using SWI-Prolog
//! - **MORK**: High-performance native Rust backend
//!
//! # Example
//!
//! ```rust,no_run
//! use petta::{PeTTaEngine, EngineConfig};
//! use std::path::Path;
//!
//! let config = EngineConfig::new(Path::new("."));
//! let mut engine = PeTTaEngine::with_config(&config).unwrap();
//! let results = engine.process_metta_string("!(+ 1 2)").unwrap();
//! assert_eq!(results[0].value, "3");
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

// Core exports
pub use engine::{
    Backend, BackendError, BackendErrorKind, EngineConfig, EngineConfigBuilder, MettaResult,
    MettaValue, PeTTaError, PeTTaEngine,
};

// Formatters
pub use engine::{
    create_formatter, CompactFormatter, JsonFormatter, OutputFormatter, PrettyFormatter,
    SExprFormatter,
};

// CLI exports
pub use cli::{BackendArg, Cli, OutputFormat};

// REPL exports
pub use repl::{run_repl, ReplConfig};

// Profiler exports
pub use profiler::{ProfileStats, QueryProfile};

// Observability exports
pub use observability::{HealthStatus, Metrics, ObservabilityConfig, ServiceStatus};

// Reliability exports
pub use reliability::{CircuitBreaker, CircuitState, ReliabilityConfig};

// Differential testing exports
pub use differential::{
    assert_backend_parity, compare_results, BackendResult, DifferentialTest, ParityTestSuite,
    TestSuiteResult,
};

// Visualization exports
pub use viz::{format_stats_table, visualize_expression};

// GXHash exports (optional)
#[cfg(feature = "fast-hasher")]
pub use gxhash::GxHasher;
