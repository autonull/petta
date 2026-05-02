//! Core trie implementation for PathMap
//!
//! This module provides the fundamental trie data structure used by PathMap,
//! with optimized operations for path-based lookups and updates.
//!
//! # Architecture
//!
//! The trie module contains:
//! - Core node types and operations
//! - Path traversal algorithms
//! - Memory-efficient representations

// Re-export trie_core as the main trie implementation
pub use crate::pathmap::trie_core::*;

// Additional trie operations and optimizations
mod ops;
pub use ops::*;
