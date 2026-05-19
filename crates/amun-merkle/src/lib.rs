#![no_std]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]

pub mod domain;
pub mod proof;
pub mod tree;

pub use domain::MerkleDomain;
pub use proof::MerkleProof;
pub use tree::MerkleTree;

#[cfg(test)]
mod tests;
