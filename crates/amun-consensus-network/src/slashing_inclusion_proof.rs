// N121.4 — Merkle Inclusion Proofs for Slashing Ledger
// =====================================================
// Enables light clients and validators to verify that a specific
// slash is included in the slashing ledger given only the
// slashing_root and a Merkle proof.

use crate::slashing_ledger::ExecutedSlash;
use blake3::Hasher;

/// N121.4: A Merkle inclusion proof for an executed slash.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct SlashingInclusionProof {
    /// The slash being proved
    pub slash: ExecutedSlash,

    /// Index of the slash in the ledger (0-based)
    pub index: usize,

    /// Total number of slashes in the ledger
    pub total_count: usize,

    /// Merkle proof: sibling hashes from leaf to root
    pub siblings: Vec<[u8; 32]>,

    /// The expected Merkle root
    #[serde(with = "serde_bytes")]
    pub expected_root: [u8; 32],
}

impl SlashingInclusionProof {
    /// N121.4: Compute leaf hash for a slash (same as merkle tree).
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

    /// N121.4: Verify the inclusion proof against the expected root.
    pub fn verify(&self) -> Result<(), String> {
        if self.total_count == 0 {
            return Err("N121.4: empty ledger has no inclusions".into());
        }
        if self.index >= self.total_count {
            return Err(format!(
                "N121.4: index {} out of bounds (total={})",
                self.index, self.total_count
            ));
        }

        // Start from the leaf hash
        let mut current = Self::leaf_hash(&self.slash);

        // Climb the tree using siblings
        let mut idx = self.index;
        let mut total = self.total_count;

        for sibling in &self.siblings {
            let mut hasher = Hasher::new();
            hasher.update(b"AMUN_SLASH_BRANCH_V1");

            if idx.is_multiple_of(2) {
                // Current is left child
                hasher.update(&current);
                hasher.update(sibling);
            } else {
                // Current is right child
                hasher.update(sibling);
                hasher.update(&current);
            }

            current = hasher.finalize().into();
            idx /= 2;
            total = total.div_ceil(2);
        }

        if current != self.expected_root {
            return Err(format!(
                "N121.4: inclusion proof failed. Computed={:02x?}, Expected={:02x?}",
                &current[..4],
                &self.expected_root[..4]
            ));
        }

        Ok(())
    }
}

/// N121.4: Build an inclusion proof from the full slash history.
pub fn build_inclusion_proof(
    slashes: &[ExecutedSlash],
    index: usize,
) -> Result<SlashingInclusionProof, String> {
    if slashes.is_empty() {
        return Err("N121.4: cannot build proof for empty ledger".into());
    }
    if index >= slashes.len() {
        return Err(format!(
            "N121.4: index {} out of bounds (len={})",
            index,
            slashes.len()
        ));
    }

    let mut leaves: Vec<[u8; 32]> = slashes
        .iter()
        .map(SlashingInclusionProof::leaf_hash)
        .collect();
    let total_count = leaves.len();
    let slash = slashes[index].clone();

    let mut siblings = Vec::new();
    let mut idx = index;
    let mut total = total_count;

    while total > 1 {
        let is_left = idx.is_multiple_of(2);
        let sibling_idx = if is_left { idx + 1 } else { idx - 1 };

        if sibling_idx < total {
            siblings.push(leaves[sibling_idx]);
        } else {
            // Odd number: duplicate the last node
            siblings.push(leaves[idx]);
        }

        // Compute parent level
        let mut next_level = Vec::new();
        for chunk in leaves.chunks(2) {
            let mut hasher = Hasher::new();
            hasher.update(b"AMUN_SLASH_BRANCH_V1");
            hasher.update(&chunk[0]);
            if chunk.len() == 2 {
                hasher.update(&chunk[1]);
            } else {
                hasher.update(&chunk[0]);
            }
            next_level.push(hasher.finalize().into());
        }
        leaves = next_level;
        idx /= 2;
        total = total.div_ceil(2);
    }

    let expected_root = leaves[0];

    Ok(SlashingInclusionProof {
        slash,
        index,
        total_count,
        siblings,
        expected_root,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slashing_ledger::ExecutedSlash;
    use crate::slashing_merkle::merkle_root;

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
    fn n121_4_single_element_inclusion_proof() {
        let slashes = vec![make_slash(1, 100)];
        let proof = build_inclusion_proof(&slashes, 0).unwrap();
        assert!(
            proof.verify().is_ok(),
            "N121.4 FAIL: single element proof must verify"
        );
        assert_eq!(proof.total_count, 1);
        assert_eq!(proof.index, 0);
    }

    #[test]
    fn n121_4_multi_element_inclusion_proof() {
        let slashes: Vec<_> = (1..=5).map(|i| make_slash(i, i as u64 * 100)).collect();

        // Prove inclusion of element at index 2
        let proof = build_inclusion_proof(&slashes, 2).unwrap();
        assert!(
            proof.verify().is_ok(),
            "N121.4 FAIL: inclusion proof for index 2 must verify"
        );
        assert_eq!(proof.slash.certificate_id, [3; 32]);
    }

    #[test]
    fn n121_4_wrong_element_rejected() {
        let slashes: Vec<_> = (1..=3).map(|i| make_slash(i, i as u64 * 100)).collect();
        let mut proof = build_inclusion_proof(&slashes, 1).unwrap();

        // Replace the slash with a different one
        proof.slash = make_slash(9, 999);

        assert!(
            proof.verify().is_err(),
            "N121.4 FAIL: proof with wrong slash must be rejected"
        );
    }

    #[test]
    fn n121_4_out_of_bounds_index_rejected() {
        let slashes = vec![make_slash(1, 100)];
        assert!(build_inclusion_proof(&slashes, 5).is_err());
    }

    #[test]
    fn n121_4_empty_ledger_rejected() {
        assert!(build_inclusion_proof(&[], 0).is_err());
    }

    #[test]
    fn n121_4_proof_matches_merkle_root() {
        let slashes: Vec<_> = (1..=7).map(|i| make_slash(i, i as u64 * 100)).collect();
        let expected = merkle_root(&slashes);

        for i in 0..slashes.len() {
            let proof = build_inclusion_proof(&slashes, i).unwrap();
            assert_eq!(
                proof.expected_root, expected,
                "N121.4 FAIL: proof root must match merkle root at index {}",
                i
            );
            assert!(
                proof.verify().is_ok(),
                "N121.4 FAIL: proof must verify at index {}",
                i
            );
        }
    }
}
