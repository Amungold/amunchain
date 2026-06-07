use amun_transition_proof::transition_proof::TransitionProof;
use serde::{Deserialize, Serialize};

/// A replay verification result for a single transition proof.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayVerificationRecord {
    pub proof_hash: [u8; 32],
    pub state_root_match: bool,
    pub proof_hash_match: bool,
    pub gas_used_match: bool,
    pub replay_success: bool,
}

impl ReplayVerificationRecord {
    /// Returns true if all checks passed.
    pub fn is_verified(&self) -> bool {
        self.replay_success && self.state_root_match && self.proof_hash_match && self.gas_used_match
    }
}

/// A block that has been verified through deterministic replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayVerifiedBlock {
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub replay_root: [u8; 32],
    pub transitions: Vec<TransitionProof>,
    pub replay_verifications: Vec<ReplayVerificationRecord>,
    pub all_verified: bool,
}

impl ReplayVerifiedBlock {
    /// Compute the replay root — Merkle root of all replay verification hashes.
    pub fn compute_replay_root(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_REPLAY_ROOT_V1");
        for rv in &self.replay_verifications {
            hasher.update(&rv.proof_hash);
            hasher.update(&[rv.replay_success as u8]);
        }
        let hash = hasher.finalize();
        let mut root = [0u8; 32];
        root.copy_from_slice(hash.as_bytes());
        root
    }
}

/// A QC that includes replay verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBackedQC {
    pub block_hash: [u8; 32],
    pub block_height: u64,
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub replay_root: [u8; 32],
    pub signatures: Vec<Vec<u8>>,
    pub signer_count: usize,
    pub quorum_threshold: usize,
    pub all_replays_verified: bool,
}

impl ReplayBackedQC {
    pub fn is_valid(&self) -> bool {
        self.signer_count >= self.quorum_threshold && self.all_replays_verified
    }

    pub fn for_block(block: &ReplayVerifiedBlock, threshold: usize) -> Self {
        Self {
            block_hash: block.block_hash,
            block_height: block.block_height,
            state_root: block.state_root,
            proof_root: block.proof_root,
            replay_root: block.replay_root,
            signatures: Vec::new(),
            signer_count: 0,
            quorum_threshold: threshold,
            all_replays_verified: block.all_verified,
        }
    }
}

/// A constitutional finality certificate backed by replay verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBackedFinalityCertificate {
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub replay_root: [u8; 32],
    pub qc: ReplayBackedQC,
    pub certificate_hash: [u8; 32],
}

impl ReplayBackedFinalityCertificate {
    pub fn issue(block: &ReplayVerifiedBlock, qc: ReplayBackedQC) -> Self {
        let mut cert = Self {
            block_height: block.block_height,
            block_hash: block.block_hash,
            state_root: block.state_root,
            proof_root: block.proof_root,
            replay_root: block.replay_root,
            qc,
            certificate_hash: [0u8; 32],
        };
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_REPLAY_FINALITY_V1");
        hasher.update(&self.block_height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.state_root);
        hasher.update(&self.proof_root);
        hasher.update(&self.replay_root);
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn verify(&self) -> bool {
        self.certificate_hash == self.compute_hash() && self.qc.is_valid()
    }
}
