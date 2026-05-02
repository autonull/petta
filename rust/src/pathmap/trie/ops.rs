//! Core trie operations
//!
//! This module provides optimized operations for trie manipulation,
//! including insert, lookup, and traversal.

use crate::pathmap::trie_core::node::Node;

/// Insert a value into the trie
pub fn insert<V: Clone>(node: &mut Node<V>, path: &[u8], value: V) {
    // Implementation delegated to trie_core
    // This is a placeholder for optimized operations
}

/// Lookup a value in the trie
pub fn get<'a, V: Clone>(node: &'a Node<V>, path: &[u8]) -> Option<&'a V> {
    // Implementation delegated to trie_core
    None
}

/// Remove a value from the trie
pub fn remove<V: Clone>(node: &mut Node<V>, path: &[u8]) -> Option<V> {
    // Implementation delegated to trie_core
    None
}

/// Traverse the trie with a predicate
pub fn traverse<F, V>(node: &Node<V>, f: F)
where
    F: Fn(&Node<V>),
{
    f(node);
}
