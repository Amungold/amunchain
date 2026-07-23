//! AmunChain Merkle Tree
//!
//! ADR-026: Transactions Root
//!
//! This crate is the single source of truth for all Merkle tree
//! computations in the AmunChain protocol.

use std::sync::LazyLock;

// ============================================================================
// Protocol Constants (ADR-026 §4.3)
// ============================================================================

pub const TX_LEAF_DOMAIN: &[u8] = b"AMUN_TX_LEAF_V1";
pub const TX_NODE_DOMAIN: &[u8] = b"AMUN_TX_NODE_V1";

/// Pre-computed empty transactions root (internal).
static EMPTY_TX_ROOT: LazyLock<[u8; 32]> = LazyLock::new(|| {
    let hash = blake3::hash(b"AMUN_EMPTY_TX_ROOT_V1");
    *hash.as_bytes()
});

/// Return the protocol-defined empty transactions root.
pub fn empty_tx_root() -> [u8; 32] {
    *EMPTY_TX_ROOT
}

// ============================================================================
// Core Merkle Functions
// ============================================================================

#[inline]
fn leaf_hash(leaf: &[u8; 32], domain: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(domain);
    h.update(leaf);
    h.finalize().into()
}

#[inline]
fn node_hash(left: &[u8; 32], right: &[u8; 32], domain: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(domain);
    h.update(left);
    h.update(right);
    h.finalize().into()
}

/// Compute a Merkle root from a slice of 32-byte leaf hashes.
///
/// # Rules (ADR-026 §4.6)
/// - Empty leaves: returns `empty_root`
/// - Single leaf: returns `leaf_hash(leaf, leaf_domain)`
/// - Odd number of nodes at any level: last node is duplicated
///
/// # Determinism (ADR-026 §4.2)
/// Fully deterministic across all platforms.
pub fn merkle_root(
    leaves: &[[u8; 32]],
    leaf_domain: &[u8],
    node_domain: &[u8],
    empty_root: [u8; 32],
) -> [u8; 32] {
    match leaves.len() {
        0 => return empty_root,
        1 => return leaf_hash(&leaves[0], leaf_domain),
        _ => {}
    }

    let mut current: Vec<[u8; 32]> = leaves
        .iter()
        .map(|leaf| leaf_hash(leaf, leaf_domain))
        .collect();

    while current.len() > 1 {
        if current.len() % 2 != 0 {
            current.push(current[current.len() - 1]);
        }
        let mut next = Vec::with_capacity(current.len() / 2);
        for chunk in current.chunks_exact(2) {
            next.push(node_hash(&chunk[0], &chunk[1], node_domain));
        }
        current = next;
    }

    current[0]
}

// ============================================================================
// Domain-Specific Helpers
// ============================================================================

pub fn transactions_root(tx_hashes: &[[u8; 32]]) -> [u8; 32] {
    merkle_root(tx_hashes, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_tx_root())
}

// ============================================================================
// Tests (ADR-026 §7)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_root() -> [u8; 32] {
        empty_tx_root()
    }

    #[test]
    fn test_merkle_empty() {
        let root = merkle_root(&[], TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        assert_eq!(root, empty_root());
    }

    #[test]
    fn test_merkle_single_leaf() {
        let leaf = [1u8; 32];
        let root = merkle_root(&[leaf], TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        let expected = leaf_hash(&leaf, TX_LEAF_DOMAIN);
        assert_eq!(root, expected);
    }

    #[test]
    fn test_merkle_determinism() {
        let leaves = [[1u8; 32], [2u8; 32], [3u8; 32]];
        let r1 = merkle_root(&leaves, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        let r2 = merkle_root(&leaves, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_merkle_order_sensitive() {
        let a = [[1u8; 32], [2u8; 32]];
        let b = [[2u8; 32], [1u8; 32]];
        let ra = merkle_root(&a, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        let rb = merkle_root(&b, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        assert_ne!(ra, rb);
    }

    #[test]
    fn test_merkle_odd_leaves() {
        let leaves = [[1u8; 32], [2u8; 32], [3u8; 32]];
        let root = merkle_root(&leaves, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        assert_ne!(root, [0u8; 32]);
        let root2 = merkle_root(&leaves, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        assert_eq!(root, root2);
    }

    #[test]
    fn test_merkle_power_of_two() {
        let leaves = [[1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32]];
        let root = merkle_root(&leaves, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        assert_ne!(root, [0u8; 32]);
    }

    #[test]
    fn test_transactions_root_wrapper() {
        let tx_hashes = [[1u8; 32], [2u8; 32]];
        let direct = merkle_root(&tx_hashes, TX_LEAF_DOMAIN, TX_NODE_DOMAIN, empty_root());
        let wrapped = transactions_root(&tx_hashes);
        assert_eq!(direct, wrapped);
    }
}
