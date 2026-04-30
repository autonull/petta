//! PeTTa - Production MeTTa Runtime
//!
//! A production-grade MeTTa language implementation with dual backends:
//! - **Prolog WAM**: Mature, stable backend using SWI-Prolog
//! - **MORK**: High-performance native Rust backend
//!
//! # Features
//!
//! - Dual execution backends (Prolog and MORK)
//! - Memory-safe execution (no GC, no segfaults)
//! - Comprehensive error handling with helpful diagnostics
//! - Ergonomic API for embedding MeTTa in Rust applications
//! - Interactive REPL with history and syntax highlighting
//! - Production-ready with health checks and monitoring
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
//! assert_eq!(result.value, "3");
//!
//! // Load and execute MeTTa files
//! let results = engine.load_metta_file(Path::new("examples/fib.metta")).unwrap();
//! ```
//!
//! # Backend Selection
//!
//! ```rust,no_run
//! use petta::{PeTTaEngine, Backend, EngineConfig};
//! use std::path::Path;
//!
//! // Auto-detect best backend
//! let config = EngineConfig::new(Path::new(".")).backend(Backend::auto_detect());
//!
//! // Or explicitly choose
//! let config = EngineConfig::new(Path::new("."))
//!     .backend(Backend::Mork)  // or Backend::Swipl
//!     .verbose(true);
//!
//! let mut engine = PeTTaEngine::with_config(&config).unwrap();
//! ```

#![cfg_attr(all(test, feature = "mork"), allow(implicit_autoref))]
#![cfg_attr(feature = "mork", allow(internal_features))]
#![cfg_attr(feature = "mork", feature(core_intrinsics))]
#![cfg_attr(feature = "mork", feature(portable_simd))]
#![cfg_attr(feature = "mork", feature(allocator_api))]
#![cfg_attr(feature = "mork", feature(coroutine_trait))]
#![cfg_attr(feature = "mork", feature(coroutines))]
#![cfg_attr(feature = "mork", feature(stmt_expr_attributes))]
#![cfg_attr(feature = "mork", feature(gen_blocks))]
#![cfg_attr(feature = "mork", feature(yield_expr))]

pub mod parser;

#[cfg(feature = "profiling")]
pub mod profiler;

#[cfg(feature = "mork")]
pub mod mork;
#[cfg(not(feature = "mork"))]
mod mork;

pub mod gxhash;
pub mod pathmap;
pub mod utils;
pub mod viz;

pub mod engine;
pub use engine::{
    Backend, BackendCapabilities, BackendConfig, BackendErrorKind, DiagLocation, DiagSeverity,
    Diagnostic, EngineConfig, EngineConfigBuilder, MettaResult, MettaValue, PeTTaEngine, PeTTaError,
    create_formatter, CompactFormatter, JsonFormatter, OutputFormatter, PrettyFormatter,
    SExprFormatter, MIN_SWIPL_VERSION, swipl_available,
};

#[cfg(feature = "bench")]
pub mod benchmark;
pub mod differential;

pub mod observability;
pub mod reliability;

pub use observability::{HealthStatus, Metrics, ObservabilityConfig, ServiceStatus};
pub use reliability::{CircuitBreaker, CircuitState, ReliabilityConfig};

#[cfg(test)]
mod lib_tests;
