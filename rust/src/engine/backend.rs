//! Unified backend trait for PeTTa execution engines
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
//!
//! # Example
//!
//! ```rust,no_run
//! use petta::engine::{BackendImpl, EngineConfig};
//! use petta::engine::Backend;
//!
//! fn execute_with_backend<B: BackendImpl>(backend: &mut B, config: &EngineConfig) {
//! // Backend-agnostic execution
//! let results = backend.process_metta_string("!(+ 1 2)", config);
//! }
//! ```

use std::path::Path;
use super::config::EngineConfig;
use super::errors::PeTTaError;
use super::values::MettaResult;

/// Core backend trait for MeTTa execution
///
/// This trait defines the minimal interface that all backends must implement.
/// It provides backend-agnostic execution of MeTTa code.
pub trait BackendImpl: Send + Sync {
    /// Get backend name for identification
    fn name(&self) -> &'static str;
    
    /// Check if backend is healthy and responsive
    fn is_alive(&mut self) -> bool;
    
    /// Load and execute a single MeTTa file
    fn load_metta_file(&mut self, path: &Path, config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError>;
    
    /// Load and execute multiple MeTTa files
    fn load_metta_files(&mut self, paths: &[&Path], config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError>;
    
    /// Process a MeTTa code string
    fn process_metta_string(&mut self, code: &str, config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError>;
    
    /// Get stderr output (if available)
    fn stderr_output(&self) -> String;
    
    /// Shutdown backend gracefully
    fn shutdown(&mut self);
    
    /// Restart backend (for crash recovery)
    fn restart(&mut self, config: &EngineConfig) -> Result<(), PeTTaError>;
}

/// Backend metadata and capabilities
#[derive(Debug, Clone)]
pub struct BackendInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub supports_parallel: bool,
    pub supports_streaming: bool,
}

impl BackendInfo {
    pub const fn new(name: &'static str, version: &'static str) -> Self {
        Self {
            name,
            version,
            supports_parallel: false,
            supports_streaming: false,
        }
    }
    
    pub const fn with_parallel(mut self, supported: bool) -> Self {
        self.supports_parallel = supported;
        self
    }
    
    pub const fn with_streaming(mut self, supported: bool) -> Self {
        self.supports_streaming = supported;
        self
    }
}

/// Backend execution statistics
#[derive(Debug, Clone, Default)]
pub struct BackendStats {
    pub queries_executed: u64,
    pub total_execution_time_ns: u64,
    pub errors_encountered: u64,
    pub restarts: u64,
}

impl BackendStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn record_query(&mut self, execution_time_ns: u64) {
        self.queries_executed += 1;
        self.total_execution_time_ns += execution_time_ns;
    }
    
    pub fn record_error(&mut self) {
        self.errors_encountered += 1;
    }
    
    pub fn record_restart(&mut self) {
        self.restarts += 1;
    }
    
    pub fn average_query_time_ns(&self) -> Option<u64> {
        if self.queries_executed == 0 {
            None
        } else {
            Some(self.total_execution_time_ns / self.queries_executed)
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
