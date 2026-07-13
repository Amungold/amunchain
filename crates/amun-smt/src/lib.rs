//! # Amun-SMT: Canonical Authenticated Sparse Merkle Tree
//!
//! A consensus-hardened deterministic state core.
//!
//! ## Key Hashing
//!
//! `Key256` is a pre-image. The tree internally hashes it with BLAKE3
//! to produce the 256-bit trie key. External implementations MUST
//! replicate this exactly.
//!
//! ## Modules
//!
//! - [`hash`]: domain-separated hashing
//! - [`node`]: SMT node types and invariants
//! - [`tree`]: the `SparseMerkleTree` type
//! - [`proof`]: Merkle proof types and verification
//! - [`validator`]: structural invariant checker
//! - [`context`]: content-addressed node interner
//! - [`error`]: error types

pub mod hash;
pub mod node;
pub mod tree;
pub mod proof;
pub mod validator;
pub mod canonical_model;
pub mod context;
pub mod error;

pub use tree::{Key256, SparseMerkleTree, StateRoot};
pub use hash::Hash;
pub use node::{Node, EMPTY_NODE_HASH};
pub use proof::{AbsenceReason, LeafWitness, MerkleProof, ProofStep};
pub use validator::validate_tree;
pub use context::Context;
pub use error::SmtError;
