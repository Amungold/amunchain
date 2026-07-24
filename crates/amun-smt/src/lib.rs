//! Sparse Merkle Tree for State Commitment (ADR-029)
//!
//! 256-bit key space. Domain-separated leaf and node hashing.

// ============================================================================
// Protocol Constants
// ============================================================================

pub const STATE_LEAF_DOMAIN: &[u8] = b"AMUN_STATE_LEAF_V1";
pub const STATE_NODE_DOMAIN: &[u8] = b"AMUN_STATE_NODE_V1";
pub const EMPTY_HASH: [u8; 32] = [0u8; 32];

// ============================================================================
// Core SMT
// ============================================================================

pub struct SparseMerkleTree {
    root: [u8; 32],
    leaves: std::collections::BTreeMap<[u8; 32], [u8; 32]>, // key → value_hash
}

impl Default for SparseMerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

impl SparseMerkleTree {
    pub fn new() -> Self {
        Self {
            root: EMPTY_HASH,
            leaves: std::collections::BTreeMap::new(),
        }
    }

    pub fn root(&self) -> [u8; 32] {
        self.root
    }

    pub fn is_empty(&self) -> bool {
        self.leaves.is_empty()
    }

    pub fn insert(&mut self, key: [u8; 32], value_hash: [u8; 32]) {
        self.leaves.insert(key, value_hash);
        self.root = Self::compute_root_from_leaves(&self.leaves);
    }

    pub fn remove(&mut self, key: &[u8; 32]) {
        self.leaves.remove(key);
        self.root = Self::compute_root_from_leaves(&self.leaves);
    }

    fn compute_root_from_leaves(
        leaves: &std::collections::BTreeMap<[u8; 32], [u8; 32]>,
    ) -> [u8; 32] {
        if leaves.is_empty() {
            return EMPTY_HASH;
        }

        // Collect all leaf hashes at their positions
        let mut current_level: Vec<([u8; 32], [u8; 32])> =
            leaves.iter().map(|(k, v)| (*k, leaf_hash(k, v))).collect();

        // Build tree bottom-up, merging adjacent keys
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            let mut i = 0;
            while i < current_level.len() {
                if i + 1 < current_level.len() {
                    let (_, left_hash) = current_level[i];
                    let (_, right_hash) = current_level[i + 1];
                    // Use the higher key as parent key
                    let parent_key = current_level[i + 1].0;
                    next_level.push((parent_key, node_hash(&left_hash, &right_hash)));
                    i += 2;
                } else {
                    // Odd node — promote to next level
                    next_level.push(current_level[i]);
                    i += 1;
                }
            }
            current_level = next_level;
        }

        current_level[0].1
    }
}

// ============================================================================
// Hash Functions
// ============================================================================

fn leaf_hash(key: &[u8; 32], value_hash: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(STATE_LEAF_DOMAIN);
    h.update(key);
    h.update(value_hash);
    h.finalize().into()
}

fn node_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(STATE_NODE_DOMAIN);
    h.update(left);
    h.update(right);
    h.finalize().into()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree_root() {
        let tree = SparseMerkleTree::new();
        assert_eq!(tree.root(), EMPTY_HASH);
    }

    #[test]
    fn test_single_leaf() {
        let mut tree = SparseMerkleTree::new();
        tree.insert([1u8; 32], [42u8; 32]);
        assert_ne!(tree.root(), EMPTY_HASH);
    }

    #[test]
    fn test_deterministic() {
        let mut t1 = SparseMerkleTree::new();
        let mut t2 = SparseMerkleTree::new();
        t1.insert([1u8; 32], [10u8; 32]);
        t1.insert([2u8; 32], [20u8; 32]);
        t2.insert([1u8; 32], [10u8; 32]);
        t2.insert([2u8; 32], [20u8; 32]);
        assert_eq!(t1.root(), t2.root());
    }

    #[test]
    fn test_root_changes_with_value() {
        let mut t1 = SparseMerkleTree::new();
        let mut t2 = SparseMerkleTree::new();
        t1.insert([1u8; 32], [10u8; 32]);
        t2.insert([1u8; 32], [99u8; 32]);
        assert_ne!(t1.root(), t2.root());
    }

    #[test]
    fn test_remove() {
        let mut tree = SparseMerkleTree::new();
        tree.insert([1u8; 32], [10u8; 32]);
        tree.remove(&[1u8; 32]);
        assert_eq!(tree.root(), EMPTY_HASH);
    }

    #[test]
    fn test_multiple_leaves() {
        let mut tree = SparseMerkleTree::new();
        for i in 0..10u8 {
            tree.insert([i; 32], [i; 32]);
        }
        assert_ne!(tree.root(), EMPTY_HASH);
        // Deterministic
        let mut tree2 = SparseMerkleTree::new();
        for i in 0..10u8 {
            tree2.insert([i; 32], [i; 32]);
        }
        assert_eq!(tree.root(), tree2.root());
    }
}
