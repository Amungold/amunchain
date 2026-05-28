use super::lineage::LineageProof;
use super::migration::MigrationWitness;

/// A CompatibilityTheorem proves that two protocol versions can
/// interoperate under specific constraints.
#[derive(Debug, Clone)]
pub struct CompatibilityTheorem {
    pub source_version: u32,
    pub target_version: u32,
    pub lineage_proof: Option<LineageProof>,
    pub migration_witness: Option<MigrationWitness>,
    pub is_state_compatible: bool,
    pub is_replay_compatible: bool,
    pub is_proof_compatible: bool,
    pub theorem_hash: [u8; 32],
}

impl CompatibilityTheorem {
    pub fn new(
        source_version: u32,
        target_version: u32,
        lineage_proof: Option<LineageProof>,
        migration_witness: Option<MigrationWitness>,
    ) -> Self {
        let is_state = migration_witness
            .as_ref()
            .map(|w| w.identity_preserved)
            .unwrap_or(false);
        let is_replay = migration_witness
            .as_ref()
            .map(|w| w.replay_preserved)
            .unwrap_or(false);
        let is_proof = lineage_proof
            .as_ref()
            .map(|p| p.is_verified)
            .unwrap_or(false);

        let mut theorem = Self {
            source_version,
            target_version,
            lineage_proof,
            migration_witness,
            is_state_compatible: is_state,
            is_replay_compatible: is_replay,
            is_proof_compatible: is_proof,
            theorem_hash: [0u8; 32],
        };
        theorem.theorem_hash = theorem.compute_hash();
        theorem
    }

    fn compute_hash(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(b"AMUN_COMPATIBILITY_THEOREM_V1");
        h.update(self.source_version.to_be_bytes());
        h.update(self.target_version.to_be_bytes());
        h.update([self.is_state_compatible as u8]);
        h.update([self.is_replay_compatible as u8]);
        h.update([self.is_proof_compatible as u8]);
        h.finalize().into()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.theorem_hash
    }
}

/// CompatibilityVerdict determines if a cross-protocol operation is lawful.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityVerdict {
    /// Fully compatible - no migration needed
    Identical,
    /// Lawful descendant - migration with proof
    LawfulDescendant,
    /// Compatible with migration witness
    MigrationRequired,
    /// Compatible but replay not preserved
    PartialCompatibility,
    /// Incompatible - no lawful relationship
    Incompatible,
}

impl CompatibilityVerdict {
    pub fn determine(theorem: &CompatibilityTheorem) -> Self {
        if theorem.source_version == theorem.target_version {
            return CompatibilityVerdict::Identical;
        }
        if theorem.is_proof_compatible
            && theorem.is_replay_compatible
            && theorem.is_state_compatible
        {
            return CompatibilityVerdict::LawfulDescendant;
        }
        if theorem.is_state_compatible && theorem.lineage_proof.is_some() {
            return CompatibilityVerdict::MigrationRequired;
        }
        if theorem.is_state_compatible {
            return CompatibilityVerdict::PartialCompatibility;
        }
        CompatibilityVerdict::Incompatible
    }
}
