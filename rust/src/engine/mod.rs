//! Unified PeTTa engine with backend abstraction
//!
//! This module provides a unified interface to both Prolog and MORK backends,
//! with automatic crash recovery, restart management, and ergonomic APIs.
//!
//! # Architecture
//!
//! The engine is built on a trait-based backend abstraction that allows:
//! - Multiple execution backends (SWI-Prolog, MORK)
//! - Unified error handling
//! - Consistent configuration
//! - Backend-agnostic execution
//!
//! # Example
//!
//! ```rust,no_run
//! use petta::{PeTTaEngine, EngineConfig};
//! use std::path::Path;
//!
//! let config = EngineConfig::new(Path::new("."));
//! let mut engine = PeTTaEngine::with_config(&config).unwrap();
//!
//! let result = engine.eval("!(+ 1 2)").unwrap();
//! assert_eq!(result, "3");
//! ```

mod backend;
mod backends;
mod client;
mod config;
mod errors;
mod formatters;
mod server;
mod subprocess;
mod version;

pub use backend::{BackendImpl, BackendInfo, BackendStats, HealthStatus as BackendHealth};
pub use backends::SwiplBackend;
#[cfg(feature = "mork")]
pub use backends::MorkBackend;
pub use config::{Backend, EngineConfig, EngineConfigBuilder};
pub use errors::{BackendError, BackendErrorKind, PeTTaError, parse_backend_error};
pub use formatters::{create_formatter, CompactFormatter, JsonFormatter, OutputFormatter, PrettyFormatter, SExprFormatter};
pub use version::{MIN_SWIPL_VERSION, swipl_available};

// Re-export for internal use
use crate::values::{MettaResult, MettaValue};

use std::path::Path;

// ============================================================================
// Backend State Management
// ============================================================================

/// Unified backend state using trait objects
enum BackendState {
Swipl(backends::SwiplBackend),
#[cfg(feature = "mork")]
Mork(backends::MorkBackend),
}

impl BackendState {
fn new(config: &EngineConfig) -> Result<Self, PeTTaError> {
match config.backend {
#[cfg(feature = "mork")]
Backend::Mork => Ok(Self::Mork(backends::MorkBackend::new())),
#[cfg(not(feature = "mork"))]
Backend::Mork => Err(PeTTaError::Mork(
"Mork backend not available (requires nightly Rust)".into()
)),
Backend::Swipl => backends::SwiplBackend::new(config).map(Self::Swipl),
}
}

fn name(&self) -> &'static str {
match self {
#[cfg(feature = "mork")]
Self::Mork(backend) => backend.name(),
Self::Swipl(backend) => backend.name(),
}
}

fn is_alive(&mut self) -> bool {
match self {
#[cfg(feature = "mork")]
Self::Mork(backend) => backend.is_alive(),
Self::Swipl(backend) => backend.is_alive(),
}
}

fn stderr(&self) -> String {
match self {
#[cfg(feature = "mork")]
Self::Mork(backend) => backend.stderr_output(),
Self::Swipl(backend) => backend.stderr_output(),
}
}

fn restart(&mut self, config: &EngineConfig) -> Result<(), PeTTaError> {
match self {
#[cfg(feature = "mork")]
Self::Mork(backend) => backend.restart(config),
Self::Swipl(backend) => backend.restart(config),
}
}

fn shutdown(&mut self) {
match self {
#[cfg(feature = "mork")]
Self::Mork(backend) => backend.shutdown(),
Self::Swipl(backend) => backend.shutdown(),
}
}
}

// BackendImpl trait methods delegated through BackendState
impl BackendState {
fn load_metta_file(&mut self, path: &Path, config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
match self {
#[cfg(feature = "mork")]
Self::Mork(backend) => backend.load_metta_file(path, config),
Self::Swipl(backend) => backend.load_metta_file(path, config),
}
}

fn load_metta_files(&mut self, paths: &[&Path], config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
match self {
#[cfg(feature = "mork")]
Self::Mork(backend) => backend.load_metta_files(paths, config),
Self::Swipl(backend) => backend.load_metta_files(paths, config),
}
}

fn process_metta_string(&mut self, code: &str, config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
match self {
#[cfg(feature = "mork")]
Self::Mork(backend) => backend.process_metta_string(code, config),
Self::Swipl(backend) => backend.process_metta_string(code, config),
}
}
}

// ============================================================================
// PeTTa Engine - Main Interface
// ============================================================================

/// Main PeTTa execution engine
///
/// Provides a unified interface to MeTTa execution with support for:
/// - Multiple backends (SWI-Prolog, MORK)
/// - Automatic crash recovery
/// - Error handling with suggestions
/// - Performance profiling
pub struct PeTTaEngine {
backend: BackendState,
config: EngineConfig,
restarts: u32,
}

impl PeTTaEngine {
/// Create a new engine with the given configuration
///
/// # Example
///
/// ```rust,no_run
/// use petta::{PeTTaEngine, EngineConfig};
/// use std::path::Path;
///
/// let config = EngineConfig::new(Path::new("."));
/// let mut engine = PeTTaEngine::with_config(&config).unwrap();
/// ```
pub fn with_config(config: &EngineConfig) -> Result<Self, PeTTaError> {
Ok(Self {
backend: BackendState::new(config)?,
config: config.clone(),
restarts: 0,
})
}

/// Create a new engine with default configuration for the given project root
pub fn new(project_root: &Path, verbose: bool) -> Result<Self, PeTTaError> {
let config = EngineConfig::new(project_root).verbose(verbose);
Self::with_config(&config)
}

/// Get the backend name
pub fn backend_name(&self) -> &'static str {
self.backend.name()
}

/// Check if the backend is alive and responsive
pub fn is_alive(&mut self) -> bool {
self.backend.is_alive()
}

/// Load and execute a single MeTTa file
pub fn load_metta_file(&mut self, path: &Path) -> Result<Vec<MettaResult>, PeTTaError> {
self.retry_on_crash(|backend, cfg| backend.load_metta_file(path, cfg))
}

/// Load and execute multiple MeTTa files
pub fn load_metta_files(&mut self, paths: &[&Path]) -> Result<Vec<MettaResult>, PeTTaError> {
self.retry_on_crash(|backend, cfg| backend.load_metta_files(paths, cfg))
}

/// Process a MeTTa code string
pub fn process_metta_string(&mut self, code: &str) -> Result<Vec<MettaResult>, PeTTaError> {
self.retry_on_crash(|backend, cfg| backend.process_metta_string(code, cfg))
}

/// Get stderr output from the backend
pub fn stderr_output(&self) -> String {
self.backend.stderr()
}

/// Get the current configuration
pub fn config(&self) -> &EngineConfig {
&self.config
}

/// Shutdown the backend gracefully
pub fn shutdown(&mut self) {
self.backend.shutdown();
}

// =========================================================================
// High-Level Convenience Methods
// =========================================================================

/// Evaluate a MeTTa expression and return the first result as a string
///
/// This is a convenience method for simple queries that return a single result.
pub fn eval(&mut self, code: &str) -> Result<String, PeTTaError> {
self.process_metta_string(code)
.and_then(|mut r| r.pop().ok_or_else(|| PeTTaError::Protocol("No results".into())))
.map(|r| r.value)
}

/// Evaluate and parse as integer
///
/// Convenience method for arithmetic expressions.
pub fn eval_int(&mut self, code: &str) -> Result<i64, PeTTaError> {
self.eval(code)?.parse().map_err(|_| PeTTaError::Protocol("Not an integer".into()))
}

/// Load and execute multiple files
///
/// Generic over path types for convenience.
pub fn load_files<P: AsRef<Path>>(&mut self, paths: &[P]) -> Result<Vec<MettaResult>, PeTTaError> {
let refs: Vec<&Path> = paths.iter().map(|p| p.as_ref()).collect();
self.load_metta_files(&refs)
}

/// Check if engine is healthy and responsive
pub fn health_check(&mut self) -> bool {
self.is_alive() || self.process_metta_string("!(id 1)").is_ok()
}

// =========================================================================
// Internal Implementation
// =========================================================================

fn retry_on_crash<F>(&mut self, mut f: F) -> Result<Vec<MettaResult>, PeTTaError>
where
F: FnMut(&mut BackendState, &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError>,
{
let mut attempts = 0u32;
loop {
match f(&mut self.backend, &self.config) {
Err(PeTTaError::Protocol(ref m)) if m.contains("child closed") => {
if attempts >= self.config.max_restarts {
return Err(PeTTaError::Crash { restarts: self.restarts });
}
attempts += 1;
self.backend.restart(&self.config)?;
self.restarts = self.restarts.saturating_add(1);
}
other => return other,
}
}
}
}

impl Drop for PeTTaEngine {
fn drop(&mut self) {
self.shutdown();
}
}
