//! Core types and traits for PeTTa
//!
//! This module provides the foundational types, traits, and abstractions
//! used throughout the PeTTa runtime.
//!
//! # Architecture
//!
//! The core module contains:
//! - **Backend trait**: Unified interface for execution backends
//! - **Error types**: Unified error handling
//! - **Value types**: MeTTa value representations
//! - **Type definitions**: Common type aliases and utilities

mod backend;
mod errors;
mod values;
mod types;

pub use backend::{Backend, BackendCapabilities, BackendInfo, BackendStats, HealthStatus};
pub use errors::{Error, BackendError, Result, SourceLocation};
pub use values::{MettaValue, MettaResult};
pub use types::{Type, ProjectRoot, MettaFile};

// Re-export for convenience
pub use crate::engine::Backend as BackendType;
