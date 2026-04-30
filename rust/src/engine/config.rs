//! Engine configuration with smart defaults

use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Backend selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Backend {
    #[default]
    Swipl,
    Mork,
}

impl Backend {
    pub fn auto_detect() -> Self {
        #[cfg(feature = "mork")]
        { Backend::Mork }
        #[cfg(not(feature = "mork"))]
        { Backend::Swipl }
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

/// Engine configuration
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_backend_display() {
        assert_eq!(Backend::Mork.to_string(), "Mork");
        assert_eq!(Backend::Swipl.to_string(), "Swipl");
    }
}
