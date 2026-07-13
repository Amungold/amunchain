//! Replay Semantics — formal constitutional model for replay.
//! Layer 0.75 — Constitutional Replay Theory.
//! Defines WHAT replay IS, not HOW to implement it.

extern crate alloc;
use alloc::vec::Vec;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayDomain { Consensus, Execution, FullSystem, Governance, Transcript }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayEpoch { pub epoch_id: [u8; 32], pub start_sequence: u64, pub end_sequence: Option<u64>, pub replay_version: u32 }
impl ReplayEpoch {
    pub fn new(epoch_id: [u8; 32], start_sequence: u64, replay_version: u32) -> Self { Self { epoch_id, start_sequence, end_sequence: None, replay_version } }
    pub fn contains(&self, sequence: u64) -> bool { sequence >= self.start_sequence && self.end_sequence.map(|e| sequence <= e).unwrap_or(true) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayBoundary { pub finalized_sequence: u64, pub boundary_chain_hash: [u8; 32], pub boundary_state_root: [u8; 32], pub epoch: ReplayEpoch, pub replay_version: u32 }
impl ReplayBoundary { pub fn genesis(genesis_hash: [u8; 32]) -> Self { Self { finalized_sequence: 0, boundary_chain_hash: genesis_hash, boundary_state_root: genesis_hash, epoch: ReplayEpoch::new(genesis_hash, 0, 1), replay_version: 1 } } }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCertificate {
    pub domain: ReplayDomain, pub epoch: ReplayEpoch,
    pub transcript_root: [u8; 32], pub state_root: [u8; 32],
    pub receipt_root: [u8; 32], pub ordering_root: [u8; 32],
    pub boundary: ReplayBoundary, pub event_count: u64, pub replay_version: u32,
    pub certificate_hash: [u8; 32],
}
impl ReplayCertificate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(domain: ReplayDomain, epoch: ReplayEpoch, transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], boundary: ReplayBoundary, event_count: u64, replay_version: u32) -> Self {
        let mut cert = Self { domain, epoch, transcript_root, state_root, receipt_root, ordering_root, boundary, event_count, replay_version, certificate_hash: [0; 32] };
        cert.certificate_hash = cert.compute_hash(); cert
    }
    fn compute_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new(); h.update(b"AMUN|REPLAY_CERT|V1"); h.update((self.domain as u8).to_le_bytes()); h.update(self.epoch.epoch_id);
        h.update(self.transcript_root); h.update(self.state_root); h.update(self.receipt_root); h.update(self.ordering_root);
        h.update(self.boundary.boundary_chain_hash); h.update(self.event_count.to_le_bytes()); h.update(self.replay_version.to_le_bytes());
        h.finalize().into()
    }
    pub fn verify(&self) -> bool { self.certificate_hash == self.compute_hash() }
    pub fn prove_equivalence(a: &Self, b: &Self) -> bool { a.transcript_root == b.transcript_root && a.state_root == b.state_root && a.receipt_root == b.receipt_root && a.ordering_root == b.ordering_root && a.domain == b.domain }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayEquivalence { Strict, Semantic, EpochBounded(ReplayEpoch) }
impl ReplayEquivalence {
    pub fn verify(&self, a: &ReplayCertificate, b: &ReplayCertificate) -> bool {
        match self { ReplayEquivalence::Strict => a.certificate_hash == b.certificate_hash, ReplayEquivalence::Semantic => ReplayCertificate::prove_equivalence(a, b), ReplayEquivalence::EpochBounded(ep) => a.epoch.epoch_id == ep.epoch_id && b.epoch.epoch_id == ep.epoch_id && ReplayCertificate::prove_equivalence(a, b) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayFailure {
    TranscriptMismatch { expected_root: [u8; 32], actual_root: [u8; 32] },
    StateDivergence { expected_root: [u8; 32], actual_root: [u8; 32] },
    OrderingViolation { expected_sequence: u64, actual_sequence: u64 },
    EpochBoundaryViolation { expected_epoch: Box<ReplayEpoch>, actual_epoch: Box<ReplayEpoch> },
    VersionMismatch { expected_version: u32, actual_version: u32 },
    BoundaryViolation { expected_boundary: Box<ReplayBoundary>, actual_boundary: Box<ReplayBoundary> },
    ReplayResourceExhaustion { limit: usize, attempted: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayAuthority { SelfVerification, ValidatorQuorum { required_signatures: u64 }, ConstitutionalCourt, PublicVerification }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayWitness { pub start_sequence: u64, pub end_sequence: u64, pub pre_state_root: [u8; 32], pub post_state_root: [u8; 32], pub transcript_fragment_hash: [u8; 32], pub witness_data: Vec<u8> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCheckpoint { pub sequence: u64, pub state_root: [u8; 32], pub transcript_chain_hash: [u8; 32], pub certificate: ReplayCertificate }
impl ReplayCheckpoint { pub fn verify(&self, boundary: &ReplayBoundary) -> bool { self.sequence >= boundary.finalized_sequence && self.certificate.verify() && self.certificate.boundary.boundary_chain_hash == boundary.boundary_chain_hash } }

pub mod laws {
    use super::*;
    pub fn replay_determinism(a: &ReplayCertificate, b: &ReplayCertificate) -> Result<(), ReplayFailure> {
        if a.transcript_root != b.transcript_root { return Err(ReplayFailure::TranscriptMismatch { expected_root: a.transcript_root, actual_root: b.transcript_root }); }
        if a.state_root != b.state_root { return Err(ReplayFailure::StateDivergence { expected_root: a.state_root, actual_root: b.state_root }); }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn te() -> ReplayEpoch { ReplayEpoch::new([0xBB; 32], 0, 1) }
    fn tb() -> ReplayBoundary { ReplayBoundary::genesis([0xAA; 32]) }
    #[test] fn test_cert_self_verifying() { assert!(ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1).verify()); }
    #[test] fn test_cert_tamper_detected() { let mut c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); c.state_root = [0xFF;32]; assert!(!c.verify()); }
    #[test] fn test_equivalence_strict() { let c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(ReplayEquivalence::Strict.verify(&c, &c)); }
    #[test] fn test_law_determinism_divergence() { let a = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); let b = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0xFF;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(laws::replay_determinism(&a, &b).is_err()); }
}
