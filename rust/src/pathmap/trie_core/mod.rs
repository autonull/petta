//! Core trie types and the PathMap struct

pub(crate) mod node;
pub(crate) mod dense_byte;
pub(crate) mod empty;
pub(crate) mod line_list;
pub(crate) mod tiny;
#[cfg(feature = "bridge_nodes")]
pub(crate) mod bridge;

pub(crate) mod map;
pub(crate) mod r#ref;

// Re-export the main types
pub use map::PathMap;
// Internal types re-exported for crate-internal use
pub(crate) use node::{TrieNode, TrieNodeODRc, TaggedNodeRef, AbstractNodeRef, TrieNodeDowncast};
pub(crate) use r#ref::{TrieRef, TrieRefBorrowed, TrieRefOwned};
pub(crate) use node::PayloadRef;
// Re-export node types that may be needed internally
pub(crate) use dense_byte::{DenseByteNode, ByteNode, CellByteNode};
pub(crate) use line_list::LineListNode;
pub(crate) use empty::EmptyNode;
pub(crate) use tiny::TinyRefNode;
#[cfg(feature = "bridge_nodes")]
pub(crate) use bridge::BridgeNode;
