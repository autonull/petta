//! SWI-Prolog backend implementation
//!
//! This module provides the SWI-Prolog backend for PeTTa,
//! implementing the unified Backend trait.

use std::path::Path;

use crate::engine::{EngineConfig, SwiplBackend as CoreSwiplBackend, BackendImpl};
use crate::values::MettaResult;
use crate::Error;
use crate::core::BackendCapabilities;

/// SWI-Prolog subprocess backend wrapper
pub struct SwiplBackend {
    inner: CoreSwiplBackend,
}

impl SwiplBackend {
    pub fn new(config: &EngineConfig) -> Result<Self, Error> {
        Ok(Self {
            inner: CoreSwiplBackend::new(config)?,
        })
    }
}

impl crate::backends::Backend for SwiplBackend {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn version(&self) -> &'static str {
        self.inner.version()
    }

    fn capabilities(&mut self) -> BackendCapabilities {
        self.inner.capabilities()
    }

    fn is_alive(&mut self) -> bool {
        self.inner.is_alive()
    }

    fn load_file(&mut self, path: &Path, config: &EngineConfig) -> Result<Vec<MettaResult>, Error> {
        self.inner.load_metta_file(path, config)
    }

    fn execute(&mut self, code: &str, config: &EngineConfig) -> Result<Vec<MettaResult>, Error> {
        self.inner.process_metta_string(code, config)
    }

    fn restart(&mut self, config: &EngineConfig) -> Result<(), Error> {
        self.inner.restart(config)
    }

    fn shutdown(&mut self) {
        self.inner.shutdown()
    }
}
