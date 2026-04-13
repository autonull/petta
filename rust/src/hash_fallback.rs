//! Fallback hasher for environments where gxhash is unavailable (miri, riscv64).
//! Uses a simple XOR-based hash to avoid panics.

/// Simple XOR hasher used when gxhash is unavailable (miri, riscv64).
#[derive(Clone, Default)]
pub struct GxHasher { state_lo: u64, state_hi: u64 }
impl GxHasher {
    pub fn with_seed(seed: i64) -> Self {
        let seed = u64::from_ne_bytes(seed.to_ne_bytes());
        Self { state_lo: seed ^ 0xA5A5A5A5_A5A5A5A5, state_hi: !seed ^ 0x5A5A5A5A_5A5A5A5A }
    }
    pub fn finish_u128(&self) -> u128 {
        ((self.state_hi as u128) << 64) | self.state_lo as u128
    }
}
impl core::hash::Hasher for GxHasher {
    fn write(&mut self, buf: &[u8]) { for &c in buf { self.write_u8(c); } }
    fn write_u8(&mut self, i: u8) {
        self.state_lo = self.state_lo.wrapping_add(i as u64);
        self.state_hi ^= (i as u64).rotate_left(11);
        self.state_lo = self.state_lo.rotate_left(3);
    }
    fn write_u128(&mut self, i: u128) {
        let low = i as u64;
        let high = (i >> 64) as u64;
        self.state_lo = self.state_lo.wrapping_add(low);
        self.state_hi ^= high.rotate_left(17);
        self.state_lo ^= high.rotate_left(9);
    }
    fn finish(&self) -> u64 { self.finish_u128() as u64 }
}

pub use std::collections::{HashMap, HashSet};
pub fn gxhash128(data: &[u8], _seed: i64) -> u128 { xxhash_rust::const_xxh3::xxh3_128(data) }

/// Marker trait for gxhash-compatible HashMaps (fallback).
pub trait HashMapExt {}
/// Marker trait for gxhash-compatible HashSets (fallback).
pub trait HashSetExt {}
