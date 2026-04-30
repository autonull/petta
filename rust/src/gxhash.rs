//! GXHash wrapper - provides fast hashing when available
//!
//! Re-exports the gxhash crate when the `fast-hasher` feature is enabled.

#[cfg(all(not(any(miri, target_arch = "riscv64")), feature = "fast-hasher"))]
pub use gxhash::{gxhash, gxhash_with_seed, GxHasher};
