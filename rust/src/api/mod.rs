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

mod engine;
mod config;
mod result;

pub use engine::{PeTTa, PeTTaEngine, Builder as PeTTaBuilder};
pub use engine::{PeTTaTyped, Uninitialized, Initialized, Running};
pub use config::{EngineConfig, EngineConfigBuilder, Backend};
pub use result::{ExecutionResult, MettaResult, ExecutionStats};

// Re-export for convenience
pub use crate::values::MettaValue;
