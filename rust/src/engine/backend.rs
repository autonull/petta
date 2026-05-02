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
//! use petta::engine::{BackendImpl, BackendCapabilities, EngineConfig};
//! use std::path::Path;
//!
//! fn execute_with_backend<B: BackendImpl>(backend: &mut B, config: &EngineConfig) 
//!     -> Result<(), petta::Error> 
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

/// Backend capabilities for feature detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendCapabilities {
 /// Supports parallel execution
 pub supports_parallel: bool,
 /// Supports streaming results
 pub supports_streaming: bool,
 /// Supports incremental updates
 pub supports_incremental: bool,
 /// Supports persistence
 pub supports_persistence: bool,
 /// Supports transactional operations
 pub supports_transactions: bool,
}

impl Default for BackendCapabilities {
 fn default() -> Self {
  Self {
   supports_parallel: false,
   supports_streaming: false,
   supports_incremental: false,
   supports_persistence: false,
   supports_transactions: false,
  }
 }
}

impl BackendCapabilities {
 /// Create new capabilities with all features disabled
 pub const fn new() -> Self {
  Self {
   supports_parallel: false,
   supports_streaming: false,
   supports_incremental: false,
   supports_persistence: false,
   supports_transactions: false,
  }
 }

 /// Enable parallel execution support
 pub const fn with_parallel(mut self, supported: bool) -> Self {
  self.supports_parallel = supported;
  self
 }

 /// Enable streaming support
 pub const fn with_streaming(mut self, supported: bool) -> Self {
  self.supports_streaming = supported;
  self
 }

 /// Enable incremental updates support
 pub const fn with_incremental(mut self, supported: bool) -> Self {
  self.supports_incremental = supported;
  self
 }

 /// Enable persistence support
 pub const fn with_persistence(mut self, supported: bool) -> Self {
  self.supports_persistence = supported;
  self
 }

 /// Enable transactional support
 pub const fn with_transactions(mut self, supported: bool) -> Self {
  self.supports_transactions = supported;
  self
 }
}

/// Core backend trait for MeTTa execution.
///
/// This trait defines the minimal interface that all backends must implement.
/// It provides backend-agnostic execution of MeTTa code with capability-based
/// feature detection.
///
/// # Implementing a Backend
///
/// ```rust,no_run
/// use petta::engine::{BackendImpl, BackendCapabilities, EngineConfig};
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
    fn health_status(&self) -> HealthStatus {
        HealthStatus::Healthy
    }

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

/// Backend metadata
#[derive(Debug, Clone)]
pub struct BackendInfo {
 /// Backend name
 pub name: &'static str,
 /// Backend version
 pub version: &'static str,
 /// Backend description
 pub description: &'static str,
 /// Supported MeTTa language version
 pub metta_version: &'static str,
}

impl BackendInfo {
 /// Create new backend info
 pub const fn new(
  name: &'static str,
  version: &'static str,
  description: &'static str,
 ) -> Self {
  Self {
   name,
   version,
   description,
   metta_version: "0.5",
  }
 }

 /// Set MeTTa version support
 pub const fn with_metta_version(mut self, version: &'static str) -> Self {
  self.metta_version = version;
  self
 }
}

/// Backend execution statistics
#[derive(Debug, Clone, Default)]
pub struct BackendStats {
 /// Total queries executed
 pub queries_executed: u64,
 /// Total execution time in nanoseconds
 pub total_execution_time_ns: u64,
 /// Total errors encountered
 pub errors_encountered: u64,
 /// Total restarts performed
 pub restarts: u64,
 /// Successful queries
 pub successful_queries: u64,
 /// Failed queries
 pub failed_queries: u64,
}

impl BackendStats {
 /// Create new stats
 pub fn new() -> Self {
  Self::default()
 }

 /// Record a query execution
 pub fn record_query(&mut self, execution_time_ns: u64, success: bool) {
  self.queries_executed += 1;
  self.total_execution_time_ns += execution_time_ns;
  if success {
   self.successful_queries += 1;
  } else {
   self.failed_queries += 1;
  }
 }

 /// Record an error
 pub fn record_error(&mut self) {
  self.errors_encountered += 1;
  self.failed_queries += 1;
 }

 /// Record a restart
 pub fn record_restart(&mut self) {
  self.restarts += 1;
 }

 /// Get average query execution time
 pub fn average_query_time_ns(&self) -> Option<u64> {
  if self.queries_executed == 0 {
   None
  } else {
   Some(self.total_execution_time_ns / self.queries_executed)
  }
 }

 /// Get success rate
 pub fn success_rate(&self) -> f64 {
  if self.queries_executed == 0 {
   1.0
  } else {
   self.successful_queries as f64 / self.queries_executed as f64
  }
 }

 /// Get error rate
 pub fn error_rate(&self) -> f64 {
  if self.queries_executed == 0 {
   0.0
  } else {
   self.errors_encountered as f64 / self.queries_executed as f64
  }
 }
}

/// Backend health status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }
    
    pub fn message(&self) -> Option<&str> {
        match self {
            HealthStatus::Healthy => None,
            HealthStatus::Degraded(msg) | HealthStatus::Unhealthy(msg) => Some(msg),
        }
    }
}
