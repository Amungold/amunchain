//! Revision Migration — constitutional evolution law.
//!
//! Defines how constitutional truth survives revision boundaries.
//! When the constitution changes, artifacts from the old revision
//! must have a defined relationship to the new revision.
//!
//! This is NOT automatic versioning — it is constitutional law.

use crate::kernel_types::ConstitutionalHash;

/// The compatibility relationship between two constitutional revisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevisionCompatibility {
    /// Artifacts from old revision are fully valid in new revision.
    /// No migration needed.
    FullyCompatible,

    /// Artifacts from old revision are valid but require migration
    /// (e.g., re-hashing with new domain tags).
    MigrationRequired,

    /// Artifacts from old revision are NOT valid in new revision.
    /// Explicit governance action required for each artifact.
    Incompatible,

    /// Old revision artifacts are accepted as historical only.
    /// They may be referenced but not used for new proof generation.
    HistoricalOnly,
}

/// A migration path between two constitutional revisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationPath {
    /// The source revision.
    pub from_revision: u32,
    /// The target revision.
    pub to_revision: u32,
    /// The compatibility relationship.
    pub compatibility: RevisionCompatibility,
    /// Hash of the governance action that authorized this migration.
    pub governance_action_hash: ConstitutionalHash,
    /// Whether backward verification is supported.
    pub backward_verification: bool,
}

impl MigrationPath {
    pub fn new(
        from_revision: u32,
        to_revision: u32,
        compatibility: RevisionCompatibility,
        governance_action_hash: ConstitutionalHash,
    ) -> Self {
        Self {
            from_revision,
            to_revision,
            compatibility,
            governance_action_hash,
            backward_verification: false,
        }
    }

    /// Returns true if artifacts from the source revision are usable
    /// in the target revision.
    pub fn is_usable(&self) -> bool {
        matches!(
            self.compatibility,
            RevisionCompatibility::FullyCompatible | RevisionCompatibility::MigrationRequired
        )
    }

    /// Returns true if this is a breaking change.
    pub fn is_breaking(&self) -> bool {
        matches!(self.compatibility, RevisionCompatibility::Incompatible)
    }

    /// Enable backward verification.
    pub fn with_backward_verification(mut self) -> Self {
        self.backward_verification = true;
        self
    }
}

/// Proof that an artifact has been upgraded across a revision boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradeProof {
    /// The artifact hash before migration.
    pub original_hash: ConstitutionalHash,
    /// The artifact hash after migration.
    pub migrated_hash: ConstitutionalHash,
    /// The migration path used.
    pub migration_path_hash: ConstitutionalHash,
    /// Whether the upgrade was verified.
    pub verified: bool,
}

impl UpgradeProof {
    pub fn new(
        original_hash: ConstitutionalHash,
        migrated_hash: ConstitutionalHash,
        migration_path_hash: ConstitutionalHash,
    ) -> Self {
        Self {
            original_hash,
            migrated_hash,
            migration_path_hash,
            verified: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_compatible_is_usable() {
        let path = MigrationPath::new(1, 2, RevisionCompatibility::FullyCompatible, [0xAA; 32]);
        assert!(path.is_usable());
        assert!(!path.is_breaking());
    }

    #[test]
    fn test_incompatible_is_breaking() {
        let path = MigrationPath::new(1, 2, RevisionCompatibility::Incompatible, [0xAA; 32]);
        assert!(!path.is_usable());
        assert!(path.is_breaking());
    }

    #[test]
    fn test_upgrade_proof() {
        let proof = UpgradeProof::new([0x01; 32], [0x02; 32], [0x03; 32]);
        assert!(proof.verified);
        assert_eq!(proof.original_hash, [0x01; 32]);
        assert_eq!(proof.migrated_hash, [0x02; 32]);
    }
}
