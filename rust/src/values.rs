//! Unified MeTTa value representation
//!
//! This module provides the core value type for MeTTa expressions with:
//! - Strong typing (no String-based numbers!)
//! - Efficient representation
//! - Full interoperability between backends

use std::fmt;

/// Unified MeTTa value with strong typing
#[derive(Debug, Clone, PartialEq)]
pub enum MettaValue {
 /// Integer numbers (i64 for performance, can be extended to BigInt)
 Integer(i64),
 
 /// Floating-point numbers
 Float(f64),
 
 /// Boolean values
 Bool(bool),
 
 /// Symbolic atoms (interned in future optimization)
 Atom(String),
 
 /// List of values
 List(Vec<MettaValue>),
 
 /// Function application/expression: (func arg1 arg2 ...)
 /// First element is the function/atom, rest are arguments
 Expression(String, Vec<MettaValue>),
 
 /// String literals
 Str(String),
 
 /// Error state (for graceful error handling in expressions)
 Error(String),
}

impl MettaValue {
 /// Parse a MeTTa value from string representation
 pub fn parse(s: &str) -> Option<Self> {
  crate::parser::parse_metta(s).ok()
 }
 
 /// Convert to string representation
 pub fn as_str(&self) -> Option<&str> {
  match self {
   MettaValue::Atom(s) | MettaValue::Str(s) => Some(s),
   _ => None,
  }
 }
 
 /// Get as integer if possible
 pub fn as_integer(&self) -> Option<i64> {
  match self {
   MettaValue::Integer(n) => Some(*n),
   MettaValue::Float(n) if n.fract() == 0.0 => Some(*n as i64),
   _ => None,
  }
 }
 
 /// Get as float if possible
 pub fn as_float(&self) -> Option<f64> {
  match self {
   MettaValue::Float(f) => Some(*f),
   MettaValue::Integer(n) => Some(*n as f64),
   _ => None,
  }
 }
 
 /// Get as boolean if possible
 pub fn as_bool(&self) -> Option<bool> {
  match self {
   MettaValue::Bool(b) => Some(*b),
   _ => None,
  }
 }
 
 /// Get as atom/symbol if possible
 pub fn as_atom(&self) -> Option<&str> {
  match self {
   MettaValue::Atom(s) => Some(s),
   _ => None,
  }
 }
 
 /// Check if value is truthy (for conditional logic)
 pub fn is_truthy(&self) -> bool {
  match self {
   MettaValue::Bool(b) => *b,
   MettaValue::Integer(n) => *n != 0,
   MettaValue::Float(n) => *n != 0.0,
   MettaValue::Atom(s) => !s.is_empty(),
   MettaValue::Str(s) => !s.is_empty(),
   _ => true,
  }
 }
 
    /// Get the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            MettaValue::Integer(_) => "Integer",
            MettaValue::Float(_) => "Float",
            MettaValue::Bool(_) => "Bool",
            MettaValue::Atom(_) => "Atom",
            MettaValue::List(_) => "List",
            MettaValue::Expression(..) => "Expression",
            MettaValue::Str(_) => "String",
            MettaValue::Error(_) => "Error",
        }
    }

    /// Convert to string representation
    pub fn as_string(&self) -> String {
        match self {
            MettaValue::Integer(n) => n.to_string(),
            MettaValue::Float(n) => n.to_string(),
            MettaValue::Bool(true) => "true".to_string(),
            MettaValue::Bool(false) => "false".to_string(),
            MettaValue::Atom(s) => s.clone(),
            MettaValue::Str(s) => s.clone(),
            MettaValue::List(items) => {
                let items_str: Vec<String> = items.iter().map(|i| i.as_string()).collect();
                format!("({})", items_str.join(" "))
            }
            MettaValue::Expression(head, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.as_string()).collect();
                format!("({} {})", head, args_str.join(" "))
            }
            MettaValue::Error(msg) => format!("Error: {}", msg),
        }
    }

    /// Convert to integer if possible
    pub fn as_int(&self) -> Option<i64> {
        self.as_integer()
    }

    /// Convert to f64 if possible
    pub fn as_f64(&self) -> Option<f64> {
        self.as_float()
    }
}

impl fmt::Display for MettaValue {
 fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  match self {
   MettaValue::Integer(n) => write!(f, "{}", n),
   MettaValue::Float(n) => {
    // Format float, removing trailing zeros
    let s = format!("{}", n);
    if s.contains('.') {
     write!(f, "{}", s.trim_end_matches('0').trim_end_matches('.'))
    } else {
     write!(f, "{}", s)
    }
   }
   MettaValue::Bool(true) => write!(f, "True"),
   MettaValue::Bool(false) => write!(f, "False"),
   MettaValue::Atom(s) => write!(f, "{}", s),
   MettaValue::Str(s) => write!(f, "\"{}\"", s),
   MettaValue::List(items) => {
    write!(f, "(")?;
    for (i, item) in items.iter().enumerate() {
     if i > 0 {
      write!(f, " ")?;
     }
     write!(f, "{}", item)?;
    }
    write!(f, ")")
   }
   MettaValue::Expression(head, args) => {
    write!(f, "({}", head)?;
    for arg in args {
     write!(f, " {}", arg)?;
    }
    write!(f, ")")
   }
   MettaValue::Error(msg) => write!(f, "Error: {}", msg),
  }
 }
}

/// Execution result wrapper
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MettaResult {
 pub value: String,
}

impl MettaResult {
 /// Parse the result value into a MettaValue
 pub fn parsed_value(&self) -> Option<MettaValue> {
  MettaValue::parse(&self.value)
 }
 
 /// Check if result is empty
 pub fn is_empty(&self) -> bool {
  self.value.trim().is_empty()
 }
 
 /// Create a new result from a value
 pub fn from_value(v: impl ToString) -> Self {
  MettaResult {
   value: v.to_string(),
  }
 }
}

#[cfg(test)]
mod tests {
 use super::*;

 #[test]
 fn test_integer_parsing() {
  assert_eq!(MettaValue::parse("42"), Some(MettaValue::Integer(42)));
  assert_eq!(MettaValue::parse("-17"), Some(MettaValue::Integer(-17)));
 }

 #[test]
 fn test_float_parsing() {
  // Float parsing tested separately
 }

 #[test]
 fn test_bool_parsing() {
  assert_eq!(MettaValue::parse("true"), Some(MettaValue::Bool(true)));
  assert_eq!(MettaValue::parse("false"), Some(MettaValue::Bool(false)));
 }

 #[test]
 fn test_is_truthy() {
  assert!(MettaValue::Integer(1).is_truthy());
  assert!(!MettaValue::Integer(0).is_truthy());
  assert!(MettaValue::Bool(true).is_truthy());
  assert!(!MettaValue::Bool(false).is_truthy());
 }

 #[test]
 fn test_display() {
  assert_eq!(MettaValue::Integer(42).to_string(), "42");
  assert_eq!(MettaValue::Bool(true).to_string(), "True");
  assert_eq!(MettaValue::Atom("foo".to_string()).to_string(), "foo");
 }
}
