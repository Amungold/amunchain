use amun_certificate_network::distribution::LightClientProofBundle;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub mod bootstrap;
pub mod chain;
pub mod inclusion;

// ============================================================
// N12A: Checkpoint Certificate
// ============================================================

/// A checkpoint certificate covering a range of blocks [start_height, end_height].
///
/// It commits to the final state root, evidence root, and replay certificate root
/// at end_height. Uses Blake3 for constitutional hash consistency with the rest
/// of the AmunChain proof layers (ReplayCertificate, CertificateMerkleRoot, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointCertificate {
    /// First block height in this checkpoint range.
    pub start_height: u64,
    /// Last block height in this checkpoint range (inclusive).
    pub end_height: u64,
    /// Number of blocks covered by this checkpoint.
    pub block_count: u64,
    /// State root at end_height.
    pub final_state_root: String,
    /// Evidence root at end_height.
    pub final_evidence_root: String,
    /// Replay certificate merkle root at end_height.
    pub final_replay_certificate_root: String,
    /// Blake3 hash of all checkpoint fields (constitutional commitment).
    pub checkpoint_hash: String,
}

impl CheckpointCertificate {
    /// Create a checkpoint certificate from a set of proof bundles.
    ///
    /// Verification steps:
    ///   1. Bundles must not be empty.
    ///   2. Bundle count must match the expected range [start_height, end_height].
    ///   3. Heights must be continuous (no gaps).
    ///   4. Every bundle must be individually valid (light client verification).
    ///
    /// Returns an error if any verification step fails.
    pub fn create(
        start_height: u64,
        end_height: u64,
        bundles: &[LightClientProofBundle],
    ) -> Result<Self, String> {
        // 1. Non-empty check
        if bundles.is_empty() {
            return Err("No bundles provided for checkpoint".into());
        }

        // 2. Count must match expected range
        let expected_count = end_height - start_height + 1;
        if bundles.len() as u64 != expected_count {
            return Err(format!(
                "Bundle count mismatch: expected {} for range [{}, {}] but got {}",
                expected_count,
                start_height,
                end_height,
                bundles.len()
            ));
        }

        // 3. Verify height continuity and individual bundle validity
        for (i, bundle) in bundles.iter().enumerate() {
            let expected_height = start_height + i as u64;
            let actual_height = bundle.block_header.block_height;

            if actual_height != expected_height {
                return Err(format!(
                    "Height discontinuity at position {}: expected height {} but got height {}",
                    i, expected_height, actual_height
                ));
            }

            // 4. Verify bundle is cryptographically valid
            bundle
                .verify()
                .map_err(|e| format!("Bundle at height {} invalid: {}", actual_height, e))?;
        }

        let last_bundle = &bundles[bundles.len() - 1];

        let mut cert = Self {
            start_height,
            end_height,
            block_count: bundles.len() as u64,
            final_state_root: last_bundle.block_header.state_root.clone(),
            final_evidence_root: last_bundle.block_header.evidence_root.clone(),
            final_replay_certificate_root: last_bundle.block_header.replay_certificate_root.clone(),
            checkpoint_hash: String::new(),
        };

        // Compute constitutional Blake3 hash
        cert.checkpoint_hash = cert.compute_hash();
        Ok(cert)
    }

    /// Compute a constitutional hash of this checkpoint certificate using Blake3.
    ///
    /// Uses domain separation prefix `AMUN_CHECKPOINT_V1` consistent with other
    /// AmunChain proof layers (ReplayCertificate uses `AMUN_REPLAY_CERTIFICATE_V1`,
    /// Block hashing uses `AMUN_BLOCK_V1`).
    pub fn compute_hash(&self) -> String {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.start_height.to_le_bytes());
        bytes.extend_from_slice(&self.end_height.to_le_bytes());
        bytes.extend_from_slice(&self.block_count.to_le_bytes());
        bytes.extend_from_slice(self.final_state_root.as_bytes());
        bytes.extend_from_slice(self.final_evidence_root.as_bytes());
        bytes.extend_from_slice(self.final_replay_certificate_root.as_bytes());

        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CHECKPOINT_V1");
        hasher.update(&bytes);
        hex::encode(hasher.finalize().as_bytes())
    }

    /// Verify the checkpoint integrity by recomputing and comparing the hash.
    ///
    /// This is a self-verification: it ensures the stored checkpoint_hash matches
    /// the Blake3 hash of all the checkpoint fields.
    pub fn verify(&self) -> Result<(), String> {
        let recomputed = self.compute_hash();
        if self.checkpoint_hash != recomputed {
            return Err(format!(
                "Checkpoint hash mismatch: stored hash '{}' does not match recomputed hash '{}'",
                self.checkpoint_hash, recomputed
            ));
        }
        Ok(())
    }
}

// ============================================================
// N12A: Checkpoint Store
// ============================================================

/// Stores checkpoint certificates keyed by their end_height.
///
/// This enables efficient retrieval of checkpoints for stateless sync,
/// fast bootstrapping, and light client verification of chain segments.
#[derive(Debug, Clone, Default)]
pub struct CheckpointStore {
    checkpoints: BTreeMap<u64, CheckpointCertificate>,
}

impl CheckpointStore {
    /// Create a new empty CheckpointStore.
    pub fn new() -> Self {
        Self::default()
    }

    /// Store a checkpoint certificate.
    /// The certificate is keyed by its end_height.
    pub fn store(&mut self, cert: CheckpointCertificate) {
        self.checkpoints.insert(cert.end_height, cert);
    }

    /// Retrieve a checkpoint by its end_height.
    pub fn get(&self, end_height: u64) -> Option<&CheckpointCertificate> {
        self.checkpoints.get(&end_height)
    }

    /// Get the latest (highest end_height) checkpoint.
    pub fn latest(&self) -> Option<&CheckpointCertificate> {
        self.checkpoints.values().last()
    }

    /// Number of stored checkpoints.
    pub fn count(&self) -> usize {
        self.checkpoints.len()
    }

    /// Get all checkpoint heights.
    pub fn all_heights(&self) -> Vec<u64> {
        self.checkpoints.keys().copied().collect()
    }
}

// ============================================================
// Tests — N12A
// ============================================================

impl CheckpointCertificate {
    /// Returns the checkpoint hash as raw bytes (32 bytes).
    /// Necessary for Merkle tree operations that expect [u8; 32].
    pub fn checkpoint_hash_bytes(&self) -> [u8; 32] {
        let decoded =
            hex::decode(&self.checkpoint_hash).expect("checkpoint hash must be valid hex");
        let mut out = [0u8; 32];
        out.copy_from_slice(&decoded[..32]);
        out
    }
}

#[cfg(test)]
mod n12a_tests {
    use super::*;
    use amun_constitutional_block::ConstitutionalBlock;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    /// Helper: create a valid LightClientProofBundle at a given height.
    fn create_bundle(height: u64, parent_hash: &str) -> LightClientProofBundle {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
        let block = ConstitutionalBlock::new(
            height,
            parent_hash.into(),
            "t".into(),
            "p".into(),
            vec![],
            hex::encode(rt.state_root()),
            "g".into(),
            "e".into(),
            "ev".into(),
            merkle_root,
        );
        LightClientProofBundle::new(block, cert, proof)
    }

    // --- Test: Basic checkpoint creation with 3 blocks ---
    #[test]
    fn n12a_checkpoint_creation() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let b1 = create_bundle(1, &b0.block_header.block_hash);
        let b2 = create_bundle(2, &b1.block_header.block_hash);

        let bundles = vec![b0, b1, b2];
        let cert = CheckpointCertificate::create(0, 2, &bundles).unwrap();

        assert_eq!(cert.start_height, 0);
        assert_eq!(cert.end_height, 2);
        assert_eq!(cert.block_count, 3);
        assert!(cert.verify().is_ok());
    }

    // --- Test: Single-block checkpoint ---
    #[test]
    fn n12a_checkpoint_single_block() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let bundles = vec![b0];
        let cert = CheckpointCertificate::create(0, 0, &bundles).unwrap();
        assert_eq!(cert.block_count, 1);
        assert!(cert.verify().is_ok());
    }

    // --- Test: Empty bundles rejected ---
    #[test]
    fn n12a_checkpoint_empty_bundles_fails() {
        let bundles: Vec<LightClientProofBundle> = vec![];
        assert!(CheckpointCertificate::create(0, 0, &bundles).is_err());
    }

    // --- Test: Discontinuity (gap) rejected ---
    #[test]
    fn n12a_checkpoint_discontinuity_fails() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let b2 = create_bundle(2, &b0.block_header.block_hash); // skip height 1

        let bundles = vec![b0, b2];
        assert!(CheckpointCertificate::create(0, 2, &bundles).is_err());
    }

    // --- Test: Tampered checkpoint fails verification ---
    #[test]
    fn n12a_checkpoint_tampered_fails() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let b1 = create_bundle(1, &b0.block_header.block_hash);

        let bundles = vec![b0, b1];
        let mut cert = CheckpointCertificate::create(0, 1, &bundles).unwrap();

        // Tamper with final state root
        cert.final_state_root = "tampered".into();
        assert!(cert.verify().is_err());
    }

    // --- Test: Checkpoint store operations ---
    #[test]
    fn n12a_checkpoint_store() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let bundles = vec![b0];
        let cert = CheckpointCertificate::create(0, 0, &bundles).unwrap();

        let mut store = CheckpointStore::new();
        store.store(cert.clone());

        assert!(store.get(0).is_some());
        assert!(store.latest().is_some());
        assert_eq!(store.count(), 1);
    }

    // --- Test: Multiple checkpoints in store ---
    #[test]
    fn n12a_checkpoint_multiple_in_store() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let b1 = create_bundle(1, &b0.block_header.block_hash);

        let cert0 = CheckpointCertificate::create(0, 0, std::slice::from_ref(&b0)).unwrap();
        let cert1 = CheckpointCertificate::create(0, 1, &[b0, b1]).unwrap();

        let mut store = CheckpointStore::new();
        store.store(cert0);
        store.store(cert1);

        assert_eq!(store.count(), 2);
        assert!(store.get(0).is_some());
        assert!(store.get(1).is_some());
        assert_eq!(store.latest().unwrap().end_height, 1);
    }

    // --- Test: Hash determinism ---
    #[test]
    fn n12a_checkpoint_hash_deterministic() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let bundles = vec![b0];
        let cert1 = CheckpointCertificate::create(0, 0, &bundles).unwrap();
        let cert2 = CheckpointCertificate::create(0, 0, &bundles).unwrap();

        assert_eq!(cert1.checkpoint_hash, cert2.checkpoint_hash);
    }

    // --- Test: Different ranges produce different hashes ---
    #[test]
    fn n12a_checkpoint_hash_different_for_different_range() {
        let b0 = create_bundle(0, &"0".repeat(64));
        let b1 = create_bundle(1, &b0.block_header.block_hash);

        let cert1 = CheckpointCertificate::create(0, 0, std::slice::from_ref(&b0)).unwrap();
        let cert2 = CheckpointCertificate::create(0, 1, &[b0, b1]).unwrap();

        assert_ne!(cert1.checkpoint_hash, cert2.checkpoint_hash);
    }
}
