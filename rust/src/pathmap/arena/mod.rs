//! Arena allocation for PathMap
//!
//! This module provides arena-based memory allocation for efficient
//! batch operations and reduced allocation overhead.

mod allocator;
pub use allocator::*;

use std::sync::Arc;

/// Arena allocator for trie nodes
pub struct Arena<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T> Arena<T> {
    /// Create new arena with capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }
    
    /// Allocate space for a node
    pub fn allocate(&mut self, value: T) -> Option<usize> {
        if self.data.len() < self.capacity {
            let index = self.data.len();
            self.data.push(value);
            Some(index)
        } else {
            None
        }
    }
    
    /// Get allocated count
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if arena is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new(1024)
    }
}
