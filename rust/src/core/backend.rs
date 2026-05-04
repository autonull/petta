//! Unified backend trait and capabilities
//!
//! This module defines the core abstraction for execution backends,
//! providing a common interface for SWI-Prolog and MORK backends.

use std::path::Path;
use crate::values::MettaResult;
use crate::engine::EngineConfig;
use crate::Error;

/// Unified backend trait for all execution engines
///
/// This trait provides the common interface that all backends must implement,
/// enabling seamless switching between different execution engines.
pub trait Backend: Send + Sync {
    /// Backend name (e.g., "SWI-Prolog", "MORK")
    fn name(&self) -> &'static str;

    /// Backend version string
    fn version(&self) -> &'static str {
        "unknown"
    }

    /// Check if backend is alive and responsive
    fn is_alive(&mut self) -> bool;

    /// Get backend capabilities
    fn capabilities(&mut self) -> BackendCapabilities {
        BackendCapabilities::default()
    }

    /// Load and execute a single MeTTa file
    fn load_file(&mut self, path: &Path, config: &EngineConfig) -> Result<Vec<MettaResult>, Error>;

    /// Load and execute multiple MeTTa files
    fn load_files(&mut self, paths: &[&Path], config: &EngineConfig) -> Result<Vec<MettaResult>, Error> {
        let mut all_results = Vec::new();
        for path in paths {
            all_results.extend(self.load_file(path, config)?);
        }
        Ok(all_results)
    }

    /// Execute MeTTa code string
    fn execute(&mut self, code: &str, config: &EngineConfig) -> Result<Vec<MettaResult>, Error>;

    /// Restart backend (for crash recovery)
    fn restart(&mut self, config: &EngineConfig) -> Result<(), Error>;

    /// Shutdown backend gracefully
    fn shutdown(&mut self);

    /// Get stderr output (if available)
    fn stderr_output(&self) -> String {
        String::new()
    }
}

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



/// Backend health status
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

#[allow(dead_code)]
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
