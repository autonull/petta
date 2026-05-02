//! MORK backend implementation - re-export from engine
//!
//! This module provides the MORK native Rust backend for PeTTa.
//! Requires the `mork` feature to be enabled.

// Re-export the implementation from engine
#[cfg(feature = "mork")]
pub use crate::engine::backends::MorkBackend;
