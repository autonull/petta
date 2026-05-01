//! MeTTa value types
//!
//! This module provides the core value representations used throughout PeTTa,
//! including MettaValue and MettaResult types.

/// A MeTTa value
///
/// `MettaValue` represents a single value in the MeTTa language,
/// which can be a string, number, symbol, or expression.
#[derive(Debug, Clone, PartialEq)]
pub struct MettaValue {
    /// Internal representation
    inner: MettaValueInner,
}

#[derive(Debug, Clone, PartialEq)]
enum MettaValueInner {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Expression(String),
}

impl MettaValue {
    /// Create a new string value
    pub fn string<S: Into<String>>(s: S) -> Self {
        Self {
            inner: MettaValueInner::String(s.into()),
        }
    }

    /// Create a new integer value
    pub fn int(i: i64) -> Self {
        Self {
            inner: MettaValueInner::Int(i),
        }
    }

    /// Create a new float value
    pub fn float(f: f64) -> Self {
        Self {
            inner: MettaValueInner::Float(f),
        }
    }

    /// Create a new boolean value
    pub fn bool(b: bool) -> Self {
        Self {
            inner: MettaValueInner::Bool(b),
        }
    }

    /// Create a new expression value
    pub fn expression<S: Into<String>>(expr: S) -> Self {
        Self {
            inner: MettaValueInner::Expression(expr.into()),
        }
    }

    /// Check if value is a string
    pub fn is_string(&self) -> bool {
        matches!(self.inner, MettaValueInner::String(_))
    }

    /// Check if value is an integer
    pub fn is_int(&self) -> bool {
        matches!(self.inner, MettaValueInner::Int(_))
    }

    /// Check if value is a float
    pub fn is_float(&self) -> bool {
        matches!(self.inner, MettaValueInner::Float(_))
    }

    /// Check if value is a boolean
    pub fn is_bool(&self) -> bool {
        matches!(self.inner, MettaValueInner::Bool(_))
    }

    /// Get as string
    pub fn as_string(&self) -> Option<&str> {
        match &self.inner {
            MettaValueInner::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as integer
    pub fn as_int(&self) -> Option<i64> {
        match &self.inner {
            MettaValueInner::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Get as float
    pub fn as_float(&self) -> Option<f64> {
        match &self.inner {
            MettaValueInner::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Get as boolean
    pub fn as_bool(&self) -> Option<bool> {
        match &self.inner {
            MettaValueInner::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match &self.inner {
            MettaValueInner::String(s) => s.clone(),
            MettaValueInner::Int(i) => i.to_string(),
            MettaValueInner::Float(f) => f.to_string(),
            MettaValueInner::Bool(b) => b.to_string(),
            MettaValueInner::Expression(e) => e.clone(),
        }
    }
}

impl From<String> for MettaValue {
    fn from(s: String) -> Self {
        Self::string(s)
    }
}

impl From<&str> for MettaValue {
    fn from(s: &str) -> Self {
        Self::string(s)
    }
}

impl From<i64> for MettaValue {
    fn from(i: i64) -> Self {
        Self::int(i)
    }
}

impl From<i32> for MettaValue {
    fn from(i: i32) -> Self {
        Self::int(i as i64)
    }
}

impl From<f64> for MettaValue {
    fn from(f: f64) -> Self {
        Self::float(f)
    }
}

impl From<bool> for MettaValue {
    fn from(b: bool) -> Self {
        Self::bool(b)
    }
}

impl std::fmt::Display for MettaValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            MettaValueInner::String(s) => write!(f, "{}", s),
            MettaValueInner::Int(i) => write!(f, "{}", i),
            MettaValueInner::Float(fval) => write!(f, "{}", fval),
            MettaValueInner::Bool(b) => write!(f, "{}", b),
            MettaValueInner::Expression(e) => write!(f, "{}", e),
        }
    }
}

/// Result of MeTTa execution
#[derive(Debug, Clone)]
pub struct MettaResult {
    /// The value
    pub value: MettaValue,
}

impl MettaResult {
    /// Create new result from value
    pub fn new<V: Into<MettaValue>>(value: V) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Get the value
    pub fn value(&self) -> &MettaValue {
        &self.value
    }

    /// Convert to owned value
    pub fn into_value(self) -> MettaValue {
        self.value
    }
}

impl From<MettaValue> for MettaResult {
    fn from(value: MettaValue) -> Self {
        Self::new(value)
    }
}

impl From<String> for MettaResult {
    fn from(s: String) -> Self {
        Self::new(MettaValue::string(s))
    }
}

impl From<&str> for MettaResult {
    fn from(s: &str) -> Self {
        Self::new(MettaValue::string(s))
    }
}

impl From<i64> for MettaResult {
    fn from(i: i64) -> Self {
        Self::new(MettaValue::int(i))
    }
}

impl From<f64> for MettaResult {
    fn from(f: f64) -> Self {
        Self::new(MettaValue::float(f))
    }
}

impl From<bool> for MettaResult {
    fn from(b: bool) -> Self {
        Self::new(MettaValue::bool(b))
    }
}
