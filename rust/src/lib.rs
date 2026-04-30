//! PeTTa - Production MeTTa Runtime
//!
//! A production-grade MeTTa language implementation with dual backends:
//! - **Prolog WAM**: Mature, stable backend using SWI-Prolog
//! - **MORK**: High-performance native Rust backend

#![cfg_attr(feature = "mork", feature(core_intrinsics))]
#![cfg_attr(feature = "mork", feature(portable_simd))]
#![cfg_attr(feature = "mork", feature(allocator_api))]

pub mod engine;
pub mod parser;
pub mod utils;
pub mod differential;

#[cfg(feature = "mork")]
pub mod mork;

// Re-export main types
pub use engine::{
    Backend, BackendError, BackendErrorKind, EngineConfig, MettaResult, MettaValue, PeTTaError, PeTTaEngine,
    MIN_SWIPL_VERSION, swipl_available, parse_backend_error,
};
