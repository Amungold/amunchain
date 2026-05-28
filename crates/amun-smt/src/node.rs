//! SMT node types.
//!
//! # Invariants (enforced by tree, verified by validator)
//!
//! 1. **No empty children**: A `Branch` MUST have both children non-empty.
//! 2. **Maximal skip**: `skip_len` equals the longest common prefix of all
//!    keys in the left and right subtrees.
//! 3. **Depth bound**: For every branch at depth `d`, `d + skip_len < 256`.
//! 4. **Prefix consistency**: `prefix` bits exactly match the common prefix
//!    of all leaf keys in the subtree.
//! 5. **Lexicographic partition**: All keys in left subtree < all keys in right.

use crate::hash::Hash;
use once_cell::sync::Lazy;

/// Canonical hash of an empty subtree — computed once.
pub static EMPTY_NODE_HASH: Lazy<Hash> = Lazy::new(crate::hash::empty_node_hash);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Leaf {
        key_hash: [u8; 32],
        value_hash: [u8; 32],
        version: u64,
    },
    Branch {
        /// Number of common prefix bits shared by all leaves in this subtree.
        skip_len: u8,
        /// The common prefix bits, packed big-endian.
        /// Only the first `skip_len` bits are meaningful; trailing bits are zeroed.
        prefix: [u8; 32],
        left: Hash,
        right: Hash,
    },
}

impl Node {
    /// Compute the structural hash of this node.
    pub fn hash(&self) -> Hash {
        match self {
            Node::Leaf { key_hash, value_hash, version } => {
                crate::hash::hash_leaf(key_hash, value_hash, *version)
            }
            Node::Branch { skip_len, prefix, left, right } => {
                let mut pref = *prefix;
                crate::hash::canonicalize_prefix(&mut pref, *skip_len);
                crate::hash::hash_branch(*skip_len, &pref, left, right)
            }
        }
    }

    /// Return the key hash if this is a leaf.
    pub fn leaf_key_hash(&self) -> Option<&[u8; 32]> {
        match self {
            Node::Leaf { key_hash, .. } => Some(key_hash),
            _ => None,
        }
    }
}
