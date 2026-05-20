use amun_kernel::canonical::{CanonicalEncode, CanonicalEncoder};
use amun_kernel::hashing::domain_tags;
use super::replay::ReplayCertificate;
use super::verifier::SealCommitment;

#[derive(Clone)]
pub struct ConstitutionalSnapshot {
    pub epoch:               u64,
    pub height:              u64,
    pub state_root:          [u8; 32],
    pub validator_root:      [u8; 32],
    pub execution_root:      [u8; 32],
    pub previous_snapshot_hash: [u8; 32],
    pub replay_certificate:  ReplayCertificate,
    pub timestamp_slot:      u64,
}

impl CanonicalEncode for ConstitutionalSnapshot {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.epoch.encode_canonical(out);
        self.height.encode_canonical(out);
        out.extend_from_slice(&self.state_root);
        out.extend_from_slice(&self.validator_root);
        out.extend_from_slice(&self.execution_root);
        out.extend_from_slice(&self.previous_snapshot_hash);
        self.replay_certificate.encode_canonical(out);
        self.timestamp_slot.encode_canonical(out);
    }
}

impl ConstitutionalSnapshot {
    pub fn seal_hash(&self) -> [u8; 32] {
        CanonicalEncoder::hash_value(self, domain_tags::SNAPSHOT)
    }
}

/// A quorum‑signed seal over a snapshot.
pub struct SnapshotSeal {
    pub snapshot_hash:     [u8; 32],
    pub quorum_commitment: SealCommitment,
}
