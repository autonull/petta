//! Public API for PeTTa - Production MeTTa Runtime
//!
//! This module provides the public-facing API for the PeTTa execution engine,
//! offering ergonomic access to MeTTa execution with minimal boilerplate.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use petta::api::{PeTTa, Backend};
//!
//! // Simple usage
//! let mut engine = PeTTa::new()?;
//! let result = engine.eval("!(+ 1 2)")?;
//! assert_eq!(result, "3");
//! # Ok::<_, petta::Error>(())
//! ```
//!
//! # Architecture
//!
//! The API module provides:
//! - **PeTTa**: Simplified engine interface
//! - **PeTTaEngine**: Full-featured engine with configuration
//! - **EngineConfig**: Engine configuration
//! - **Backend**: Backend type enumeration
//! - **ExecutionResult**: Structured execution results

mod config;
mod engine;
mod result;

pub use config::{Backend, EngineConfig, EngineConfigBuilder};
pub use engine::{Builder as PeTTaBuilder, PeTTa, PeTTaEngine};
pub use engine::{Initialized, PeTTaTyped, Running, Uninitialized};
pub use result::{ExecutionResult, ExecutionStats, MettaResult};

// Re-export for convenience
pub use crate::values::MettaValue;
