use super::compatibility::CompatibilityClass;
use amun_canonical_codec::CanonicalHasher;

pub const EVOLUTION_DOMAIN: &[u8] = b"AMUN_EVOLUTION_V1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayGuarantee {
    Exact,
    Deterministic,
    Partial,
    Unsupported,
}

impl ReplayGuarantee {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            ReplayGuarantee::Exact => 0x03,
            ReplayGuarantee::Deterministic => 0x02,
            ReplayGuarantee::Partial => 0x01,
            ReplayGuarantee::Unsupported => 0x00,
        }
    }
    pub fn rank(&self) -> u8 {
        self.canonical_tag()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotGuarantee {
    Identical,
    Compatible,
    Convertible,
    Unsupported,
}

impl SnapshotGuarantee {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            SnapshotGuarantee::Identical => 0x03,
            SnapshotGuarantee::Compatible => 0x02,
            SnapshotGuarantee::Convertible => 0x01,
            SnapshotGuarantee::Unsupported => 0x00,
        }
    }
    pub fn rank(&self) -> u8 {
        self.canonical_tag()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofGuarantee {
    Identical,
    Reconstructible,
    Convertible,
    Unsupported,
}

impl ProofGuarantee {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            ProofGuarantee::Identical => 0x03,
            ProofGuarantee::Reconstructible => 0x02,
            ProofGuarantee::Convertible => 0x01,
            ProofGuarantee::Unsupported => 0x00,
        }
    }
    pub fn rank(&self) -> u8 {
        self.canonical_tag()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceGuarantee {
    Continuous,
    Migrated,
    Reset,
    Incompatible,
}

impl GovernanceGuarantee {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            GovernanceGuarantee::Continuous => 0x03,
            GovernanceGuarantee::Migrated => 0x02,
            GovernanceGuarantee::Reset => 0x01,
            GovernanceGuarantee::Incompatible => 0x00,
        }
    }
    pub fn rank(&self) -> u8 {
        self.canonical_tag()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinuityClass {
    ExactContinuity,
    GovernanceContinuity,
    StateContinuity,
    HistoricalContinuity,
    SymbolicContinuity,
    Discontinuous,
}

impl ContinuityClass {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            ContinuityClass::ExactContinuity => 0x05,
            ContinuityClass::GovernanceContinuity => 0x04,
            ContinuityClass::StateContinuity => 0x03,
            ContinuityClass::HistoricalContinuity => 0x02,
            ContinuityClass::SymbolicContinuity => 0x01,
            ContinuityClass::Discontinuous => 0x00,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvolutionRecord {
    pub parent_constitution: [u8; 32],
    pub child_constitution: [u8; 32],
    pub evolution_proof: [u8; 32],
    pub compatibility: CompatibilityClass,
    pub epoch: u64,
    pub generation: u64,
    pub record_hash: [u8; 32],
}

impl EvolutionRecord {
    pub fn new(
        parent_constitution: [u8; 32],
        child_constitution: [u8; 32],
        compatibility: CompatibilityClass,
        epoch: u64,
        generation: u64,
    ) -> Self {
        let mut record = Self {
            parent_constitution,
            child_constitution,
            evolution_proof: [0u8; 32],
            compatibility,
            epoch,
            generation,
            record_hash: [0u8; 32],
        };
        record.record_hash = record.compute_hash();
        record
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(EVOLUTION_DOMAIN);
        h.update(&self.parent_constitution);
        h.update(&self.child_constitution);
        h.update(&self.evolution_proof);
        h.update(&[self.compatibility.canonical_tag()]);
        h.update(&self.epoch.to_le_bytes());
        h.update(&self.generation.to_le_bytes());
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.record_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvolutionProof {
    pub parent_constitution: [u8; 32],
    pub child_constitution: [u8; 32],
    pub replay_guarantee: ReplayGuarantee,
    pub snapshot_guarantee: SnapshotGuarantee,
    pub proof_guarantee: ProofGuarantee,
    pub governance_guarantee: GovernanceGuarantee,
    pub continuity_class: ContinuityClass,
    pub proof_hash: [u8; 32],
}

impl EvolutionProof {
    pub fn new(
        parent: [u8; 32],
        child: [u8; 32],
        replay: ReplayGuarantee,
        snapshot: SnapshotGuarantee,
        proof: ProofGuarantee,
        governance: GovernanceGuarantee,
        continuity: ContinuityClass,
    ) -> Self {
        let mut ep = Self {
            parent_constitution: parent,
            child_constitution: child,
            replay_guarantee: replay,
            snapshot_guarantee: snapshot,
            proof_guarantee: proof,
            governance_guarantee: governance,
            continuity_class: continuity,
            proof_hash: [0u8; 32],
        };
        ep.proof_hash = ep.compute_hash();
        ep
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(EVOLUTION_DOMAIN);
        h.update(&self.parent_constitution);
        h.update(&self.child_constitution);
        h.update(&[self.replay_guarantee.canonical_tag()]);
        h.update(&[self.snapshot_guarantee.canonical_tag()]);
        h.update(&[self.proof_guarantee.canonical_tag()]);
        h.update(&[self.governance_guarantee.canonical_tag()]);
        h.update(&[self.continuity_class.canonical_tag()]);
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.proof_hash
    }
}
