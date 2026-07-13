// N120 — On-Chain Slashing Ledger Root
// =====================================
// Computes a Merkle root from the SlashingLedger's executed certificates,
// enabling the root to be committed in block headers for consensus
// verification.

use crate::slashing_ledger::ExecutedSlash;
use blake3::Hasher;

/// N120.1: Leaf hash for a single executed slash.
fn leaf_hash(slash: &ExecutedSlash) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(b"AMUN_SLASH_LEAF_V1");
    hasher.update(&slash.certificate_id);
    hasher.update(&slash.validator_id);
    hasher.update(&slash.amount.to_le_bytes());
    hasher.update(&slash.height.to_le_bytes());
    hasher.update(&slash.timestamp.to_le_bytes());
    hasher.finalize().into()
}

/// N120.1: Compute the Merkle root from a slice of executed slashes.
/// Uses a simple binary Merkle tree with blake3.
/// Returns [0u8; 32] for an empty ledger.
pub fn merkle_root(slashes: &[ExecutedSlash]) -> [u8; 32] {
    if slashes.is_empty() {
        return [0u8; 32];
    }

    let mut leaves: Vec<[u8; 32]> = slashes.iter().map(leaf_hash).collect();

    while leaves.len() > 1 {
        let mut next = Vec::new();
        for chunk in leaves.chunks(2) {
            let mut hasher = Hasher::new();
            hasher.update(b"AMUN_SLASH_BRANCH_V1");
            hasher.update(&chunk[0]);
            if chunk.len() == 2 {
                hasher.update(&chunk[1]);
            } else {
                hasher.update(&chunk[0]); // duplicate for odd
            }
            next.push(hasher.finalize().into());
        }
        leaves = next;
    }

    leaves[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slashing_ledger::ExecutedSlash;

    fn make_slash(id: u8, height: u64) -> ExecutedSlash {
        ExecutedSlash {
            certificate_id: [id; 32],
            validator_id: [0x42; 32],
            amount: 1000 * id as u64,
            height,
            timestamp: 1000 * height,
        }
    }

    #[test]
    fn n120_1_empty_ledger_gives_zero_root() {
        assert_eq!(merkle_root(&[]), [0u8; 32]);
    }

    #[test]
    fn n120_1_single_leaf_is_deterministic() {
        let slash = make_slash(1, 100);
        let root1 = merkle_root(std::slice::from_ref(&slash));
        let root2 = merkle_root(&[slash]);
        assert_eq!(root1, root2, "N120.1 FAIL: root must be deterministic");
        assert_ne!(
            root1, [0u8; 32],
            "N120.1 FAIL: single leaf root must not be zero"
        );
    }

    #[test]
    fn n120_1_same_order_same_root() {
        // Note: order matters for Merkle trees. Same set, same order = same root.
        let s1 = make_slash(1, 100);
        let s2 = make_slash(2, 200);
        let root1 = merkle_root(&[s1.clone(), s2.clone()]);
        let root2 = merkle_root(&[s1, s2]);
        assert_eq!(
            root1, root2,
            "N120.1 FAIL: identical inputs must produce identical roots"
        );
    }

    #[test]
    fn n120_1_different_slashes_different_root() {
        let s1 = make_slash(1, 100);
        let s2 = make_slash(2, 100);
        let root1 = merkle_root(std::slice::from_ref(&s1));
        let root2 = merkle_root(&[s2]);
        assert_ne!(
            root1, root2,
            "N120.1 FAIL: different slashes must have different roots"
        );
    }

    #[test]
    fn n120_1_root_changes_with_new_slash() {
        let s1 = make_slash(1, 100);
        let s2 = make_slash(2, 200);
        let root_before = merkle_root(std::slice::from_ref(&s1));
        let root_after = merkle_root(&[s1, s2]);
        assert_ne!(
            root_before, root_after,
            "N120.1 FAIL: root must change when new slash is added"
        );
    }

    #[test]
    fn n120_1_larger_tree_is_deterministic() {
        let slashes: Vec<_> = (1..=7).map(|i| make_slash(i, i as u64 * 100)).collect();
        let root1 = merkle_root(&slashes);
        let root2 = merkle_root(&slashes);
        assert_eq!(
            root1, root2,
            "N120.1 FAIL: larger tree must be deterministic"
        );
    }

    #[test]
    fn n120_1_order_affects_root() {
        let s1 = make_slash(1, 100);
        let s2 = make_slash(2, 200);
        let root_ab = merkle_root(&[s1.clone(), s2.clone()]);
        let root_ba = merkle_root(&[s2, s1]);
        assert_ne!(
            root_ab, root_ba,
            "N120.1 FAIL: Merkle root must be order-sensitive (A,B) != (B,A)"
        );
    }
}
