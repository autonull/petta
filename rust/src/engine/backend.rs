//! Unified backend trait for PeTTa execution engines.
//!
//! This module defines the core abstraction that allows PeTTa to support
//! multiple execution backends (SWI-Prolog, MORK) through a common interface.
//!
//! # Architecture
//!
//! The backend trait provides:
//! - Unified execution interface
//! - Common error handling
//! - Consistent result types
//! - Backend-agnostic configuration
//! - Capability-based feature detection
//!
//! # Example
//!
//! ```rust,no_run
//! use petta::engine::{BackendImpl, EngineConfig};
//! use petta::core::BackendCapabilities;
//! use std::path::Path;
//!
//! fn execute_with_backend<B: BackendImpl>(backend: &mut B, config: &EngineConfig)
//! -> Result<(), petta::Error>
//! {
//!     // Check capabilities
//!     let caps = backend.capabilities();
//!     if caps.supports_streaming {
//!         println!("Streaming results available");
//!     }
//!
//!     // Backend-agnostic execution
//!     let results = backend.process_metta_string("!(+ 1 2)", config)?;
//!     Ok(())
//! }
//! ```

use std::path::Path;
use super::config::EngineConfig;
use super::errors::Error;
use crate::values::MettaResult;
use crate::core::BackendCapabilities;


/// Core backend trait for MeTTa execution.
///
/// This trait defines the minimal interface that all backends must implement.
/// It provides backend-agnostic execution of MeTTa code with capability-based
/// feature detection.
///
/// # Implementing a Backend
///
/// ```rust,no_run
/// use petta::engine::{BackendImpl, EngineConfig};
/// use petta::core::BackendCapabilities;
/// use petta::{MettaResult, Error};
/// use std::path::Path;
///
/// struct MyBackend {
///     // backend state
/// }
///
/// impl BackendImpl for MyBackend {
///     fn name(&self) -> &'static str { "MyBackend" }
///     
///     fn process_metta_string(&mut self, code: &str, config: &EngineConfig) 
///         -> Result<Vec<MettaResult>, Error> 
///     {
///         // Your implementation here
///         Ok(vec![])
///     }
///     
///     fn is_alive(&mut self) -> bool { true }
///     fn shutdown(&mut self) {}
///     fn restart(&mut self, config: &EngineConfig) -> Result<(), Error> { Ok(()) }
/// }
/// ```
pub trait BackendImpl: Send + Sync {
    /// Get backend name for identification
    fn name(&self) -> &'static str;

    /// Get backend version (defaults to "unknown")
    fn version(&self) -> &'static str {
        "unknown"
    }

    /// Get backend capabilities (defaults to basic capabilities)
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::default()
    }

    /// Check if backend is healthy and responsive
    fn is_alive(&mut self) -> bool;

    /// Get current health status (defaults to Healthy)

    /// Load and execute a single MeTTa file.
    /// 
    /// Default implementation reads the file and calls `process_metta_string`.
    fn load_metta_file(
        &mut self,
        path: &Path,
        config: &EngineConfig
    ) -> Result<Vec<MettaResult>, Error> {
        let content = std::fs::read_to_string(path)
            .map_err(|_e| Error::FileNotFound(path.to_path_buf()))?;
        self.process_metta_string(&content, config)
    }

    /// Load and execute multiple MeTTa files.
    /// 
    /// Default implementation calls `load_metta_file` for each path.
    fn load_metta_files(
        &mut self, 
        paths: &[&Path], 
        config: &EngineConfig
    ) -> Result<Vec<MettaResult>, Error> {
        let mut all_results = Vec::new();
        for path in paths {
            all_results.extend(self.load_metta_file(path, config)?);
        }
        Ok(all_results)
    }

    /// Process a MeTTa code string
    fn process_metta_string(
        &mut self, 
        code: &str, 
        config: &EngineConfig
    ) -> Result<Vec<MettaResult>, Error>;

    /// Get stderr output (if available)
    fn stderr_output(&self) -> String {
        String::new()
    }

    /// Shutdown backend gracefully
    fn shutdown(&mut self);

    /// Restart backend for crash recovery
    fn restart(&mut self, config: &EngineConfig) -> Result<(), Error>;
}
