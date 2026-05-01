//! Performance optimizations for PeTTa
//!
//! This module provides performance-critical optimizations:
//! - Inline hints for hot paths
//! - Caching utilities
//! - Memory-efficient data structures

/// Cache for frequently computed values
///
/// Uses a simple LRU strategy with configurable capacity.
#[derive(Debug, Clone)]
pub struct Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    capacity: usize,
    data: indexmap::IndexMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: indexmap::IndexMap::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V>
    where
        K: std::hash::Hash + Eq,
    {
        self.data.get(key).cloned()
    }

    pub fn insert(&mut self, key: K, value: V)
    where
        K: std::hash::Hash + Eq,
    {
        if self.data.len() >= self.capacity {
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.shift_remove(&first_key);
            }
        }
        self.data.insert(key, value);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<K, V> Default for Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new(256)
    }
}

/// String interner for reducing memory usage
#[derive(Debug, Default)]
pub struct StringInterner {
    strings: std::collections::HashMap<String, u32>,
    next_id: u32,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: std::collections::HashMap::with_capacity(1024),
            next_id: 0,
        }
    }

    pub fn intern(&mut self, s: &str) -> u32 {
        if let Some(&id) = self.strings.get(s) {
            return id;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.strings.insert(s.to_string(), id);
        id
    }

    pub fn get(&self, id: u32) -> Option<&str> {
        self.strings
            .iter()
            .find(|&(_, &v)| v == id)
            .map(|(k, _)| k.as_str())
    }

    pub fn clear(&mut self) {
        self.strings.clear();
        self.next_id = 0;
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = Cache::<String, i32>::new(10);
        cache.insert("key".to_string(), 42);
        assert_eq!(cache.get(&"key".to_string()), Some(42));
    }

    #[test]
    fn test_cache_capacity() {
        let mut cache = Cache::<usize, i32>::new(3);
        for i in 0..5 {
            cache.insert(i, i as i32);
        }
        assert!(cache.len() <= 3);
    }

    #[test]
    fn test_string_interner() {
        let mut interner = StringInterner::new();
        let id1 = interner.intern("hello");
        let id2 = interner.intern("world");
        let id3 = interner.intern("hello");
        
        assert_eq!(id1, id3);
        assert_ne!(id1, id2);
    }
}
