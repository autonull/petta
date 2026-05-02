//! SWI-Prolog backend implementation
//!
//! This module provides the SWI-Prolog backend for PeTTa,
//! implementing the unified Backend trait.

use std::path::Path;

use crate::engine::EngineConfig;
use crate::values::MettaResult;
use crate::Error;
use super::Backend as BackendTrait;
use crate::core::BackendCapabilities;

/// SWI-Prolog subprocess backend
pub struct SwiplBackend {
    _config: EngineConfig,
}

impl SwiplBackend {
    pub fn new(config: &EngineConfig) -> Result<Self, Error> {
        Ok(Self {
            _config: config.clone(),
        })
    }
}

impl BackendTrait for SwiplBackend {
    fn name(&self) -> &'static str {
        "SWI-Prolog"
    }

    fn version(&self) -> &'static str {
        "9.3"
    }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::new()
            .with_streaming(true)
            .with_transactions(false)
    }

    fn is_alive(&self) -> bool {
        true
    }

    fn load_file(&mut self, _path: &Path, _config: &EngineConfig) -> Result<Vec<MettaResult>, Error> {
        // Delegate to engine's backend
        Err(Error::Protocol("SWI backend implementation delegated to engine".into()))
    }

    fn execute(&mut self, _code: &str, _config: &EngineConfig) -> Result<Vec<MettaResult>, Error> {
        // Delegate to engine's backend
        Err(Error::Protocol("SWI backend implementation delegated to engine".into()))
    }

    fn restart(&mut self, _config: &EngineConfig) -> Result<(), Error> {
        Ok(())
    }

    fn shutdown(&mut self) {
        // No-op for this implementation
    }
}

impl Drop for SwiplBackend {
    fn drop(&mut self) {
        self.shutdown();
    }
}
