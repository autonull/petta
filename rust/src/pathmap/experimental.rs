//! Experimental zipper implementations and utilities.
//!
//! This module contains experimental features that may or may not become permanent.

#![allow(unused_variables, unreachable_code, unused_mut, dead_code, unused_imports, clippy::wrong_self_convention)]

use super::PathMap;
use super::TrieValue;
use super::alloc::Allocator;
use super::ring::{AlgebraicStatus, DistributiveLattice, Lattice};
use super::trie_core::node::TrieNodeODRc;
use super::trie_core::node::*;
use super::utils::ByteMask;
use super::write_zipper::write_zipper_priv::WriteZipperPriv;
use super::zipper::*;

#[cfg(feature = "serialization")]
pub mod serialization;
#[cfg(feature = "serialization")]
pub mod tree_serialization;

#[cfg(feature = "zipper_alg")]
pub mod zipper_algebra;
#[cfg(not(feature = "zipper_alg"))]
mod zipper_algebra;
