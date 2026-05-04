//! Arena allocation for PathMap
//!
//! This module provides arena-based memory allocation for efficient
//! batch operations and reduced allocation overhead.
//!
//! # Performance Benefits
//!
//! Arena allocation provides:
//! - **Reduced allocations**: Batch allocate nodes instead of individual allocations
//! - **Better cache locality**: Contiguous memory layout
//! - **Faster deallocation**: Drop entire arena at once
//! - **Predictable performance**: No allocation surprises during hot paths
//!
//! # Usage
//!
//! ```rust
//! use crate::pathmap::arena::ArenaAllocator;
//!
//! let mut arena = ArenaAllocator::new(1024);
//! let handle = arena.allocate(MyType::new());
//! let item = arena.get(handle).unwrap();
//! ```

mod allocator;
pub use allocator::*;

use std::sync::Arc;

/// Arena allocator for trie nodes with automatic growth
///
/// This arena starts with a pre-allocated capacity but can grow if needed.
/// For best performance, size the arena appropriately for your workload.
pub struct Arena<T> {
    data: Vec<T>,
    initial_capacity: usize,
    allocated_bytes: usize,
}

impl<T> Arena<T> {
    /// Create new arena with initial capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            initial_capacity: capacity,
            allocated_bytes: 0,
        }
    }

    /// Allocate space for a node
    ///
    /// Returns the index where the value was stored.
    /// The arena will grow automatically if needed.
    #[inline]
    pub fn allocate(&mut self, value: T) -> usize {
        let index = self.data.len();
        self.allocated_bytes += std::mem::size_of::<T>();
        self.data.push(value);
        index
    }

    /// Allocate multiple items from an iterator
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, items: I) -> std::ops::Range<usize> {
        let start = self.data.len();
        let count = items.into_iter().fold(0, |count, item| {
            self.allocated_bytes += std::mem::size_of::<T>();
            self.data.push(item);
            count + 1
        });
        start..start + count
    }

    /// Get reference to allocated item by index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Get mutable reference to allocated item by index
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Get allocated count
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if arena is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get capacity
    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserve additional capacity
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Reset arena to reuse memory
    pub fn clear(&mut self) {
        self.data.clear();
        self.allocated_bytes = 0;
    }

    /// Get memory usage statistics
    pub fn stats(&self) -> ArenaStats {
        ArenaStats {
            allocated: self.data.len(),
            capacity: self.data.capacity(),
            memory_bytes: self.allocated_bytes,
            actual_bytes: self.data.len() * std::mem::size_of::<T>(),
        }
    }
}

/// Arena memory statistics
#[derive(Debug, Clone, Copy)]
pub struct ArenaStats {
    pub allocated: usize,
    pub capacity: usize,
    pub memory_bytes: usize,
    pub actual_bytes: usize,
}

impl ArenaStats {
    /// Get memory utilization percentage
    #[inline]
    pub fn utilization(&self) -> f64 {
        if self.memory_bytes == 0 {
            0.0
        } else {
            self.actual_bytes as f64 / self.memory_bytes as f64
        }
    }

    /// Get waste percentage (allocated but not used)
    #[inline]
    pub fn waste(&self) -> f64 {
        1.0 - self.utilization()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new(1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_basic() {
        let mut arena = Arena::<i32>::new(10);
        let idx1 = arena.allocate(42);
        let idx2 = arena.allocate(100);
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);
        assert_eq!(arena.get(idx1), Some(&42));
        assert_eq!(arena.get(idx2), Some(&100));
        assert_eq!(arena.len(), 2);
    }

    #[test]
    fn test_arena_grows() {
        let mut arena = Arena::<i32>::new(2);
        for i in 0..10 {
            arena.allocate(i);
        }
        assert_eq!(arena.len(), 10);
    }

    #[test]
    fn test_arena_clear() {
        let mut arena = Arena::<i32>::new(10);
        for i in 0..5 {
            arena.allocate(i);
        }
        arena.clear();
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.capacity(), 10);
    }

    #[test]
    fn test_arena_stats() {
        let mut arena = Arena::<i32>::new(10);
        arena.allocate(1);
        arena.allocate(2);
        arena.allocate(3);
        
        let stats = arena.stats();
        assert_eq!(stats.allocated, 3);
        assert_eq!(stats.actual_bytes, 3 * std::mem::size_of::<i32>());
    }

    #[test]
    fn test_arena_extend() {
        let mut arena = Arena::<i32>::new(10);
        let range = arena.extend(vec![1, 2, 3, 4, 5]);
        assert_eq!(range, 0..5);
        assert_eq!(arena.len(), 5);
    }
}
