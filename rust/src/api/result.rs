//! Execution result types
//!
//! This module provides structured types for execution results,
//! offering rich information about MeTTa execution outcomes.

use std::time::Duration;
use crate::values::MettaValue;

// Re-export MettaResult from values module
pub use crate::values::MettaResult;

/// Warning with optional suggestion
#[derive(Debug, Clone)]
pub struct Warning {
    /// Warning message
    pub message: String,
    /// Optional suggestion for fixing the issue
    pub suggestion: Option<String>,
}

impl Warning {
    /// Create a new warning
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            suggestion: None,
        }
    }

    /// Add a suggestion to the warning
    pub fn with_suggestion<S: Into<String>>(mut self, suggestion: S) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

/// Structured execution result
///
/// `ExecutionResult` wraps the results of MeTTa execution with
/// additional metadata and convenience methods.
#[derive(Debug)]
pub struct ExecutionResult {
    values: Vec<MettaValue>,
    stats: ExecutionStats,
    warnings: Vec<Warning>,
}

impl ExecutionResult {
    /// Create new execution result from raw results
    pub fn from_results(results: Vec<MettaResult>) -> Self {
        let values: Vec<MettaValue> = results.into_iter()
            .filter_map(|r| MettaValue::parse(&r.value))
            .collect();
        Self {
            stats: ExecutionStats::default(),
            values,
            warnings: Vec::new(),
        }
    }

    /// Create execution result with custom stats and warnings
    pub fn new(values: Vec<MettaValue>, stats: ExecutionStats, warnings: Vec<Warning>) -> Self {
        Self {
            values,
            stats,
            warnings,
        }
    }

    /// Get first result value
    pub fn first(&self) -> Option<&MettaValue> {
        self.values.first()
    }

    /// Get first result as string
    pub fn first_as_string(&self) -> Option<String> {
        self.values.first().and_then(|v| {
            v.as_str().map(|s| s.to_string())
        })
    }

    /// Get first result as integer
    pub fn first_as_int(&self) -> Option<i64> {
        self.values.first().and_then(|v| v.as_integer())
    }

    /// Get first result as float
    pub fn first_as_float(&self) -> Option<f64> {
        self.values.first().and_then(|v| v.as_float())
    }

    /// Get first result as boolean
    pub fn first_as_bool(&self) -> Option<bool> {
        self.values.first().and_then(|v| v.as_bool())
    }

    /// Get all result values
    pub fn values(&self) -> &[MettaValue] {
        &self.values
    }

    /// Get execution statistics
    pub fn stats(&self) -> &ExecutionStats {
        &self.stats
    }

    /// Get warnings
    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }

    /// Get number of results
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if result is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Convert to Vec of values
    pub fn into_values(self) -> Vec<MettaValue> {
        self.values
    }

    /// Add a warning to the result
    pub fn add_warning(&mut self, warning: Warning) {
        self.warnings.push(warning);
    }

    /// Add multiple warnings
    pub fn extend_warnings(&mut self, warnings: Vec<Warning>) {
        self.warnings.extend(warnings);
    }

    /// Set execution stats
    pub fn with_stats(mut self, stats: ExecutionStats) -> Self {
        self.stats = stats;
        self
    }

    /// Set execution stats (mutable version)
    pub fn set_stats(&mut self, stats: ExecutionStats) {
        self.stats = stats;
    }
}

impl From<ExecutionResult> for Vec<MettaValue> {
    fn from(result: ExecutionResult) -> Self {
        result.values
    }
}

/// Execution statistics
#[derive(Debug, Clone, Default)]
pub struct ExecutionStats {
    /// Execution duration
    pub duration: Duration,
    /// Number of reductions performed
    pub reductions: usize,
    /// Number of allocations
    pub allocations: usize,
}

impl ExecutionStats {
    /// Create new stats with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create stats with explicit values
    pub fn with_values(duration: Duration, reductions: usize, allocations: usize) -> Self {
        Self {
            duration,
            reductions,
            allocations,
        }
    }
}
