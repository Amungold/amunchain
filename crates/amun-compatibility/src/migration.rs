/// Rules governing state migration between constitutional versions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationRules {
    pub source_constitution: [u8; 32],
    pub target_constitution: [u8; 32],
    pub requires_replay: bool,
    pub requires_snapshot_rebuild: bool,
    pub requires_proof_rebuild: bool,
    pub estimated_cost_class: MigrationCostClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationCostClass {
    /// No migration needed - identical constitutions
    None,
    /// Light migration - format conversion only
    Light,
    /// Full migration - state must be replayed
    Full,
    /// Expensive migration - complete rebuild required
    Expensive,
    /// Impossible - cannot migrate
    Impossible,
}

/// Proof that a migration was performed correctly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationProof {
    pub source_root: [u8; 32],
    pub target_root: [u8; 32],
    pub migration_rules_hash: [u8; 32],
    pub proof_hash: [u8; 32],
}
