//! Transcript Semantics — constitutional causal transcript model.
//! Layer 0.75 — Constitutional Causal Ontology.

extern crate alloc;
use alloc::vec::Vec;
use amun_replay_semantics::ReplayDomain;
use sha2::{Sha256, Digest};

// ─── Event Identity ───────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventIdentity { pub event_hash: [u8; 32], pub causal_parent: [u8; 32], pub authority_root: [u8; 32], pub transcript_position: u64, pub domain: ReplayDomain, pub epoch_id: [u8; 32] }
impl EventIdentity {
    pub fn new(event_hash: [u8; 32], causal_parent: [u8; 32], authority_root: [u8; 32], transcript_position: u64, domain: ReplayDomain, epoch_id: [u8; 32]) -> Self { Self { event_hash, causal_parent, authority_root, transcript_position, domain, epoch_id } }
    pub fn verify_causal_chain(&self, parent: &EventIdentity) -> bool { self.causal_parent == parent.event_hash && self.transcript_position == parent.transcript_position + 1 }
}

// ─── Event Authority ───────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventAuthority { Authoritative, Derived, Ephemeral, LocalOnly, Certifying }
impl EventAuthority {
    pub fn is_replay_required(&self) -> bool { matches!(self, EventAuthority::Authoritative | EventAuthority::Certifying) }
    pub fn is_causal(&self) -> bool { matches!(self, EventAuthority::Authoritative | EventAuthority::Derived | EventAuthority::Certifying) }
}

// ─── Replay Class ──────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayClass { ReplayRequired, ReplayRecommended, ReplayDerived, ReplayExcluded, ReplayOptional }

// ─── Causal Model ──────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum CausalRelation { HappensBefore, Concurrent, DirectCause, DependsOn }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct CausalNode { pub identity: EventIdentity, pub dependencies: Vec<EventIdentity>, pub effects: Vec<EventIdentity> }
impl CausalNode { pub fn new(identity: EventIdentity) -> Self { Self { identity, dependencies: Vec::new(), effects: Vec::new() } } }

// ─── Transcript Entry ──────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ConsensusEventType { Proposal, Prevote, Precommit, QCFormed, RoundTimeout, ViewChange }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ConsensusEvent { pub identity: EventIdentity, pub round: u64, pub event_type: ConsensusEventType, pub authority: EventAuthority }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct ExecutionEvent { pub identity: EventIdentity, pub block_hash: [u8; 32], pub pre_state_root: [u8; 32], pub post_state_root: [u8; 32], pub receipt_hash: [u8; 32], pub authority: EventAuthority }
#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum GovernanceEventType { ProposalSubmitted, VoteCast, ProposalPassed, ProposalVetoed, ProposalExecuted, EpochTransition, UpgradeActivated }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct GovernanceEvent { pub identity: EventIdentity, pub event_type: GovernanceEventType, pub authority: EventAuthority }
#[derive(Debug, Clone, PartialEq, Eq)] pub struct CertifyingEvent { pub identity: EventIdentity, pub certifies: EventIdentity, pub certificate_hash: [u8; 32], pub certifier_authority: [u8; 32] }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranscriptEntry { Consensus(ConsensusEvent), Execution(ExecutionEvent), Governance(GovernanceEvent), Certifying(CertifyingEvent) }
impl TranscriptEntry {
    pub fn identity(&self) -> &EventIdentity {
        match self { TranscriptEntry::Consensus(e) => &e.identity, TranscriptEntry::Execution(e) => &e.identity, TranscriptEntry::Governance(e) => &e.identity, TranscriptEntry::Certifying(e) => &e.identity }
    }
    pub fn authority(&self) -> EventAuthority {
        match self { TranscriptEntry::Consensus(e) => e.authority, TranscriptEntry::Execution(e) => e.authority, TranscriptEntry::Governance(e) => e.authority, TranscriptEntry::Certifying(_) => EventAuthority::Certifying }
    }
    pub fn is_replay_required(&self) -> bool { self.authority().is_replay_required() }
    pub fn domain(&self) -> ReplayDomain {
        match self { TranscriptEntry::Consensus(_) => ReplayDomain::Consensus, TranscriptEntry::Execution(_) => ReplayDomain::Execution, TranscriptEntry::Governance(_) => ReplayDomain::Governance, TranscriptEntry::Certifying(_) => ReplayDomain::FullSystem }
    }
}

// ─── Immutable Certificate ─────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImmutableReplayCertificate { inner: CertifiedEnvelope }
#[derive(Debug, Clone, PartialEq, Eq)]
struct CertifiedEnvelope { domain: ReplayDomain, epoch_id: [u8; 32], transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], event_count: u64, replay_version: u32, certificate_hash: [u8; 32] }
impl ImmutableReplayCertificate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(domain: ReplayDomain, epoch_id: [u8; 32], transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], event_count: u64, replay_version: u32) -> Self {
        let mut env = CertifiedEnvelope { domain, epoch_id, transcript_root, state_root, receipt_root, ordering_root, event_count, replay_version, certificate_hash: [0; 32] };
        env.certificate_hash = Self::compute(&env); Self { inner: env }
    }
    fn compute(env: &CertifiedEnvelope) -> [u8; 32] { let mut h = Sha256::new(); h.update(b"AMUN|IMMUTABLE_CERT|V1"); h.update((env.domain as u8).to_le_bytes()); h.update(env.epoch_id); h.update(env.transcript_root); h.update(env.state_root); h.update(env.receipt_root); h.update(env.ordering_root); h.update(env.event_count.to_le_bytes()); h.update(env.replay_version.to_le_bytes()); h.finalize().into() }
    pub fn domain(&self) -> ReplayDomain { self.inner.domain }
    pub fn state_root(&self) -> [u8; 32] { self.inner.state_root }
    pub fn certificate_hash(&self) -> [u8; 32] { self.inner.certificate_hash }
    pub fn verify(&self) -> bool { self.inner.certificate_hash == Self::compute(&self.inner) }
    pub fn prove_equivalence(a: &Self, b: &Self) -> bool { a.inner.transcript_root == b.inner.transcript_root && a.inner.state_root == b.inner.state_root && a.inner.receipt_root == b.inner.receipt_root && a.inner.ordering_root == b.inner.ordering_root && a.inner.domain == b.inner.domain }
}

// ─── Witness Types ─────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayWitness {
    MerkleWitness { leaf_hash: [u8; 32], proof_hashes: Vec<[u8; 32]>, leaf_index: u64 },
    ExecutionWitness { block_hash: [u8; 32], trace_hash: [u8; 32], step_count: u64 },
    TranscriptWitness { start_sequence: u64, end_sequence: u64, fragment_hash: [u8; 32] },
    ReceiptWitness { receipt_hashes: Vec<[u8; 32]>, chain_root: [u8; 32] },
    CompositeWitness { witnesses: Vec<ReplayWitness>, composite_hash: [u8; 32] },
}
impl ReplayWitness {
    pub fn witness_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new(); h.update(b"AMUN|WITNESS|V1");
        match self { ReplayWitness::MerkleWitness { leaf_hash, proof_hashes, leaf_index } => { h.update(b"MERKLE"); h.update(leaf_hash); h.update(leaf_index.to_le_bytes()); for ph in proof_hashes { h.update(ph); } } ReplayWitness::ExecutionWitness { block_hash, trace_hash, step_count } => { h.update(b"EXECUTION"); h.update(block_hash); h.update(trace_hash); h.update(step_count.to_le_bytes()); } ReplayWitness::TranscriptWitness { start_sequence, end_sequence, fragment_hash } => { h.update(b"TRANSCRIPT"); h.update(start_sequence.to_le_bytes()); h.update(end_sequence.to_le_bytes()); h.update(fragment_hash); } ReplayWitness::ReceiptWitness { receipt_hashes, chain_root } => { h.update(b"RECEIPT"); for rh in receipt_hashes { h.update(rh); } h.update(chain_root); } ReplayWitness::CompositeWitness { composite_hash, .. } => { h.update(b"COMPOSITE"); h.update(composite_hash); } }
        h.finalize().into()
    }
}

pub mod laws {
    use super::*;
    #[derive(Debug, Clone, PartialEq, Eq)] pub enum TranscriptError { CausalChainBroken { parent_hash: [u8; 32], child_parent_hash: [u8; 32] }, AuthorityMismatch { expected: EventAuthority, actual: EventAuthority }, IncompleteReplay { expected: usize, actual: usize } }
    pub fn causal_integrity(parent: &EventIdentity, child: &EventIdentity) -> Result<(), TranscriptError> { if !child.verify_causal_chain(parent) { Err(TranscriptError::CausalChainBroken { parent_hash: parent.event_hash, child_parent_hash: child.causal_parent }) } else { Ok(()) } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_causal_chain() { let p = EventIdentity::new([0x01;32],[0x00;32],[0xAA;32],1,ReplayDomain::Consensus,[0xBB;32]); let c = EventIdentity::new([0x02;32],[0x01;32],[0xAA;32],2,ReplayDomain::Consensus,[0xBB;32]); assert!(c.verify_causal_chain(&p)); }
    #[test] fn test_authority_replay_required() { assert!(EventAuthority::Authoritative.is_replay_required()); assert!(!EventAuthority::Derived.is_replay_required()); }
    #[test] fn test_immutable_cert() { let c = ImmutableReplayCertificate::new(ReplayDomain::Consensus,[0xBB;32],[0x01;32],[0x02;32],[0x03;32],[0x04;32],100,1); assert!(c.verify()); let h1 = c.certificate_hash(); let h2 = c.certificate_hash(); assert_eq!(h1, h2); }
    #[test] fn test_witness_hash() { let w = ReplayWitness::MerkleWitness { leaf_hash: [0x01;32], proof_hashes: vec![[0x02;32]], leaf_index: 0 }; assert_ne!(w.witness_hash(), [0;32]); }
}
