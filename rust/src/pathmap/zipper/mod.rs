//! Zipper implementations for PathMap
//!
//! This module provides various zipper implementations for efficient
//! navigation and modification of trie structures.
//!
//! # Zipper Types
//!
//! - **Base Zipper**: Standard zipper for single trie navigation
//! - **Overlay Zipper**: Layered zipper for overlay operations
//! - **Product Zipper**: Zippers for product spaces
//! - **Write Zipper**: Optimized for write-heavy workloads

mod base;
mod overlay;
mod product;
mod write;

pub use base::BaseZipper;
pub use overlay::OverlayZipper;
pub use product::ProductZipper;
pub use write::WriteZipper;

/// Common zipper trait
pub trait Zipper {
    /// Current position path
    fn path(&self) -> &[u8];
    
    /// Move to child
    fn down(&mut self, index: u8) -> bool;
    
    /// Move to parent
    fn up(&mut self) -> bool;
    
    /// Get current value
    fn get(&self) -> Option<&Self::Value>
    where
        Self: Sized;
}
