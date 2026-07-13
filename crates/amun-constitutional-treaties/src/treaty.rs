/// A constitutional treaty between two civilizations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalTreaty {
    pub treaty_id: [u8; 32],
    pub party_a: [u8; 32],
    pub party_b: [u8; 32],
    pub treaty_type: TreatyType,
    pub status: TreatyStatus,
    pub established_at_epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreatyType {
    /// Full synchronization allowed
    FullSync,
    /// Read-only access to state
    ReadOnlyAccess,
    /// Snapshot exchange permitted
    SnapshotExchange,
    /// Diplomatic channel only (identity exchange)
    DiplomaticChannel,
    /// Non-aggression pact (no hostile forks)
    NonAggression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreatyStatus {
    Proposed,
    Ratified { by_a: bool, by_b: bool },
    Active,
    Suspended { reason: String },
    Terminated { reason: String },
}
