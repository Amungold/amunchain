//! CausalEdge — a verified constitutional causal link between two artifacts.
//!
//! A CausalEdge answers: WHY does target constitutionally depend on source?
//!
//! DISTINCTION from ArtifactEdge (PHASE 52):
//!   - ArtifactEdge: structural relationship (evidence→commitment, etc.)
//!   - CausalEdge: constitutional dependency (WHY target needs source)

use crate::causality_type::CausalityType;
use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;

/// A verified causal link: source → target with explicit causality type.
///
/// INVARIANT: The causality type determines whether target is constitutionally
/// valid without source. Hard dependencies (ConstitutionalDependency) mean
/// target is INVALID without source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalEdge {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub edge_id: u64,
    pub edge_hash: ConstitutionalHash,

    /// The artifact that is depended upon.
    pub source_hash: ConstitutionalHash,

    /// The artifact that depends on the source.
    pub target_hash: ConstitutionalHash,

    /// WHY target depends on source.
    pub causality_type: CausalityType,

    /// The context where this causal relationship holds.
    pub context_hash: ConstitutionalHash,
}

impl ConstitutionalIdentity for CausalEdge {
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

impl ConstitutionalObject for CausalEdge {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"CAUSAL_EDGE")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.edge_id)
            .update_bytes(&self.source_hash)
            .update_bytes(&self.target_hash)
            .update_u8(self.causality_type as u8)
            .update_bytes(&self.context_hash);
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0014 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.edge_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid causal edge schema",
            ));
        }
        if self.source_hash == self.target_hash {
            return Err(ConstitutionalFailure::new(
                self.edge_id,
                failure_type::INVARIANT_BROKEN,
                failure_domain::CONSTITUTIONAL,
                severity::HARD_FAILURE,
                "Self-referential causal edge",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.edge_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.edge_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Causal edge hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.edge_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }
}

impl CausalEdge {
    pub fn new(
        edge_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        source_hash: ConstitutionalHash,
        target_hash: ConstitutionalHash,
        causality_type: CausalityType,
        context_hash: ConstitutionalHash,
    ) -> Self {
        let mut e = Self {
            schema_id: 0x0014,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            edge_id,
            edge_hash: [0; 32],
            source_hash,
            target_hash,
            causality_type,
            context_hash,
        };
        e.edge_hash = e.constitutional_hash();
        e
    }

    pub fn is_hard_dependency(&self) -> bool {
        self.causality_type.is_hard_dependency()
    }
    pub fn is_constitutional(&self) -> bool {
        self.causality_type.is_constitutional_dependency()
    }
    pub fn is_non_causal(&self) -> bool {
        self.causality_type.is_non_causal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_edge_verifies() {
        let e = CausalEdge::new(
            1,
            1,
            1,
            [0xAA; 32],
            [0xBB; 32],
            CausalityType::StateDerivation,
            [0xAB; 32],
        );
        assert!(e.verify().is_ok());
    }
    #[test]
    fn test_self_referential_rejected() {
        let e = CausalEdge::new(
            1,
            1,
            1,
            [0xAA; 32],
            [0xAA; 32],
            CausalityType::AdmissibilityCause,
            [0xAB; 32],
        );
        assert!(e.verify_structure().is_err());
    }
    #[test]
    fn test_hard_dependency() {
        let e = CausalEdge::new(
            1,
            1,
            1,
            [0xAA; 32],
            [0xBB; 32],
            CausalityType::ConstitutionalDependency,
            [0xAB; 32],
        );
        assert!(e.is_hard_dependency());
    }
    #[test]
    fn test_non_causal() {
        let e = CausalEdge::new(
            1,
            1,
            1,
            [0xAA; 32],
            [0xBB; 32],
            CausalityType::AncestralOnly,
            [0xAB; 32],
        );
        assert!(e.is_non_causal());
    }
    #[test]
    fn test_hash_deterministic() {
        let e1 = CausalEdge::new(
            1,
            1,
            1,
            [0xAA; 32],
            [0xBB; 32],
            CausalityType::StateDerivation,
            [0xAB; 32],
        );
        let e2 = CausalEdge::new(
            1,
            1,
            1,
            [0xAA; 32],
            [0xBB; 32],
            CausalityType::StateDerivation,
            [0xAB; 32],
        );
        assert_eq!(e1.edge_hash, e2.edge_hash);
    }
}
