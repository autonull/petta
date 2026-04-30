//! Unified PeTTa engine with backend abstraction
//!
//! This module provides a unified interface to both Prolog and MORK backends,
//! with automatic crash recovery, restart management, and ergonomic APIs.

mod client;
mod config;
mod errors;
mod formatters;
mod server;
mod subprocess;
mod values;
mod version;

#[cfg(feature = "mork")]
mod mork_engine;

pub use config::{Backend, EngineConfig, EngineConfigBuilder};
pub use errors::{BackendError, BackendErrorKind, PeTTaError, parse_backend_error};
pub use formatters::{create_formatter, CompactFormatter, JsonFormatter, OutputFormatter, PrettyFormatter, SExprFormatter};
pub use values::{MettaResult, MettaValue};
pub use version::{MIN_SWIPL_VERSION, swipl_available};

use std::io::{BufReader, Write};
use std::path::Path;

#[cfg(feature = "mork")]
use mork_engine::MORKEngine;

use client::{load_metta_file as load_file_client, load_metta_files as load_files_client, process_metta_string as process_string_client};
use subprocess::SubprocessManager;

// ============================================================================
// Backend State Management
// ============================================================================

enum BackendState {
    #[cfg(feature = "mork")]
    Mork(MORKEngine),
    Swipl(SwiplState),
}

struct SwiplState {
    child: Option<std::process::Child>,
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
    stderr: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl SwiplState {
    fn new(config: &EngineConfig) -> Result<Self, PeTTaError> {
        let mgr = SubprocessManager::new(config.clone());
        let (child, stdin, stdout, stderr) = mgr.spawn()?;
        Ok(Self { child: Some(child), stdin, stdout, stderr })
    }

    fn kill(&mut self) {
        if let Some(mut c) = self.child.take() {
            let _ = c.kill();
            let _ = c.wait();
        }
    }
}

impl BackendState {
    fn new(config: &EngineConfig) -> Result<Self, PeTTaError> {
        match config.backend {
            #[cfg(feature = "mork")]
            Backend::Mork => Ok(Self::Mork(MORKEngine::new())),
            #[cfg(not(feature = "mork"))]
            Backend::Mork => Err(PeTTaError::Mork("Mork backend not available (requires nightly Rust)".into())),
            Backend::Swipl => SwiplState::new(config).map(Self::Swipl),
        }
    }

    #[inline]
    fn is_mork(&self) -> bool {
        #[cfg(feature = "mork")]
        return matches!(self, Self::Mork(..));
        #[cfg(not(feature = "mork"))]
        return false;
    }

    fn stderr(&self) -> String {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(_) => String::new(),
            Self::Swipl(s) => s.stderr.lock()
                .map(|d| String::from_utf8_lossy(&d).into_owned())
                .unwrap_or_else(|_| "<error>".into()),
        }
    }

    #[allow(irrefutable_let_patterns)]
    fn restart(&mut self, config: &EngineConfig) -> Result<(), PeTTaError> {
        if let Self::Swipl(s) = self {
            s.kill();
            *s = SwiplState::new(config)?;
        }
        Ok(())
    }

    #[allow(irrefutable_let_patterns)]
    fn shutdown(&mut self) {
        if let Self::Swipl(s) = self {
            let _ = s.stdin.write_all(&[b'Q', 0, 0, 0, 0]);
            let _ = s.stdin.flush();
            s.kill();
        }
    }
}

// ============================================================================
// PeTTa Engine - Main Interface
// ============================================================================

pub struct PeTTaEngine {
    backend: BackendState,
    config: EngineConfig,
    restarts: u32,
}

impl PeTTaEngine {
    /// Create a new engine with the given configuration
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

    /// Check if the backend is alive and responsive
    pub fn is_alive(&mut self) -> bool {
        self.backend.is_mork()
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
    pub fn eval(&mut self, code: &str) -> Result<String, PeTTaError> {
        self.process_metta_string(code)
            .and_then(|mut r| r.pop().ok_or_else(|| PeTTaError::Protocol("No results".into())))
            .map(|r| r.value)
    }

    /// Evaluate and parse as integer
    pub fn eval_int(&mut self, code: &str) -> Result<i64, PeTTaError> {
        self.eval(code)?.parse().map_err(|_| PeTTaError::Protocol("Not an integer".into()))
    }

    /// Load and execute multiple files
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

// ============================================================================
// Backend State Implementation - Backend-specific operations
// ============================================================================

impl BackendState {
    fn load_metta_file(&mut self, path: &Path, config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(mork) => std::fs::read_to_string(path)
                .map_err(|e| PeTTaError::PathError(e.to_string()))
                .map(|s| mork.process(&s).into_iter().map(|v| MettaResult { value: v }).collect()),
            Self::Swipl(state) => load_file_client(&mut state.stdin, &mut state.stdout, path, config),
        }
    }

    fn load_metta_files(&mut self, paths: &[&Path], config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(mork) => {
                let all = paths.iter()
                    .filter_map(|p| std::fs::read_to_string(p).ok())
                    .flat_map(|s| mork.process(&s).into_iter().map(move |v| MettaResult { value: v }))
                    .collect();
                Ok(all)
            }
            Self::Swipl(state) => load_files_client(&mut state.stdin, &mut state.stdout, paths, config),
        }
    }

    fn process_metta_string(&mut self, code: &str, config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(mork) => Ok(mork.process(code).into_iter().map(|s| MettaResult { value: s }).collect()),
            Self::Swipl(state) => process_string_client(&mut state.stdin, &mut state.stdout, code, config),
        }
    }
}
