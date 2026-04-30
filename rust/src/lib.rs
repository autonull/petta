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

// Tests moved to integration tests

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

// Version and utilities
pub use engine::{parse_backend_error, swipl_available, MIN_SWIPL_VERSION};

// Optional exports
#[cfg(feature = "mork")]
pub use mork::MorkEngine;
