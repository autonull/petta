// Crate-level wrapper to provide a stable `gxhash` module regardless of
// whether the external `gxhash` crate is available or we're using the
// local fallback. Many modules reference `gxhash::...` directly, so this
// file re-exports the appropriate implementation.

// If consumers opt into the `fast-hasher` feature, re-export the external
// `gxhash` crate when the platform is suitable. Otherwise default to our
// pure-Rust fallback implementation so builds succeed on all hosts.
#[cfg(all(not(any(miri, target_arch = "riscv64")), feature = "fast-hasher"))]
pub use ::gxhash::*;

#[cfg(not(all(not(any(miri, target_arch = "riscv64")), feature = "fast-hasher")))]
pub use crate::hash_fallback::*;
