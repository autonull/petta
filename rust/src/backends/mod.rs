//! Backend implementations for PeTTa
//!
//! This module provides concrete backend implementations:
//! - **SwiplBackend**: SWI-Prolog backend via binary protocol
//! - **MorkBackend**: MORK native Rust backend (requires `mork` feature)
//!
//! # Backend Selection
//!
//! Backends can be selected via configuration:
//!
//! ```rust,no_run
//! use petta::backends::{BackendRegistry, BackendType};
//!
//! // Auto-select best available backend
//! let backend = BackendRegistry::auto_select()?;
//!
//! // Or specify explicitly
//! let backend = BackendRegistry::create(BackendType::Swipl)?;
//! # Ok::<_, petta::Error>(())
//! ```

mod swipl;
#[cfg(feature = "mork")]
mod mork;

pub use swipl::SwiplBackend;
#[cfg(feature = "mork")]
pub use mork::MorkBackend;

use std::collections::HashMap;
use crate::engine::EngineConfig;
use crate::Error;

/// Backend type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    Swipl,
    #[cfg(feature = "mork")]
    Mork,
}

/// Unified backend trait
pub trait Backend: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str { "unknown" }
    fn is_alive(&self) -> bool;
    fn capabilities(&self) -> crate::core::BackendCapabilities {
        crate::core::BackendCapabilities::default()
    }
    fn load_file(&mut self, path: &std::path::Path, config: &EngineConfig) -> Result<Vec<crate::values::MettaResult>, Error>;
    fn execute(&mut self, code: &str, config: &EngineConfig) -> Result<Vec<crate::values::MettaResult>, Error>;
    fn restart(&mut self, config: &EngineConfig) -> Result<(), Error>;
    fn shutdown(&mut self);
}

/// Backend registry for managing multiple backends
pub struct BackendRegistry {
    #[allow(dead_code)]
    backends: HashMap<String, Box<dyn Backend>>,
}

impl BackendRegistry {
    /// Create new empty registry
    pub fn new() -> Self {
        Self {
            backends: HashMap::new(),
        }
    }

    /// Auto-select best available backend
    pub fn auto_select() -> Result<Box<dyn Backend>, Error> {
        #[cfg(feature = "mork")]
        {
            // Prefer MORK if available
            return Ok(Box::new(MorkBackend::new()));
        }

        #[cfg(not(feature = "mork"))]
        {
            // Fall back to SWI-Prolog
            return Ok(Box::new(SwiplBackend::new(&EngineConfig::default())?));
        }
    }

    /// Create specific backend
    pub fn create(backend_type: BackendType) -> Result<Box<dyn Backend>, Error> {
        match backend_type {
            BackendType::Swipl => Ok(Box::new(SwiplBackend::new(&EngineConfig::default())?)),
            #[cfg(feature = "mork")]
            BackendType::Mork => Ok(Box::new(MorkBackend::new())),
        }
    }
}

impl Default for BackendRegistry {
    fn default() -> Self {
        Self::new()
    }
}