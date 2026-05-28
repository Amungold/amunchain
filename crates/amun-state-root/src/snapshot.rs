use alloc::vec::Vec;
use amun_execution_receipt::ExecutionTranscript;
use amun_kernel::canonical::{CanonicalEncode, CanonicalEncoder};
use amun_kernel::hashing::domain_tags;

/// A snapshot carrying replay-verifiable constitutional truth.
#[derive(Clone)]
pub struct ConstitutionalSnapshot {
    pub epoch: u64,
    pub height: u64,
    pub state_root: [u8; 32],
    pub validator_root: [u8; 32],
    pub execution_root: [u8; 32],
    pub previous_snapshot_hash: [u8; 32],
    /// Full deterministic execution transcript.
    pub execution_transcript: ExecutionTranscript,
    /// Replay equivalence proof.
    pub replay_equivalence_proof: ReplayEquivalenceProof,
    pub timestamp_slot: u64,
}

/// Proof that live and replayed executions produced the same state root.
#[derive(Clone)]
pub struct ReplayEquivalenceProof {
    /// Diagnostic field: did the execution claim to be identical?
    /// Authority is derived from root equality, not this flag.
    pub identical: bool,
    pub live_root: [u8; 32],
    pub replayed_root: [u8; 32],
}

impl ReplayEquivalenceProof {
    /// Constitutional verification: only root equality is authoritative.
    /// `identical` is a diagnostic field, not an authority field.
    pub fn verify(&self) -> bool {
        self.live_root == self.replayed_root
    }
}

impl CanonicalEncode for ReplayEquivalenceProof {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.live_root);
        out.extend_from_slice(&self.replayed_root);
        (self.identical as u8).encode_canonical(out);
    }
}

impl CanonicalEncode for ConstitutionalSnapshot {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.epoch.encode_canonical(out);
        self.height.encode_canonical(out);
        out.extend_from_slice(&self.state_root);
        out.extend_from_slice(&self.validator_root);
        out.extend_from_slice(&self.execution_root);
        out.extend_from_slice(&self.previous_snapshot_hash);
        self.execution_transcript.encode_canonical(out);
        self.replay_equivalence_proof.encode_canonical(out);
        self.timestamp_slot.encode_canonical(out);
    }
}

impl ConstitutionalSnapshot {
    /// Compute the canonical seal hash.
    /// Uses `debug_assert!` to verify integrity in debug/test builds
    /// without panicking in production consensus.
    pub fn seal_hash(&self) -> [u8; 32] {
        debug_assert!(
            self.verify_constitutional_integrity(),
            "constitutional integrity violation in snapshot seal"
        );
        CanonicalEncoder::hash_value(self, domain_tags::SNAPSHOT)
    }

    /// Full constitutional integrity check before consensus finalization.
    pub fn verify_constitutional_integrity(&self) -> bool {
        self.execution_transcript.verify_transcript().is_ok()
            && self.replay_equivalence_proof.verify()
    }
}

/// Quorum seal over constitutional truth.
pub struct SnapshotSeal {
    pub snapshot_hash: [u8; 32],
    pub quorum_commitment: super::verifier::SealCommitment,
}
