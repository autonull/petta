//! Ring operation implementations
//!
//! This module provides concrete implementations of ring operations.

/// Perform ring concatenation
pub fn concat<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    [a, b].concat()
}

/// Perform ring reversal
pub fn reverse<T: Clone>(path: &[T]) -> Vec<T> {
    path.iter().rev().cloned().collect()
}

/// Rotate path left
pub fn rotate_left<T: Clone>(path: &[T], positions: usize) -> Vec<T> {
    if path.is_empty() {
        return Vec::new();
    }
    let len = path.len();
    let shift = positions % len;
    if shift == 0 {
        path.to_vec()
    } else {
        let mut result = Vec::with_capacity(len);
        result.extend_from_slice(&path[shift..]);
        result.extend_from_slice(&path[..shift]);
        result
    }
}
