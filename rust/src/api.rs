//! High-level ergonomic API for PeTTa

use std::path::{Path, PathBuf};
use std::time::Duration;
use crate::engine::{EngineConfig, MettaResult, PeTTaEngine, PeTTaError};

pub struct PeTTaBuilder {
    config: EngineConfig,
}

impl PeTTaBuilder {
    pub fn new(project_root: impl AsRef<Path>) -> Self {
        let config = EngineConfig::new(project_root.as_ref());
        Self { config }
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }

    pub fn profile(mut self, profile: bool) -> Self {
        self.config.profile = profile;
        self
    }

    pub fn max_restarts(mut self, n: u32) -> Self {
        self.config.max_restarts = n;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.query_timeout = Some(timeout);
        self
    }

    pub fn swipl_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.swipl_path = path.into();
        self
    }

    pub fn build(self) -> Result<PeTTa, PeTTaError> {
        PeTTa::with_config(&self.config)
    }
}

pub struct PeTTa {
    engine: PeTTaEngine,
}

impl PeTTa {
    pub fn builder(project_root: impl AsRef<Path>) -> PeTTaBuilder {
        PeTTaBuilder::new(project_root)
    }

    pub fn new(project_root: impl AsRef<Path>) -> Result<Self, PeTTaError> {
        let engine = PeTTaEngine::new(project_root.as_ref(), false)?;
        Ok(Self { engine })
    }

    pub fn with_config(config: &EngineConfig) -> Result<Self, PeTTaError> {
        let engine = PeTTaEngine::with_config(config)?;
        Ok(Self { engine })
    }

    pub fn eval(&mut self, expr: &str) -> Result<MettaResult, PeTTaError> {
        let results = self.engine.process_metta_string(expr)?;
        Ok(results.into_iter().next().unwrap_or(MettaResult { value: String::new() }))
    }

    pub fn eval_all(&mut self, expr: &str) -> Result<Vec<MettaResult>, PeTTaError> {
        self.engine.process_metta_string(expr)
    }

    pub fn eval_int(&mut self, expr: &str) -> Result<i64, PeTTaError> {
        let result = self.eval(expr)?;
        result.value.parse().map_err(|_| {
            PeTTaError::ProtocolError(format!("Expected integer, got: {}", result.value))
        })
    }

    pub fn eval_float(&mut self, expr: &str) -> Result<f64, PeTTaError> {
        let result = self.eval(expr)?;
        result.value.parse().map_err(|_| {
            PeTTaError::ProtocolError(format!("Expected float, got: {}", result.value))
        })
    }

    pub fn eval_bool(&mut self, expr: &str) -> Result<bool, PeTTaError> {
        let result = self.eval(expr)?;
        result.value.parse().map_err(|_| {
            PeTTaError::ProtocolError(format!("Expected bool, got: {}", result.value))
        })
    }

    pub fn eval_str(&mut self, expr: &str) -> Result<String, PeTTaError> {
        self.eval(expr).map(|r| r.value)
    }

    pub fn load(&mut self, path: impl AsRef<Path>) -> Result<Vec<MettaResult>, PeTTaError> {
        self.engine.load_metta_file(path.as_ref())
    }

    pub fn load_many(&mut self, paths: &[impl AsRef<Path>]) -> Result<Vec<MettaResult>, PeTTaError> {
        let paths: Vec<&Path> = paths.iter().map(|p| p.as_ref()).collect();
        self.engine.load_metta_files(&paths)
    }

    pub fn is_alive(&mut self) -> bool {
        self.engine.is_alive()
    }

    pub fn config(&self) -> &EngineConfig {
        self.engine.config()
    }
}
