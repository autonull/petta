//! Unified PeTTa engine with backend abstraction

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

pub use config::{Backend, EngineConfig};
pub use errors::{BackendError, BackendErrorKind, PeTTaError, parse_backend_error};
pub use formatters::{create_formatter, CompactFormatter, JsonFormatter, OutputFormatter, PrettyFormatter, SExprFormatter};
pub use values::{MettaResult, MettaValue};
pub use version::{MIN_SWIPL_VERSION, swipl_available};

use std::io::{BufReader, Write};
use std::path::Path;

#[cfg(feature = "mork")]
use mork_engine::MORKEngine;

use client::{load_metta_file, load_metta_files, process_metta_string};
use subprocess::SubprocessManager;

/// Backend state
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
        Ok(Self {
            child: Some(child),
            stdin,
            stdout,
            stderr,
        })
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
            Backend::Swipl | _ => {
                let state = SwiplState::new(config)?;
                Ok(Self::Swipl(state))
            }
        }
    }
    
    fn is_mork(&self) -> bool {
        #[cfg(feature = "mork")]
        return matches!(self, Self::Mork(..));
        #[cfg(not(feature = "mork"))]
        false
    }
    
    fn stderr(&self) -> String {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(_) => String::new(),
            Self::Swipl(s) => s.stderr.lock().map(|d| String::from_utf8_lossy(&d).to_string())
                .unwrap_or_else(|_| "<error>".into()),
        }
    }
    
    fn restart(&mut self, config: &EngineConfig) -> Result<(), PeTTaError> {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(_) => Ok(()),
            Self::Swipl(s) => {
                s.kill();
                *s = SwiplState::new(config)?;
                Ok(())
            }
        }
    }
    
    fn shutdown(&mut self) {
        match self {
            Self::Swipl(s) => {
                let _ = s.stdin.write_all(&[b'Q', 0, 0, 0, 0]);
                let _ = s.stdin.flush();
                s.kill();
            }
            #[cfg(feature = "mork")]
            Self::Mork(_) => {}
        }
    }
}

/// Main PeTTa engine
pub struct PeTTaEngine {
    backend: BackendState,
    config: EngineConfig,
    restarts: u32,
}

impl PeTTaEngine {
    pub fn with_config(config: &EngineConfig) -> Result<Self, PeTTaError> {
        Ok(Self {
            backend: BackendState::new(config)?,
            config: config.clone(),
            restarts: 0,
        })
    }
    
    pub fn new(project_root: &Path, verbose: bool) -> Result<Self, PeTTaError> {
        let config = EngineConfig::new(project_root).verbose(verbose);
        Self::with_config(&config)
    }
    
    pub fn is_alive(&mut self) -> bool {
        self.backend.is_mork()
    }
    
    pub fn load_metta_file(&mut self, path: &Path) -> Result<Vec<MettaResult>, PeTTaError> {
        let cfg = self.config.clone();
        self.retry_on_crash(|backend| backend.load_metta_file(path, &cfg))
    }
    
    pub fn load_metta_files(&mut self, paths: &[&Path]) -> Result<Vec<MettaResult>, PeTTaError> {
        let cfg = self.config.clone();
        self.retry_on_crash(|backend| backend.load_metta_files(paths, &cfg))
    }
    
    pub fn process_metta_string(&mut self, code: &str) -> Result<Vec<MettaResult>, PeTTaError> {
        let cfg = self.config.clone();
        self.retry_on_crash(|backend| backend.process_metta_string(code, &cfg))
    }
    
    pub fn stderr_output(&self) -> String {
        self.backend.stderr()
    }
    
    pub fn config(&self) -> &EngineConfig {
        &self.config
    }
    
    pub fn shutdown(&mut self) {
        self.backend.shutdown();
    }
    
    fn retry_on_crash<F>(&mut self, mut f: F) -> Result<Vec<MettaResult>, PeTTaError>
    where
        F: FnMut(&mut BackendState) -> Result<Vec<MettaResult>, PeTTaError>,
    {
        let mut attempts = 0u32;
        loop {
            match f(&mut self.backend) {
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

// === BackendState implementations ===

impl BackendState {
    fn load_metta_file(&mut self, path: &Path, config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(mork) => {
                let s = std::fs::read_to_string(path)
                    .map_err(|e| PeTTaError::PathError(e.to_string()))?;
                Ok(mork.process(&s).into_iter().map(|v| MettaResult { value: v }).collect())
            }
            Self::Swipl(state) => load_metta_file(&mut state.stdin, &mut state.stdout, path, config),
        }
    }
    
    fn load_metta_files(&mut self, paths: &[&Path], config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(mork) => {
                let mut all = Vec::new();
                for p in paths {
                    let s = std::fs::read_to_string(p)
                        .map_err(|e| PeTTaError::PathError(e.to_string()))?;
                    all.extend(mork.process(&s).into_iter().map(|v| MettaResult { value: v }));
                }
                Ok(all)
            }
            Self::Swipl(state) => {
                let refs: Vec<&Path> = paths.to_vec();
                load_metta_files(&mut state.stdin, &mut state.stdout, &refs, config)
            }
        }
    }
    
    fn process_metta_string(&mut self, code: &str, config: &EngineConfig) -> Result<Vec<MettaResult>, PeTTaError> {
        match self {
            #[cfg(feature = "mork")]
            Self::Mork(mork) => Ok(mork.process(code).into_iter().map(|s| MettaResult { value: s }).collect()),
            Self::Swipl(state) => process_metta_string(&mut state.stdin, &mut state.stdout, code, config),
        }
    }
}
