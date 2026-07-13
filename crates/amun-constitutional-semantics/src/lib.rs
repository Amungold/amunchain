//! Constitutional Semantics — complete truth model for replay verification.
//! Layer 0.75 — Constitutional Truth System.

extern crate alloc;
use alloc::vec::Vec;
use amun_replay_semantics::{ReplayDomain, ReplayBoundary, ReplayFailure};
use amun_transcript_semantics::{EventAuthority, TranscriptEntry};
use sha2::{Sha256, Digest};

// ─── Transcript Commitment ─────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)] pub enum AccumulatorType { Sequential, MerkleTree, RSAAccumulator }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptCommitment { pub root: [u8; 32], pub event_count: u64, pub start_sequence: u64, pub end_sequence: u64, pub epoch_id: [u8; 32], pub accumulator_type: AccumulatorType, pub commitment_version: u32 }
impl TranscriptCommitment {
    pub fn new_sequential(events: &[TranscriptEntry], epoch_id: [u8; 32]) -> Self {
        let start = events.first().map(|e| e.identity().transcript_position).unwrap_or(0);
        let end = events.last().map(|e| e.identity().transcript_position).unwrap_or(0);
        let mut h = Sha256::new(); h.update(b"AMUN|TSCR_COMMIT|V1"); h.update((events.len() as u64).to_le_bytes());
        for event in events { h.update(event.identity().event_hash); }
        Self { root: h.finalize().into(), event_count: events.len() as u64, start_sequence: start, end_sequence: end, epoch_id, accumulator_type: AccumulatorType::Sequential, commitment_version: 1 }
    }
    pub fn verify_events(&self, events: &[TranscriptEntry]) -> bool { let r = Self::new_sequential(events, self.epoch_id); r.root == self.root && r.event_count == self.event_count }
    pub fn verify_boundary(&self, boundary: &ReplayBoundary) -> bool { self.start_sequence >= boundary.finalized_sequence }
}

// ─── Event Finality ────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] pub enum EventFinality { Tentative, QuorumAccepted, Finalized, ReplayCertified }
impl EventFinality {
    pub fn is_replay_safe(&self) -> bool { matches!(self, EventFinality::Finalized | EventFinality::ReplayCertified) }
    pub fn is_immutable(&self) -> bool { matches!(self, EventFinality::Finalized | EventFinality::ReplayCertified) }
}
#[derive(Debug, Clone, PartialEq, Eq)] pub struct FinalizedEvent { pub event: TranscriptEntry, pub finality: EventFinality, pub finalized_at_sequence: Option<u64>, pub commitment_root: Option<[u8; 32]> }
impl FinalizedEvent {
    pub fn new(event: TranscriptEntry, finality: EventFinality) -> Self { Self { event, finality, finalized_at_sequence: None, commitment_root: None } }
    pub fn with_finalization(mut self, at_sequence: u64, commitment: [u8; 32]) -> Self { self.finality = EventFinality::Finalized; self.finalized_at_sequence = Some(at_sequence); self.commitment_root = Some(commitment); self }
    pub fn with_replay_certification(mut self, at_sequence: u64) -> Self { self.finality = EventFinality::ReplayCertified; self.finalized_at_sequence = Some(at_sequence); self }
}

// ─── Authority Binding ─────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)] pub enum AuthorityProof { SingleSignature { validator_id: u64, signature: [u8; 64] }, ThresholdSignature { signer_ids: Vec<u64>, aggregate_signature: [u8; 96] }, ConstitutionalGrant { grant_proof: [u8; 32] }, SelfCertifying { derivation_proof: [u8; 32] } }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct AuthorityBinding { pub authority: EventAuthority, pub authority_set_root: [u8; 32], pub authority_epoch: [u8; 32], pub authority_proof: AuthorityProof }
impl AuthorityBinding {
    pub fn verify_binding(&self, event_authority: EventAuthority) -> bool { self.authority == event_authority }
    pub fn verify_epoch(&self, current_epoch: &[u8; 32]) -> bool { self.authority_epoch == *current_epoch }
}

// ─── Witness Normalization ─────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)] pub struct NormalizedWitness { pub domain: ReplayDomain, pub sequence: u64, pub witness_hash: [u8; 32] }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct WitnessNormalization { pub witnesses: Vec<NormalizedWitness>, pub normalization_root: [u8; 32], pub normalization_version: u32 }
impl WitnessNormalization {
    pub fn normalize(witnesses: &[(ReplayDomain, u64, [u8; 32])]) -> Self {
        let mut sorted = witnesses.to_vec(); sorted.sort_by(|a,b| { (a.0 as u8).cmp(&(b.0 as u8)).then(a.1.cmp(&b.1)) });
        let n: Vec<_> = sorted.iter().map(|(d,s,h)| NormalizedWitness { domain:*d, sequence:*s, witness_hash:*h }).collect();
        let mut hasher = Sha256::new(); hasher.update(b"AMUN|WITNESS_NORM|V1");
        for w in &n { hasher.update((w.domain as u8).to_le_bytes()); hasher.update(w.sequence.to_le_bytes()); hasher.update(w.witness_hash); }
        Self { witnesses: n, normalization_root: hasher.finalize().into(), normalization_version: 1 }
    }
    pub fn verify_normalization(&self, witnesses: &[(ReplayDomain, u64, [u8; 32])]) -> bool { Self::normalize(witnesses).normalization_root == self.normalization_root }
}

// ─── Replay Policy ─────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReplayPolicy { pub authority: EventAuthority, pub replay_required: bool, pub divergence_is_violation: bool, pub contributes_to_causality: bool, pub can_be_checkpoint: bool, pub requires_certification: bool }
impl ReplayPolicy {
    /// Compute a canonical hash that uniquely identifies this policy.
    pub fn constitutional_policy_hash(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut h = Sha256::new();
        h.update(b"AMUN|POLICY_ID|V1");
        h.update(&[
            self.replay_required as u8,
            self.divergence_is_violation as u8,
            self.contributes_to_causality as u8,
            self.can_be_checkpoint as u8,
            self.requires_certification as u8,
        ]);
        match self.authority {
            crate::EventAuthority::Authoritative => h.update(b"AUTHORITATIVE"),
            crate::EventAuthority::Derived => h.update(b"DERIVED"),
            crate::EventAuthority::Ephemeral => h.update(b"EPHEMERAL"),
            crate::EventAuthority::LocalOnly => h.update(b"LOCAL_ONLY"),
            crate::EventAuthority::Certifying => h.update(b"CERTIFYING"),
        }
        h.finalize().into()
    }

    pub const CONSENSUS_AUTHORITATIVE: Self = Self { authority: EventAuthority::Authoritative, replay_required: true, divergence_is_violation: true, contributes_to_causality: true, can_be_checkpoint: true, requires_certification: true };
    pub const DERIVED: Self = Self { authority: EventAuthority::Derived, replay_required: false, divergence_is_violation: false, contributes_to_causality: true, can_be_checkpoint: false, requires_certification: false };
    pub const EPHEMERAL: Self = Self { authority: EventAuthority::Ephemeral, replay_required: false, divergence_is_violation: false, contributes_to_causality: false, can_be_checkpoint: false, requires_certification: false };
    pub const CERTIFYING: Self = Self { authority: EventAuthority::Certifying, replay_required: true, divergence_is_violation: true, contributes_to_causality: true, can_be_checkpoint: true, requires_certification: true };
}

// ─── Continuity ────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ContinuityResult { pub is_continuous: bool, pub first_sequence: u64, pub last_sequence: u64, pub gaps: Vec<(u64, u64)>, pub violations: Vec<ContinuityViolation> }
#[derive(Debug, Clone, PartialEq, Eq)] pub enum ContinuityViolation { Gap { missing_from: u64, missing_to: u64 }, HiddenBranch { branch_root: [u8; 32], first_sequence: u64 }, EquivocationWindow { start: u64, end: u64, first_hash: [u8; 32], second_hash: [u8; 32] }, OrphanSegment { segment_root: [u8; 32], start_sequence: u64, end_sequence: u64 } }

pub mod laws {
    use super::*;
    #[allow(clippy::result_large_err)]
    pub fn transcript_continuity(events: &[TranscriptEntry]) -> Result<ContinuityResult, ReplayFailure> {
        if events.is_empty() { return Ok(ContinuityResult { is_continuous: true, first_sequence: 0, last_sequence: 0, gaps: vec![], violations: vec![] }); }
        let mut gaps = Vec::new(); let mut violations = Vec::new();
        for window in events.windows(2) { let cs = window[0].identity().transcript_position; let ns = window[1].identity().transcript_position; if ns != cs + 1 { gaps.push((cs+1, ns-1)); violations.push(ContinuityViolation::Gap { missing_from: cs+1, missing_to: ns-1 }); } }
        let first = events[0].identity().transcript_position; let last = events.last().unwrap().identity().transcript_position;
        Ok(ContinuityResult { is_continuous: gaps.is_empty(), first_sequence: first, last_sequence: last, gaps, violations })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_transcript_semantics::{ConsensusEvent, ConsensusEventType};
use amun_transcript_semantics::EventIdentity;
    fn mk(seq: u64, hash: [u8; 32], parent: [u8; 32]) -> TranscriptEntry { TranscriptEntry::Consensus(ConsensusEvent { identity: EventIdentity::new(hash, parent, [0xAA; 32], seq, ReplayDomain::Consensus, [0xBB; 32]), round: seq, event_type: ConsensusEventType::Proposal, authority: EventAuthority::Authoritative }) }
    #[test] fn test_commitment_verification() { let e = vec![mk(1,[0x01;32],[0x00;32]), mk(2,[0x02;32],[0x01;32])]; let c = TranscriptCommitment::new_sequential(&e, [0xBB;32]); assert!(c.verify_events(&e)); }
    #[test] fn test_commitment_detects_tamper() { let e1 = mk(1,[0x01;32],[0x00;32]); let e2 = mk(2,[0x02;32],[0x01;32]); let e3 = mk(3,[0x03;32],[0x02;32]); let c = TranscriptCommitment::new_sequential(&vec![e1.clone(), e2], [0xBB;32]); assert!(!c.verify_events(&vec![e1, e3])); }
    #[test] fn test_finality_progression() { assert!(EventFinality::Tentative < EventFinality::Finalized); assert!(EventFinality::Finalized < EventFinality::ReplayCertified); }
    #[test] fn test_finality_replay_safety() { assert!(!EventFinality::Tentative.is_replay_safe()); assert!(EventFinality::Finalized.is_replay_safe()); }
    #[test] fn test_witness_normalization_deterministic() { let w = vec![(ReplayDomain::Consensus,2,[0x02;32]),(ReplayDomain::Consensus,1,[0x01;32])]; assert_eq!(WitnessNormalization::normalize(&w).normalization_root, WitnessNormalization::normalize(&w).normalization_root); }
    #[test] fn test_continuity_detects_gap() { let r = laws::transcript_continuity(&vec![mk(1,[0x01;32],[0x00;32]), mk(3,[0x03;32],[0x01;32])]).unwrap(); assert!(!r.is_continuous); }
    #[test] fn test_replay_policy() { assert!(ReplayPolicy::CONSENSUS_AUTHORITATIVE.replay_required); assert!(!ReplayPolicy::EPHEMERAL.replay_required); }
    #[test] fn test_authority_binding() { let b = AuthorityBinding { authority: EventAuthority::Authoritative, authority_set_root: [0xAA;32], authority_epoch: [0xBB;32], authority_proof: AuthorityProof::SingleSignature { validator_id: 1, signature: [0;64] } }; assert!(b.verify_binding(EventAuthority::Authoritative)); assert!(!b.verify_binding(EventAuthority::Derived)); }
}
