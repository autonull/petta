//! MORK backend implementation
//!
//! This module provides the MORK native Rust backend for PeTTa.
//! Requires the `mork` feature to be enabled.

#[cfg(feature = "mork")]
use {
    crate::engine::EngineConfig,
    crate::values::{MettaResult, MettaValue},
    crate::Error,
    super::Backend as BackendTrait,
    crate::core::BackendCapabilities,
};

#[cfg(feature = "mork")]
use crate::mork::interpreter::Interpreter;

#[cfg(feature = "mork")]
use std::path::Path;

#[cfg(feature = "mork")]
use crate::mork::space::Space;

/// MORK native Rust backend
#[cfg(feature = "mork")]
pub struct MorkBackend {
    interpreter: Interpreter,
    stats: crate::core::BackendStats,
}

#[cfg(feature = "mork")]
impl MorkBackend {
    pub fn new() -> Self {
        use std::sync::{Arc, Mutex};

        let space = Arc::new(Mutex::new(Space::new()));
        let interpreter = Interpreter::new(space);

        Self {
            interpreter,
            stats: crate::core::BackendStats::new(),
        }
    }
}

#[cfg(feature = "mork")]
impl BackendTrait for MorkBackend {
    fn name(&self) -> &'static str {
        "MORK"
    }

    fn version(&self) -> &'static str {
        "1.0"
    }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::new()
            .with_streaming(true)
            .with_transactions(false)
            .with_parallel(true)
    }

    fn is_alive(&self) -> bool {
        true // MORK is always alive (in-process)
    }

    fn load_file(&mut self, path: &Path, _config: &EngineConfig) -> Result<Vec<MettaResult>, Error> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::PathError(e.to_string()))?;
        Ok(self.execute(&content, _config)?)
    }

    fn execute(&mut self, code: &str, _config: &EngineConfig) -> Result<Vec<MettaResult>, Error> {
        let results = self.interpreter
            .process(code)
            .into_iter()
            .map(|v| MettaResult { value: v })
            .collect();
        Ok(results)
    }

    fn restart(&mut self, _config: &EngineConfig) -> Result<(), Error> {
        self.stats.record_restart();
        Ok(())
    }

    fn shutdown(&mut self) {
        // No-op for in-process backend
    }
}

#[cfg(feature = "mork")]
impl Default for MorkBackend {
    fn default() -> Self {
        Self::new()
    }
}
