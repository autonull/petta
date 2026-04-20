//! Fallback hasher using xxhash-rust, replacing gxhash.

pub use std::collections::{HashMap, HashSet};

pub use xxhash_rust::xxh3::{Xxh3Builder as XxBuildHasher, Xxh3 as XxHasher};

#[allow(dead_code)]
pub fn xxhash128(data: &[u8], seed: i64) -> u128 {
    xxhash_rust::xxh3::xxh3_128_with_seed(data, seed as u64)
}

#[allow(dead_code)]
pub trait HashMapExt: Sized {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
}

#[allow(dead_code)]
pub trait HashSetExt: Sized {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
}

impl<K, V> HashMapExt for HashMap<K, V, XxBuildHasher> {
    fn new() -> Self {
        HashMap::with_hasher(XxBuildHasher::default())
    }
    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity_and_hasher(capacity, XxBuildHasher::default())
    }
}

impl<T> HashSetExt for HashSet<T, XxBuildHasher> {
    fn new() -> Self {
        HashSet::with_hasher(XxBuildHasher::default())
    }
    fn with_capacity(capacity: usize) -> Self {
        HashSet::with_capacity_and_hasher(capacity, XxBuildHasher::default())
    }
}
