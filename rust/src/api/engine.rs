//! PeTTa engine implementations with ergonomic API
//!
//! This module provides both simplified (`PeTTa`) and full-featured (`PeTTaEngine`)
//! interfaces to the MeTTa execution engine.

use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use crate::engine::{EngineConfig, PeTTaEngine as CoreEngine, Backend};
use crate::values::MettaResult;
use crate::Error;
use super::ExecutionResult;

// ============================================================================
// Type-State Pattern for Compile-Time Safety
// ============================================================================

/// Marker type for uninitialized engine state
pub struct Uninitialized;
/// Marker type for initialized engine state
pub struct Initialized;
/// Marker type for running engine state
pub struct Running;

// ============================================================================
// PeTTa - Simplified Interface
// ============================================================================

/// Simplified PeTTa engine with ergonomic defaults.
///
/// `PeTTa` provides a streamlined interface for common MeTTa operations
/// with minimal boilerplate and sensible defaults.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust,no_run
/// use petta::api::PeTTa;
///
/// let mut engine = PeTTa::new()?;
/// let result = engine.eval("!(+ 1 2)")?;
/// println!("Result: {}", result);
/// # Ok::<_, petta::Error>(())
/// ```
///
/// ## Builder Pattern
///
/// ```rust,no_run
/// use petta::api::{PeTTa, Backend};
///
/// let mut engine = PeTTa::builder()
///     .backend(Backend::Mork)
///     .verbose(true)
///     .build()?;
/// # Ok::<_, petta::Error>(())
/// ```
pub struct PeTTa {
    engine: CoreEngine,
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
    /// use petta::api::PeTTa;
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
    /// use petta::api::PeTTa;
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
    /// use petta::api::{PeTTa, EngineConfig, Backend};
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
            engine: CoreEngine::with_config(&config)?,
            loaded_files: Vec::new(),
        })
    }

    /// Create a new builder for custom configuration.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::api::{PeTTa, Backend};
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
    /// use petta::api::PeTTa;
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
    /// use petta::api::PeTTa;
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
    /// use petta::api::PeTTa;
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
    /// use petta::api::PeTTa;
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
    /// use petta::api::PeTTa;
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

// ============================================================================
// Builder Pattern
// ============================================================================

/// Builder for ergonomic PeTTa configuration.
///
/// # Example
///
/// ```rust,no_run
/// use petta::api::{PeTTa, Backend};
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
    pub fn backend(mut self, backend: Backend) -> Self {
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

// ============================================================================
// PeTTaEngine - Full-Featured Interface
// ============================================================================

/// Full-featured PeTTa execution engine.
///
/// `PeTTaEngine` provides complete control over MeTTa execution with
/// advanced configuration options and detailed result handling.
///
/// # Example
///
/// ```rust,no_run
/// use petta::api::{PeTTaEngine, EngineConfig};
/// use std::path::Path;
///
/// let config = EngineConfig::new(Path::new("."));
/// let mut engine = PeTTaEngine::with_config(&config)?;
///
/// let result = engine.execute("!(+ 1 2)")?;
/// println!("Results: {:?}", result);
/// # Ok::<_, petta::Error>(())
/// ```
pub struct PeTTaEngine {
    engine: CoreEngine,
}

impl PeTTaEngine {
    /// Create engine with explicit configuration.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::api::{PeTTaEngine, EngineConfig};
    /// use std::path::Path;
    ///
    /// let config = EngineConfig::new(Path::new("."));
    /// let mut engine = PeTTaEngine::with_config(&config)?;
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn with_config(config: &EngineConfig) -> Result<Self, Error> {
        Ok(Self {
            engine: CoreEngine::with_config(config)?,
        })
    }

    /// Create engine with default configuration.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            engine: CoreEngine::with_config(&EngineConfig::default())?,
        })
    }

    /// Execute MeTTa code and return structured results.
    ///
    /// # Arguments
    ///
    /// * `code` - MeTTa code to execute
    ///
    /// # Returns
    ///
    /// Structured execution results with statistics
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use petta::api::PeTTaEngine;
    ///
    /// let mut engine = PeTTaEngine::new()?;
    /// let result = engine.execute("!(+ 1 2)")?;
    /// println!("Result: {:?}", result.first());
    /// # Ok::<_, petta::Error>(())
    /// ```
    pub fn execute(&mut self, code: &str) -> Result<ExecutionResult, Error> {
        let results = self.engine.process_metta_string(code)?;
        Ok(ExecutionResult::from_results(results))
    }

    /// Load and execute a single MeTTa file.
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<MettaResult>, Error> {
        self.engine.load_metta_file(path.as_ref())
    }

    /// Load and execute multiple MeTTa files.
    pub fn load_files<P: AsRef<Path>>(&mut self, paths: &[P]) -> Result<Vec<MettaResult>, Error> {
        let path_refs: Vec<&Path> = paths.iter().map(|p| p.as_ref()).collect();
        self.engine.load_metta_files(&path_refs)
    }

    /// Get backend name
    pub fn backend_name(&self) -> &'static str {
        self.engine.backend_name()
    }

    /// Check if backend is alive
    pub fn is_alive(&mut self) -> bool {
        self.engine.is_alive()
    }

    /// Shutdown the engine
    pub fn shutdown(&mut self) {
        self.engine.shutdown();
    }
}

impl Default for PeTTaEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create PeTTaEngine")
    }
}

// ============================================================================
// Type-State PeTTa Engine (Compile-Time State Validation)
// ============================================================================

/// PeTTa engine with type-state for compile-time state validation.
///
/// This version of PeTTa uses the type-state pattern to ensure that
/// operations are only performed in valid states at compile time.
///
/// # Example
///
/// ```rust,no_run
/// use petta::api::{PeTTaTyped, Uninitialized, Initialized, Running};
///
/// // Create and initialize
/// let engine = PeTTaTyped::<Uninitialized>::new()?;
/// let engine = engine.init()?;
///
/// // Start the engine (transition to Running state)
/// let mut running = engine.start()?;
///
/// // Execute queries
/// let result = running.eval("!(+ 1 2)")?;
///
/// // Stop the engine (transition back to Initialized)
/// let _engine = running.stop()?;
/// # Ok::<_, petta::Error>(())
/// ```
pub struct PeTTaTyped<State> {
    engine: CoreEngine,
    loaded_files: Vec<PathBuf>,
    _state: PhantomData<State>,
}

// Type-safe constructors
impl PeTTaTyped<Uninitialized> {
    /// Create new uninitialized PeTTa engine
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            engine: CoreEngine::with_config(&EngineConfig::default())?,
            loaded_files: Vec::new(),
            _state: PhantomData,
        })
    }

    /// Create with custom configuration
    pub fn with_config(config: EngineConfig) -> Result<Self, Error> {
        Ok(Self {
            engine: CoreEngine::with_config(&config)?,
            loaded_files: Vec::new(),
            _state: PhantomData,
        })
    }

    /// Initialize the engine (transition to Initialized state)
    pub fn init(self) -> Result<PeTTaTyped<Initialized>, Error> {
        Ok(PeTTaTyped {
            engine: self.engine,
            loaded_files: self.loaded_files,
            _state: PhantomData,
        })
    }
}

impl Default for PeTTaTyped<Uninitialized> {
    fn default() -> Self {
        Self::new().expect("Failed to create PeTTa engine")
    }
}

// Initialized state - ready to run
impl PeTTaTyped<Initialized> {
    /// Start the engine (transition to Running state)
    pub fn start(self) -> Result<PeTTaTyped<Running>, Error> {
        Ok(PeTTaTyped {
            engine: self.engine,
            loaded_files: self.loaded_files,
            _state: PhantomData,
        })
    }

    /// Load a file in initialized state
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<MettaResult>, Error> {
        let path_buf = path.as_ref().to_path_buf();
        let results = self.engine.load_metta_file(path.as_ref())?;
        self.loaded_files.push(path_buf);
        Ok(results)
    }
}

// Running state - can execute queries
impl PeTTaTyped<Running> {
    /// Evaluate a MeTTa expression
    pub fn eval(&mut self, code: &str) -> Result<String, Error> {
        self.engine.eval(code)
    }

    /// Execute MeTTa code
    pub fn execute(&mut self, code: &str) -> Result<Vec<MettaResult>, Error> {
        self.engine.process_metta_string(code)
    }

    /// Stop the engine (transition to Initialized state)
    pub fn stop(self) -> Result<PeTTaTyped<Initialized>, Error> {
        Ok(PeTTaTyped {
            engine: self.engine,
            loaded_files: self.loaded_files,
            _state: PhantomData,
        })
    }

    /// Check if engine is healthy
    pub fn is_healthy(&mut self) -> bool {
        self.engine.health_check()
    }

    /// Get backend name
    pub fn backend_name(&self) -> &'static str {
        self.engine.backend_name()
    }
}

// Convenience methods for Running state
impl PeTTaTyped<Running> {
    /// Evaluate and parse as integer
    pub fn eval_int(&mut self, code: &str) -> Result<i64, Error> {
        self.engine.eval_int(code)
    }

    /// Load a file
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<MettaResult>, Error> {
        let path_buf = path.as_ref().to_path_buf();
        let results = self.engine.load_metta_file(path.as_ref())?;
        self.loaded_files.push(path_buf);
        Ok(results)
    }

    /// Get list of loaded files
    pub fn loaded_files(&self) -> &[PathBuf] {
        &self.loaded_files
    }
}
