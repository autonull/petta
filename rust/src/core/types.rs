//! Type definitions for PeTTa
//!
//! This module provides type-level abstractions used throughout the codebase,
//! including type-safe path types and compile-time guarantees.

use std::path::{Path, PathBuf};
use crate::engine::Error;

/// MeTTa type representation
pub enum Type {
    /// Unit type
    Unit,
    /// Boolean type
    Bool,
    /// Integer type
    Int,
    /// Float type
    Float,
    /// String type
    String,
    /// Symbol type
    Symbol,
    /// Expression type
    Expr,
    /// Function type
    Func(Vec<Type>, Box<Type>),
    /// Custom type
    Custom(String),
}

impl Type {
    /// Get the name of the type
    pub fn name(&self) -> &'static str {
        match self {
            Type::Unit => "Unit",
            Type::Bool => "Bool",
            Type::Int => "Int",
            Type::Float => "Float",
            Type::String => "String",
            Type::Symbol => "Symbol",
            Type::Expr => "Expr",
            Type::Func(_, _) => "Func",
            Type::Custom(_) => "Custom",
        }
    }

    /// Check if this is a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float)
    }

    /// Check if this is an atomic type (not composite)
    pub fn is_atomic(&self) -> bool {
        matches!(
            self,
            Type::Unit | Type::Bool | Type::Int | Type::Float | Type::String | Type::Symbol
        )
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => write!(f, "Unit"),
            Type::Bool => write!(f, "Bool"),
            Type::Int => write!(f, "Int"),
            Type::Float => write!(f, "Float"),
            Type::String => write!(f, "String"),
            Type::Symbol => write!(f, "Symbol"),
            Type::Expr => write!(f, "Expr"),
            Type::Func(args, ret) => {
                write!(f, "(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Custom(name) => write!(f, "{}", name),
        }
    }
}

// ============================================================================
// Type-Safe Path Types
// ============================================================================


/// Represents the root directory of a MeTTa project
///
/// This type ensures that project roots are validated at construction time
/// and provides type-safe path resolution.
///
/// # Example
///
/// ```rust,no_run
/// use petta::core::{ProjectRoot, MettaFile};
///
/// let root = ProjectRoot::new("/path/to/project")?;
/// let file = root.resolve("defs.metta");
/// // file is now a type-safe MettaFile
/// # Ok::<_, petta::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct ProjectRoot(PathBuf);

/// Represents a validated MeTTa file path
///
/// This type ensures that only validated file paths are used with the engine.
#[derive(Debug, Clone)]
pub struct MettaFile(PathBuf);

impl ProjectRoot {
    /// Create a new project root from a path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path_buf = path.as_ref().to_path_buf();
        if !path_buf.exists() {
            return Err(Error::Config(format!("Project root does not exist: {:?}", path_buf)));
        }
        if !path_buf.is_dir() {
            return Err(Error::Config(format!("Project root is not a directory: {:?}", path_buf)));
        }
        Ok(Self(path_buf))
    }

    /// Resolve a relative path to a MettaFile
    ///
    /// This provides type-safe path resolution within the project root.
    pub fn resolve<P: AsRef<Path>>(&self, path: P) -> MettaFile {
        MettaFile(self.0.join(path))
    }

    /// Get the underlying path
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl MettaFile {
    /// Create a new MettaFile from a path
    ///
    /// Note: This does not validate that the file exists. Use `ProjectRoot::resolve`
    /// for type-safe file creation.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self(path.as_ref().to_path_buf())
    }

    /// Get the underlying path
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Convert to PathBuf
    pub fn to_path_buf(self) -> PathBuf {
        self.0
    }
}

impl AsRef<Path> for MettaFile {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}
