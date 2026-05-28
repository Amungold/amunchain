use amun_lineage::compatibility::CompatibilityClass;
use amun_snapshot_engine::ConstitutionalIdentity;

/// Dimensional compatibility - each axis is independent.
#[derive(Debug, Clone)]
pub struct CompatibilityDimensions {
    pub replay_safe: bool,
    pub snapshot_safe: bool,
    pub proof_safe: bool,
    pub governance_safe: bool,
}

impl CompatibilityDimensions {
    /// Derive the effective compatibility class from dimensions.
    pub fn to_class(&self) -> CompatibilityClass {
        if !self.replay_safe && !self.snapshot_safe {
            CompatibilityClass::Hostile
        } else if !self.replay_safe && self.snapshot_safe {
            CompatibilityClass::SnapshotCompatible
        } else if self.replay_safe && self.proof_safe && self.governance_safe {
            CompatibilityClass::Full
        } else if self.replay_safe && self.proof_safe {
            CompatibilityClass::ReplayCompatible
        } else if self.replay_safe {
            CompatibilityClass::ReadOnly
        } else {
            CompatibilityClass::MigrationRequired
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompatibilityMatrix {
    pub source_identity: ConstitutionalIdentity,
    pub target_identity: ConstitutionalIdentity,
    pub dimensions: CompatibilityDimensions,
    pub effective_class: CompatibilityClass,
    pub can_sync: bool,
    pub can_migrate: bool,
    pub can_read: bool,
}

impl CompatibilityMatrix {
    pub fn compute(source: &ConstitutionalIdentity, target: &ConstitutionalIdentity) -> Self {
        let same_constitution = source.constitutional_hash == target.constitutional_hash;
        let same_empty_root = source.canonical_empty_root == target.canonical_empty_root;
        let same_max_depth = source.max_depth == target.max_depth;
        let same_proof = source.proof_version == target.proof_version;

        let dimensions = CompatibilityDimensions {
            replay_safe: same_empty_root && same_max_depth && same_proof,
            snapshot_safe: same_empty_root && same_max_depth,
            proof_safe: same_proof,
            governance_safe: same_constitution,
        };

        let effective_class = dimensions.to_class();

        Self {
            source_identity: source.clone(),
            target_identity: target.clone(),
            dimensions,
            effective_class,
            can_sync: effective_class.rank() >= CompatibilityClass::Full.rank(),
            can_migrate: effective_class.rank() >= CompatibilityClass::SnapshotCompatible.rank(),
            can_read: effective_class.rank() >= CompatibilityClass::ReadOnly.rank(),
        }
    }
}
