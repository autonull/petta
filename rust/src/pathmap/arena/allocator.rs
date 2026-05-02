//! Arena allocator implementations
//!
//! This module provides various arena allocation strategies.

/// Simple bump allocator
pub struct BumpAllocator {
    current: usize,
    capacity: usize,
}

impl BumpAllocator {
    /// Create new bump allocator
    pub fn new(capacity: usize) -> Self {
        Self {
            current: 0,
            capacity,
        }
    }
    
    /// Allocate bytes
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if self.current + size <= self.capacity {
            let offset = self.current;
            self.current += size;
            Some(offset)
        } else {
            None
        }
    }
    
    /// Reset allocator
    pub fn reset(&mut self) {
        self.current = 0;
    }
}

impl Default for BumpAllocator {
    fn default() -> Self {
        Self::new(4096)
    }
}
