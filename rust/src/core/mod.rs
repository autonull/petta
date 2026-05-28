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
mod types;
mod values;

pub use backend::{Backend, BackendCapabilities};
pub use errors::{BackendError, Error, Result, SourceLocation};
pub use types::{MettaFile, ProjectRoot, Type};
pub use values::{MettaResult, MettaValue};

// Re-export for convenience
pub use crate::engine::Backend as BackendType;
