//! Execution result types
//!
//! This module provides structured types for execution results,
//! offering rich information about MeTTa execution outcomes.

use std::time::Duration;
use crate::values::MettaValue;

// Re-export MettaResult from values module
pub use crate::values::MettaResult;

/// Structured execution result
///
/// `ExecutionResult` wraps the results of MeTTa execution with
/// additional metadata and convenience methods.
pub struct ExecutionResult {
    values: Vec<MettaValue>,
    stats: ExecutionStats,
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
        }
    }

    /// Get first result value
    pub fn first(&self) -> Option<&MettaValue> {
        self.values.first()
    }

    /// Get all result values
    pub fn values(&self) -> &[MettaValue] {
        &self.values
    }

    /// Get execution statistics
    pub fn stats(&self) -> &ExecutionStats {
        &self.stats
    }

    /// Get number of results
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if result is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Convert to Vec of values
    pub fn into_values(self) -> Vec<MettaValue> {
        self.values
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
