// ============================================================================
// N109.8 — Cryptographic Execution Commitment
// ============================================================================
// Every validator signs an ExecutionCommitment that cryptographically binds:
//   WHO executed:  validator_id
//   WHAT:          block_hash
//   AT:            height
//   RESULT:        state_root
//
// execution_root = blake3(validator_id || height || block_hash || state_root)
//
// This prevents:
//   - Replay of commitments across heights
//   - Reuse of commitments by different validators
//   - Splitting: voting for block A while committing to block B
//
// After N109.8, every vote carries a cryptographic execution trace.
// This is the foundation for N110 Slashing and N111 Evidence.
// ============================================================================

use blake3::Hasher;
use serde::{Deserialize, Serialize};

/// N109.8: A signed statement that a specific validator executed a specific
/// block at a specific height and obtained a specific state root.
///
/// The execution_root is a domain-separated hash binding all four fields,
/// preventing any form of splitting or replay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionCommitment {
    /// The validator making this commitment
    #[serde(with = "serde_bytes")]
    pub validator_id: [u8; 32],

    /// Block height this commitment refers to
    pub height: u64,

    /// Hash of the block that was executed
    #[serde(with = "serde_bytes")]
    pub block_hash: [u8; 32],

    /// State root obtained from executing the block
    #[serde(with = "serde_bytes")]
    pub state_root: [u8; 32],

    /// blake3(validator_id || height.to_le_bytes() || block_hash || state_root)
    /// Binds all four fields into a single cryptographic commitment.
    #[serde(with = "serde_bytes")]
    pub execution_root: [u8; 32],

    /// Ed25519 signature over execution_root by validator_id's key
    #[serde(with = "serde_bytes")]
    pub signature: [u8; 64],
}

impl ExecutionCommitment {
    /// N109.8: Compute the execution_root from the four binding fields.
    /// Uses domain separation "AMUN_EXEC_COMMIT_V1" to prevent
    /// collision with other hash uses in the protocol.
    pub fn compute_execution_root(
        validator_id: &[u8; 32],
        height: u64,
        block_hash: &[u8; 32],
        state_root: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_EXEC_COMMIT_V1");
        hasher.update(validator_id);
        hasher.update(&height.to_le_bytes());
        hasher.update(block_hash);
        hasher.update(state_root);
        hasher.finalize().into()
    }

    /// N109.8: Create a new ExecutionCommitment and compute its execution_root.
    /// The caller must sign the commitment separately using their Ed25519 key.
    pub fn new(
        validator_id: [u8; 32],
        height: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
    ) -> Self {
        let execution_root =
            Self::compute_execution_root(&validator_id, height, &block_hash, &state_root);
        Self {
            validator_id,
            height,
            block_hash,
            state_root,
            execution_root,
            signature: [0u8; 64], // To be filled by signer
        }
    }

    /// N109.8: Sign the commitment with an Ed25519 signing key.
    /// The signature covers execution_root, which binds all four fields.
    pub fn sign(&mut self, signing_key: &ed25519_dalek::SigningKey) {
        use ed25519_dalek::Signer;
        self.signature = signing_key.sign(&self.execution_root).to_bytes();
    }

    /// N109.8: Verify the commitment's signature.
    /// Returns true if the signature is valid for the claimed validator_id
    /// and covers the execution_root.
    pub fn verify(&self) -> Result<(), String> {
        use ed25519_dalek::Verifier;

        // Verify the execution_root matches the four fields
        let recomputed = Self::compute_execution_root(
            &self.validator_id,
            self.height,
            &self.block_hash,
            &self.state_root,
        );
        if recomputed != self.execution_root {
            return Err("N109.8: execution_root does not match fields".into());
        }

        // Verify the signature
        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&self.validator_id)
            .map_err(|e| format!("N109.8: invalid validator_id: {}", e))?;

        let signature = ed25519_dalek::Signature::from_bytes(&self.signature);
        verifying_key
            .verify(&self.execution_root, &signature)
            .map_err(|e| format!("N109.8: signature verification failed: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    fn make_keypair(seed: u8) -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[seed; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    #[test]
    fn n109_8_compute_execution_root_is_deterministic() {
        let pk = [1u8; 32];
        let h = 42;
        let bh = [2u8; 32];
        let sr = [3u8; 32];

        let r1 = ExecutionCommitment::compute_execution_root(&pk, h, &bh, &sr);
        let r2 = ExecutionCommitment::compute_execution_root(&pk, h, &bh, &sr);
        assert_eq!(r1, r2, "execution_root must be deterministic");
    }

    #[test]
    fn n109_8_different_validator_different_root() {
        let h = 1;
        let bh = [0xAA; 32];
        let sr = [0xBB; 32];

        let r1 = ExecutionCommitment::compute_execution_root(&[1u8; 32], h, &bh, &sr);
        let r2 = ExecutionCommitment::compute_execution_root(&[2u8; 32], h, &bh, &sr);
        assert_ne!(r1, r2, "different validators must produce different roots");
    }

    #[test]
    fn n109_8_different_height_different_root() {
        let pk = [1u8; 32];
        let bh = [0xAA; 32];
        let sr = [0xBB; 32];

        let r1 = ExecutionCommitment::compute_execution_root(&pk, 1, &bh, &sr);
        let r2 = ExecutionCommitment::compute_execution_root(&pk, 2, &bh, &sr);
        assert_ne!(r1, r2, "different heights must produce different roots");
    }

    #[test]
    fn n109_8_sign_and_verify() {
        let (sk, pk) = make_keypair(42);
        let bh = [0xCC; 32];
        let sr = [0xDD; 32];

        let mut commit = ExecutionCommitment::new(pk, 5, bh, sr);
        commit.sign(&sk);

        assert!(commit.verify().is_ok(), "signature must verify");
    }

    #[test]
    fn n109_8_tampered_state_root_rejected() {
        let (sk, pk) = make_keypair(7);
        let bh = [0x11; 32];
        let sr = [0x22; 32];

        let mut commit = ExecutionCommitment::new(pk, 3, bh, sr);
        commit.sign(&sk);

        // Tamper: change state_root without re-signing
        commit.state_root = [0xFF; 32];

        assert!(
            commit.verify().is_err(),
            "tampered state_root must fail verification"
        );
    }

    #[test]
    fn n109_8_replayed_commitment_rejected() {
        let (sk, pk) = make_keypair(99);
        let bh = [0xAB; 32];
        let sr = [0xCD; 32];

        // Create commitment at height 1
        let mut commit_h1 = ExecutionCommitment::new(pk, 1, bh, sr);
        commit_h1.sign(&sk);

        // Try to replay the SAME commitment at height 2 (should fail)
        let commit_h2 = ExecutionCommitment::new(pk, 2, bh, sr);
        // execution_root differs because height is in the hash
        assert_ne!(
            commit_h1.execution_root, commit_h2.execution_root,
            "replay across heights must produce different execution_root"
        );
    }
}
