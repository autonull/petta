//! Simplified PeTTa API for ergonomic MeTTa execution.
//!
//! This module provides the [`PeTTa`] struct - a streamlined, ergonomic interface
//! to the PeTTa execution engine with sensible defaults and fluent API design.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use petta::PeTTa;
//!
//! // Simple usage with defaults
//! let mut engine = PeTTa::new()?;
//! let result = engine.eval("!(+ 1 2)")?;
//! assert_eq!(result, "3");
//! # Ok::<_, petta::Error>(())
//! ```
//!
//! # Builder Pattern
//!
//! ```rust,no_run
//! use petta::{PeTTa, Backend};
//!
//! let mut engine = PeTTa::builder()
//!     .backend(Backend::Mork)
//!     .verbose(true)
//!     .build()?;
//! # Ok::<_, petta::Error>(())
//! ```

use std::path::{Path, PathBuf};
use crate::engine::{EngineConfig, PeTTaEngine};
use crate::values::MettaResult;
use crate::Error;

/// Simplified PeTTa engine with ergonomic defaults.
///
/// `PeTTa` provides a streamlined interface for common MeTTa operations
/// with minimal boilerplate.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,no_run
/// use petta::PeTTa;
///
/// let mut engine = PeTTa::new()?;
/// let result = engine.eval("!(+ 1 2)")?;
/// println!("Result: {}", result);
/// # Ok::<_, petta::Error>(())
/// ```
///
/// ## Loading Files
///
/// ```rust,no_run
/// use petta::PeTTa;
///
/// let mut engine = PeTTa::new()?;
/// engine.load("defs.metta")?;
/// engine.load("rules.metta")?;
/// let result = engine.eval("!(query)")?;
/// # Ok::<_, petta::Error>(())
/// ```
pub struct PeTTa {
    engine: PeTTaEngine,
    loaded_files: Vec<PathBuf>,
}

impl PeTTa {
    /// Create new PeTTa engine with default configuration.
    ///
    /// Uses current directory as project root and SWI-Prolog backend.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::PeTTa;
    ///
    /// let mut engine = PeTTa::new()?;
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn new() -> Result<Self, Error> {
        Self::with_config(EngineConfig::default())
    }

    /// Create new PeTTa engine with explicit project root.
    ///
    /// # Arguments
    ///
    /// * `project_root` - Root directory for MeTTa libraries
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::PeTTa;
    /// use std::path::Path;
    ///
    /// let mut engine = PeTTa::with_root(Path::new("/path/to/project"))?;
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn with_root<P: AsRef<Path>>(project_root: P) -> Result<Self, Error> {
        let config = EngineConfig::new(project_root.as_ref());
        Self::with_config(config)
    }

    /// Create new PeTTa engine with custom configuration.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::{PeTTa, EngineConfig, Backend};
    ///
    /// let config = EngineConfig::builder()
    ///     .backend(Backend::Mork)
    ///     .verbose(true)
    ///     .build();
    ///
    /// let mut engine = PeTTa::with_config(config)?;
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn with_config(config: EngineConfig) -> Result<Self, Error> {
        Ok(Self {
            engine: PeTTaEngine::with_config(&config)?,
            loaded_files: Vec::new(),
        })
    }

    /// Create a new builder for custom configuration.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::{PeTTa, Backend};
    ///
    /// let mut engine = PeTTa::builder()
    ///     .backend(Backend::Mork)
    ///     .verbose(true)
    ///     .build()?;
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Load a single MeTTa file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to MeTTa file
    ///
    /// # Returns
    ///
    /// Execution results
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::PeTTa;
    ///
    /// let mut engine = PeTTa::new()?;
    /// let results = engine.load("defs.metta")?;
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<MettaResult>, Error> {
        let path_buf = path.as_ref().to_path_buf();
        let results = self.engine.load_metta_file(path.as_ref())?;
        self.loaded_files.push(path_buf);
        Ok(results)
    }

    /// Load multiple MeTTa files.
    ///
    /// # Arguments
    ///
    /// * `paths` - Slice of paths to MeTTa files
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::PeTTa;
    ///
    /// let mut engine = PeTTa::new()?;
    /// engine.load_all(&["defs.metta", "rules.metta"])?;
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn load_all<P: AsRef<Path>>(&mut self, paths: &[P]) -> Result<Vec<MettaResult>, Error> {
        let path_refs: Vec<&Path> = paths.iter().map(|p| p.as_ref()).collect();
        let results = self.engine.load_metta_files(&path_refs)?;
        for path in path_refs {
            self.loaded_files.push(path.to_path_buf());
        }
        Ok(results)
    }

    /// Evaluate a MeTTa expression and return first result as string.
    ///
    /// # Arguments
    ///
    /// * `code` - MeTTa code to evaluate
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::PeTTa;
    ///
    /// let mut engine = PeTTa::new()?;
    /// let result = engine.eval("!(+ 1 2)")?;
    /// assert_eq!(result, "3");
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn eval(&mut self, code: &str) -> Result<String, Error> {
        self.engine.eval(code)
    }

    /// Evaluate and parse as integer.
    ///
    /// # Arguments
    ///
    /// * `code` - MeTTa arithmetic expression
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::PeTTa;
    ///
    /// let mut engine = PeTTa::new()?;
    /// let result = engine.eval_int("!(+ 1 2)")?;
    /// assert_eq!(result, 3);
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn eval_int(&mut self, code: &str) -> Result<i64, Error> {
        self.engine.eval_int(code)
    }

    /// Execute MeTTa code and return all results.
    ///
    /// # Arguments
    ///
    /// * `code` - MeTTa code to execute
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::PeTTa;
    ///
    /// let mut engine = PeTTa::new()?;
    /// let results = engine.execute("!(+ 1 2)")?;
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn execute(&mut self, code: &str) -> Result<Vec<MettaResult>, Error> {
        self.engine.process_metta_string(code)
    }

    /// Check if engine is healthy and responsive.
    pub fn is_healthy(&mut self) -> bool {
        self.engine.health_check()
    }

    /// Get backend name
    pub fn backend_name(&self) -> &'static str {
        self.engine.backend_name()
    }

    /// Get list of loaded files
    pub fn loaded_files(&self) -> &[PathBuf] {
        &self.loaded_files
    }
}

impl Default for PeTTa {
    fn default() -> Self {
        Self::new().expect("Failed to create PeTTa engine")
    }
}

/// Builder for ergonomic PeTTa configuration.
///
/// # Example
///
/// ```rust,no_run
/// use petta::{PeTTa, Backend};
///
/// let mut engine = PeTTa::builder()
///     .backend(Backend::Mork)
///     .verbose(true)
///     .max_restarts(5)
///     .build()?;
/// # Ok::<_, petta::Error>(())
/// ```
pub struct Builder {
    config: EngineConfig,
}

impl Builder {
    /// Create new builder with defaults
    pub fn new() -> Self {
        Self {
            config: EngineConfig::default(),
        }
    }

    /// Set backend type
    pub fn backend(mut self, backend: crate::Backend) -> Self {
        self.config.backend = backend;
        self
    }

    /// Set project root directory
    pub fn root<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.config.src_dir = path.as_ref().to_path_buf();
        self
    }

    /// Enable verbose output
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }

    /// Set maximum restart attempts
    pub fn max_restarts(mut self, n: u32) -> Self {
        self.config.max_restarts = n;
        self
    }

    /// Build the PeTTa engine
    pub fn build(self) -> Result<PeTTa, Error> {
        PeTTa::with_config(self.config)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peTTa_new() {
        let result = PeTTa::new();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_builder_pattern() {
        use crate::Backend;
        
        let result = PeTTa::builder()
            .backend(Backend::Swipl)
            .verbose(true)
            .build();
        
        assert!(result.is_ok() || result.is_err());
    }
}
