// N121.3 — Slashing Fraud Proof Engine
// =====================================
// Produces verifiable proofs when a block's slashing_root does not
// match the locally computed root. These proofs can be gossiped
// so other validators can independently verify the mismatch.

use crate::slashing_ledger::ExecutedSlash;
use crate::slashing_merkle::merkle_root;

/// N121.3: A cryptographic proof that a block's slashing_root is invalid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct SlashingFraudProof {
    /// Hash of the block with the invalid root
    #[serde(with = "serde_bytes")]
    pub block_hash: [u8; 32],

    /// Height of the block
    pub block_height: u64,

    /// The slashing_root claimed in the block header
    #[serde(with = "serde_bytes")]
    pub claimed_root: [u8; 32],

    /// The correctly computed slashing_root
    #[serde(with = "serde_bytes")]
    pub expected_root: [u8; 32],

    /// The executed slashes that produce the expected_root
    pub slash_history: Vec<ExecutedSlash>,

    /// Blake3 hash of the fraud proof itself
    #[serde(with = "serde_bytes")]
    pub proof_id: [u8; 32],
}

impl SlashingFraudProof {
    /// N121.3: Create a fraud proof when a mismatch is detected.
    pub fn new(
        block_hash: [u8; 32],
        block_height: u64,
        claimed_root: [u8; 32],
        slash_history: Vec<ExecutedSlash>,
    ) -> Self {
        let expected_root = merkle_root(&slash_history);
        let mut proof = Self {
            block_hash,
            block_height,
            claimed_root,
            expected_root,
            slash_history,
            proof_id: [0u8; 32],
        };
        proof.proof_id = proof.compute_proof_id();
        proof
    }

    /// N121.3: Compute deterministic proof ID.
    fn compute_proof_id(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_FRAUD_PROOF_V1");
        hasher.update(&self.block_hash);
        hasher.update(&self.block_height.to_le_bytes());
        hasher.update(&self.claimed_root);
        hasher.update(&self.expected_root);
        hasher.finalize().into()
    }

    /// N121.3: Verify the fraud proof independently.
    /// Any validator can call this to confirm the fraud.
    pub fn verify(&self) -> Result<(), String> {
        // Verify the proof ID matches
        let recomputed_id = self.compute_proof_id();
        if recomputed_id != self.proof_id {
            return Err("N121.3: proof ID mismatch".into());
        }

        // Recompute the expected root from the provided history
        let recomputed_root = merkle_root(&self.slash_history);
        if recomputed_root != self.expected_root {
            return Err("N121.3: expected_root does not match slash history".into());
        }

        // The claimed root must differ from the expected root (otherwise it's not fraud)
        if self.claimed_root == self.expected_root {
            return Err("N121.3: claimed_root matches expected_root — no fraud detected".into());
        }

        Ok(())
    }

    /// N121.3: Returns true if this proof demonstrates fraud.
    pub fn is_fraudulent(&self) -> bool {
        self.claimed_root != self.expected_root && self.verify().is_ok()
    }
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
    fn n121_3_valid_fraud_proof_verifies() {
        let history = vec![make_slash(1, 100), make_slash(2, 200)];
        let _expected = merkle_root(&history);
        let claimed = [0xBA; 32]; // Wrong root

        let proof = SlashingFraudProof::new([0xAA; 32], 42, claimed, history);

        assert_ne!(proof.proof_id, [0u8; 32]);
        assert!(
            proof.verify().is_ok(),
            "N121.3 FAIL: valid fraud proof must verify"
        );
        assert!(proof.is_fraudulent());
    }

    #[test]
    fn n121_3_matching_roots_not_fraud() {
        let history = vec![make_slash(1, 100)];
        let root = merkle_root(&history);

        let proof = SlashingFraudProof::new([0xAA; 32], 42, root, history);

        assert!(
            proof.verify().is_err(),
            "N121.3 FAIL: matching roots must not be fraud"
        );
        assert!(!proof.is_fraudulent());
    }

    #[test]
    fn n121_3_tampered_history_rejected() {
        let history = vec![make_slash(1, 100)];
        let _expected = merkle_root(&history);
        let claimed = [0xBA; 32];

        let mut proof = SlashingFraudProof::new([0xAA; 32], 42, claimed, history);

        // Tamper with the slash history after proof creation
        proof.slash_history[0].amount = 99999;

        assert!(
            proof.verify().is_err(),
            "N121.3 FAIL: tampered slash history must invalidate proof"
        );
    }

    #[test]
    fn n121_3_tampered_proof_id_rejected() {
        let history = vec![make_slash(1, 100)];
        let _expected = merkle_root(&history);
        let claimed = [0xBA; 32];

        let mut proof = SlashingFraudProof::new([0xAA; 32], 42, claimed, history);

        proof.proof_id = [0xFF; 32]; // Tamper

        assert!(
            proof.verify().is_err(),
            "N121.3 FAIL: tampered proof ID must be rejected"
        );
    }

    #[test]
    fn n121_3_roundtrip_serialization() {
        let history = vec![make_slash(1, 100), make_slash(2, 200)];
        let _expected = merkle_root(&history);
        let proof = SlashingFraudProof::new([0xAA; 32], 42, [0xBA; 32], history);

        let encoded = postcard::to_stdvec(&proof).unwrap();
        let decoded: SlashingFraudProof = postcard::from_bytes(&encoded).unwrap();

        assert_eq!(decoded.proof_id, proof.proof_id);
        assert_eq!(decoded.claimed_root, proof.claimed_root);
        assert_eq!(decoded.expected_root, proof.expected_root);
        assert!(decoded.verify().is_ok());
    }
}
