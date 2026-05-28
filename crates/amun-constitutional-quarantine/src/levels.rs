use amun_civilizational_relations::relation::CivilizationalRelation;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuarantineLevel {
    None = 0,
    Observation = 1,
    IsolatedVerification = 2,
    ConditionalAccess = 3,
    FullQuarantine = 4,
    PermanentSeparation = 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineZone {
    pub level: QuarantineLevel,
    pub source_relation: CivilizationalRelation,
    pub admitted_snapshots: Vec<[u8; 32]>,
    pub verification_results: Vec<VerificationRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationRecord {
    pub snapshot_root: [u8; 32],
    pub physics_verified: bool,
    pub replay_verified: bool,
    pub lineage_verified: bool,
    pub released: bool,
}
