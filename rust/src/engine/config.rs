//! Engine configuration with smart defaults

use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Backend {
    #[default]
    Swipl,
    Mork,
}

impl Backend {
    pub fn auto_detect() -> Self {
        #[cfg(feature = "mork")]
        return Backend::Mork;
        #[cfg(not(feature = "mork"))]
        return Backend::Swipl;
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Backend::Mork => write!(f, "Mork"),
            Backend::Swipl => write!(f, "Swipl"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub backend: Backend,
    pub swipl_path: PathBuf,
    pub src_dir: PathBuf,
    pub verbose: bool,
    pub max_restarts: u32,
    pub timeout: Option<Duration>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            backend: Backend::auto_detect(),
            swipl_path: PathBuf::from("swipl"),
            src_dir: PathBuf::from("prolog"),
            verbose: false,
            max_restarts: 3,
            timeout: None,
        }
    }
}

impl EngineConfig {
    pub fn new(project_root: &Path) -> Self {
        Self {
            src_dir: project_root.join("prolog"),
            ..Default::default()
        }
    }

    pub fn builder() -> EngineConfigBuilder {
        EngineConfigBuilder::default()
    }

    pub fn verbose(mut self, v: bool) -> Self {
        self.verbose = v;
        self
    }

    pub fn max_restarts(mut self, n: u32) -> Self {
        self.max_restarts = n;
        self
    }

    pub fn backend(mut self, b: Backend) -> Self {
        self.backend = b;
        self
    }

    pub fn timeout(mut self, t: Duration) -> Self {
        self.timeout = Some(t);
        self
    }
}

#[derive(Debug, Default)]
pub struct EngineConfigBuilder {
    backend: Option<Backend>,
    src_dir: Option<PathBuf>,
    verbose: bool,
    max_restarts: u32,
    timeout: Option<Duration>,
}

impl EngineConfigBuilder {
    pub fn backend(mut self, backend: Backend) -> Self {
        self.backend = Some(backend);
        self
    }

    pub fn src_dir(mut self, path: PathBuf) -> Self {
        self.src_dir = Some(path);
        self
    }

    pub fn verbose(mut self, v: bool) -> Self {
        self.verbose = v;
        self
    }

    pub fn max_restarts(mut self, n: u32) -> Self {
        self.max_restarts = n;
        self
    }

    pub fn timeout(mut self, t: Duration) -> Self {
        self.timeout = Some(t);
        self
    }

    pub fn build(self) -> EngineConfig {
        EngineConfig {
            backend: self.backend.unwrap_or_else(Backend::auto_detect),
            swipl_path: PathBuf::from("swipl"),
            src_dir: self.src_dir.unwrap_or_else(|| PathBuf::from("prolog")),
            verbose: self.verbose,
            max_restarts: self.max_restarts,
            timeout: self.timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_display() {
        assert_eq!(Backend::Mork.to_string(), "Mork");
        assert_eq!(Backend::Swipl.to_string(), "Swipl");
    }

    #[test]
    fn test_builder_pattern() {
        let config = EngineConfig::builder()
            .backend(Backend::Mork)
            .verbose(true)
            .max_restarts(5)
            .build();

        assert_eq!(config.backend, Backend::Mork);
        assert!(config.verbose);
        assert_eq!(config.max_restarts, 5);
    }
}
