//! Type definitions for PeTTa
//!
//! This module provides type-level abstractions used throughout the codebase.

/// MeTTa type representation
#[derive(Debug, Clone, PartialEq, Eq)]
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
