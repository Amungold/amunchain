//! ConstitutionalWitness — a formally sufficient constitutional proof surface.
//!
//! A witness is NOT a bag of artifacts. It is a MINIMAL, CAUSALLY CLOSED,
//! CONSTITUTIONALLY SUFFICIENT set of artifacts that proves admissibility.
//!
//! INVARIANT: Every artifact in the witness has a defined WitnessType.
//! No artifact is included without constitutional justification.
//! Every omission is intentional and does not affect proof sufficiency.

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;
use crate::witness_type::WitnessType;

/// An entry in a constitutional witness — an artifact with its proof role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessEntry {
    /// Hash of the artifact included in the witness.
    pub artifact_hash: ConstitutionalHash,
    /// The proof role of this artifact.
    pub witness_type: WitnessType,
    /// Why this artifact is included (constitutional justification).
    pub inclusion_rationale: Option<Vec<u8>>,
}

impl WitnessEntry {
    pub fn new(artifact_hash: ConstitutionalHash, witness_type: WitnessType) -> Self {
        Self {
            artifact_hash,
            witness_type,
            inclusion_rationale: None,
        }
    }

    pub fn hard(artifact_hash: ConstitutionalHash) -> Self {
        Self::new(artifact_hash, WitnessType::HardDependency)
    }

    pub fn supporting(artifact_hash: ConstitutionalHash) -> Self {
        Self::new(artifact_hash, WitnessType::SupportingDependency)
    }

    pub fn audit(artifact_hash: ConstitutionalHash) -> Self {
        Self::new(artifact_hash, WitnessType::AuditDependency)
    }

    pub fn elidable(artifact_hash: ConstitutionalHash) -> Self {
        Self::new(artifact_hash, WitnessType::CompressionElidable)
    }

    pub fn is_required(&self) -> bool {
        self.witness_type.is_required()
    }
}

/// A constitutional witness — a minimal, causally closed proof surface.
///
/// The witness answers: "Given these artifacts, is admissibility provable?"
/// It does NOT answer: "What is the complete history?"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalWitness {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub witness_id: u64,
    pub witness_hash: ConstitutionalHash,

    /// The entries in this witness, ordered canonically.
    pub entries: Vec<WitnessEntry>,

    /// The terminal artifact this witness proves admissibility for.
    pub target_artifact_hash: ConstitutionalHash,

    /// The context this witness belongs to.
    pub context_hash: ConstitutionalHash,

    /// Count of hard dependencies (for quick validity checks).
    pub hard_dependency_count: u64,

    /// Count of all entries.
    pub total_entry_count: u64,
}

impl ConstitutionalIdentity for ConstitutionalWitness {
    fn schema_id(&self) -> u16 {
        self.schema_id
    }
    fn schema_version(&self) -> u16 {
        self.schema_version
    }
    fn constitutional_revision(&self) -> u32 {
        self.constitutional_revision
    }
    fn replay_revision(&self) -> u32 {
        self.replay_revision
    }
}

impl ConstitutionalObject for ConstitutionalWitness {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"CONSTITUTIONAL_WITNESS")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.witness_id)
            .update_bytes(&self.target_artifact_hash)
            .update_bytes(&self.context_hash)
            .update_u64(self.hard_dependency_count)
            .update_u64(self.total_entry_count);
        // Entries are hashed in canonical order for determinism
        for entry in &self.entries {
            h.update_bytes(&entry.artifact_hash)
                .update_u8(entry.witness_type as u8);
        }
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0015 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.witness_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid witness schema",
            ));
        }
        if self.entries.is_empty() {
            return Err(ConstitutionalFailure::new(
                self.witness_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Witness must contain at least one entry",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.witness_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.witness_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Witness hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.witness_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }

    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
        // Verify hard dependency count matches actual entries
        let actual_hard = self.entries.iter().filter(|e| e.is_required()).count() as u64;
        if actual_hard != self.hard_dependency_count {
            return Err(ConstitutionalFailure::new(
                self.witness_id,
                failure_type::INVARIANT_BROKEN,
                failure_domain::CONSTITUTIONAL,
                severity::HARD_FAILURE,
                "Hard dependency count mismatch",
            ));
        }
        if self.total_entry_count != self.entries.len() as u64 {
            return Err(ConstitutionalFailure::new(
                self.witness_id,
                failure_type::INVARIANT_BROKEN,
                failure_domain::CONSTITUTIONAL,
                severity::HARD_FAILURE,
                "Entry count mismatch",
            ));
        }
        Ok(())
    }
}

impl ConstitutionalWitness {
    pub fn new(
        witness_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        target_artifact_hash: ConstitutionalHash,
        context_hash: ConstitutionalHash,
        entries: Vec<WitnessEntry>,
    ) -> Self {
        let hard_count = entries.iter().filter(|e| e.is_required()).count() as u64;
        let total = entries.len() as u64;
        let mut w = Self {
            schema_id: 0x0015,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            witness_id,
            witness_hash: [0; 32],
            entries,
            target_artifact_hash,
            context_hash,
            hard_dependency_count: hard_count,
            total_entry_count: total,
        };
        w.witness_hash = w.constitutional_hash();
        w
    }

    /// Returns true if this witness contains at least all hard dependencies.
    /// A witness with zero hard dependencies is valid only if it proves
    /// something that requires no external dependencies.
    pub fn has_all_hard_dependencies(&self) -> bool {
        // A witness is sufficient if it contains all hard dependencies.
        // For now, we verify that the count is consistent.
        // Full closure check requires causal graph traversal (future).
        self.hard_dependency_count > 0 || self.entries.iter().all(|e| !e.is_required())
    }

    /// Returns the minimal witness — only hard dependencies.
    pub fn minimal_subset(&self) -> Vec<&WitnessEntry> {
        self.entries.iter().filter(|e| e.is_required()).collect()
    }

    /// Returns the compressible witness — without compression-elidable entries.
    pub fn compressed_subset(&self) -> Vec<&WitnessEntry> {
        self.entries
            .iter()
            .filter(|e| !e.witness_type.is_elidable())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_verifies() {
        let entries = vec![
            WitnessEntry::hard([0xAA; 32]),
            WitnessEntry::supporting([0xBB; 32]),
            WitnessEntry::audit([0xCC; 32]),
        ];
        let w = ConstitutionalWitness::new(1, 1, 1, [0xDD; 32], [0xAB; 32], entries);
        assert!(w.verify().is_ok());
        assert_eq!(w.hard_dependency_count, 1);
        assert_eq!(w.total_entry_count, 3);
    }

    #[test]
    fn test_minimal_subset() {
        let entries = vec![
            WitnessEntry::hard([0xAA; 32]),
            WitnessEntry::hard([0xBB; 32]),
            WitnessEntry::supporting([0xCC; 32]),
            WitnessEntry::elidable([0xDD; 32]),
        ];
        let w = ConstitutionalWitness::new(1, 1, 1, [0xEE; 32], [0xAB; 32], entries);
        let minimal = w.minimal_subset();
        assert_eq!(minimal.len(), 2);
        assert!(minimal.iter().all(|e| e.is_required()));
    }

    #[test]
    fn test_compressed_subset() {
        let entries = vec![
            WitnessEntry::hard([0xAA; 32]),
            WitnessEntry::elidable([0xBB; 32]),
            WitnessEntry::elidable([0xCC; 32]),
        ];
        let w = ConstitutionalWitness::new(1, 1, 1, [0xDD; 32], [0xAB; 32], entries);
        let compressed = w.compressed_subset();
        assert_eq!(compressed.len(), 1);
        assert!(compressed[0].is_required());
    }

    #[test]
    fn test_empty_witness_rejected() {
        let w = ConstitutionalWitness::new(1, 1, 1, [0xDD; 32], [0xAB; 32], vec![]);
        assert!(w.verify_structure().is_err());
    }

    #[test]
    fn test_hard_count_mismatch_rejected() {
        let entries = vec![WitnessEntry::hard([0xAA; 32])];
        let mut w = ConstitutionalWitness::new(1, 1, 1, [0xDD; 32], [0xAB; 32], entries);
        w.hard_dependency_count = 99;
        w.witness_hash = w.constitutional_hash();
        assert!(w.verify_constitutional().is_err());
    }

    #[test]
    fn test_hash_deterministic() {
        let e1 = vec![
            WitnessEntry::hard([0xAA; 32]),
            WitnessEntry::supporting([0xBB; 32]),
        ];
        let w1 = ConstitutionalWitness::new(1, 1, 1, [0xDD; 32], [0xAB; 32], e1);
        let e2 = vec![
            WitnessEntry::hard([0xAA; 32]),
            WitnessEntry::supporting([0xBB; 32]),
        ];
        let w2 = ConstitutionalWitness::new(1, 1, 1, [0xDD; 32], [0xAB; 32], e2);
        assert_eq!(w1.witness_hash, w2.witness_hash);
    }

    #[test]
    fn test_order_affects_hash() {
        let e1 = vec![
            WitnessEntry::hard([0xAA; 32]),
            WitnessEntry::supporting([0xBB; 32]),
        ];
        let e2 = vec![
            WitnessEntry::supporting([0xBB; 32]),
            WitnessEntry::hard([0xAA; 32]),
        ];
        let w1 = ConstitutionalWitness::new(1, 1, 1, [0xDD; 32], [0xAB; 32], e1);
        let w2 = ConstitutionalWitness::new(1, 1, 1, [0xDD; 32], [0xAB; 32], e2);
        assert_ne!(w1.witness_hash, w2.witness_hash);
    }
}
