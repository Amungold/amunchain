//! DivergencePoint — the constitutional position where replay diverged.
//!
//! Records the exact transcript position, the type of divergence,
//! and the competing constitutional artifacts at the point of divergence.

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::divergence_type::DivergenceType;
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;

/// The point in the transcript where replay diverged.
///
/// Captures both sides of a divergence: the expected canonical artifact
/// and the observed divergent artifact. This enables formal analysis
/// of whether the divergence is constitutionally admissible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivergencePoint {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub divergence_id: u64,
    pub divergence_hash: ConstitutionalHash,

    /// The transcript position where divergence was detected.
    pub transcript_position: u64,

    /// The constitutional classification of this divergence.
    pub divergence_type: DivergenceType,

    /// The expected canonical artifact hash.
    pub expected_hash: ConstitutionalHash,

    /// The observed divergent artifact hash.
    pub observed_hash: ConstitutionalHash,

    /// The context where divergence occurred.
    pub context_hash: ConstitutionalHash,

    /// The boundary active at the divergence point.
    pub boundary_hash: ConstitutionalHash,

    /// Reference to a ConstitutionalFailure if this divergence produced one.
    pub failure_hash: Option<ConstitutionalHash>,

    /// Previous divergence point in this lineage.
    pub parent_divergence_hash: Option<ConstitutionalHash>,

    /// Informational description (CIR-001).
    pub description: Option<Vec<u8>>,
}

impl ConstitutionalIdentity for DivergencePoint {
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

impl ConstitutionalObject for DivergencePoint {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"DIVERGENCE_POINT")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.divergence_id)
            .update_u64(self.transcript_position)
            .update_u8(self.divergence_type as u8)
            .update_bytes(&self.expected_hash)
            .update_bytes(&self.observed_hash)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.boundary_hash)
            .update_optional_hash(self.failure_hash.as_ref())
            .update_optional_hash(self.parent_divergence_hash.as_ref());
        // description excluded per CIR-001
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0012 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.divergence_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid divergence point schema",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.divergence_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.divergence_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Divergence hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.divergence_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }
}

impl DivergencePoint {
    pub fn new(
        divergence_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        transcript_position: u64,
        divergence_type: DivergenceType,
        expected_hash: ConstitutionalHash,
        observed_hash: ConstitutionalHash,
        context_hash: ConstitutionalHash,
        boundary_hash: ConstitutionalHash,
        failure_hash: Option<ConstitutionalHash>,
        parent_divergence_hash: Option<ConstitutionalHash>,
    ) -> Self {
        let mut d = Self {
            schema_id: 0x0012,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            divergence_id,
            divergence_hash: [0; 32],
            transcript_position,
            divergence_type,
            expected_hash,
            observed_hash,
            context_hash,
            boundary_hash,
            failure_hash,
            parent_divergence_hash,
            description: None,
        };
        d.divergence_hash = d.constitutional_hash();
        d
    }

    pub fn is_admissible(&self) -> bool {
        self.divergence_type.is_admissible()
    }
    pub fn is_error(&self) -> bool {
        self.divergence_type.is_error()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divergence_verifies() {
        let d = DivergencePoint::new(
            1,
            1,
            1,
            100,
            DivergenceType::ConstitutionalFork,
            [0xAA; 32],
            [0xBB; 32],
            [0xAB; 32],
            [0xBC; 32],
            None,
            None,
        );
        assert!(d.verify().is_ok());
    }
    #[test]
    fn test_admissible_fork() {
        let d = DivergencePoint::new(
            1,
            1,
            1,
            100,
            DivergenceType::ConstitutionalFork,
            [0xAA; 32],
            [0xBB; 32],
            [0xAB; 32],
            [0xBC; 32],
            None,
            None,
        );
        assert!(d.is_admissible());
        assert!(!d.is_error());
    }
    #[test]
    fn test_replay_error() {
        let d = DivergencePoint::new(
            1,
            1,
            1,
            100,
            DivergenceType::ReplayError,
            [0xAA; 32],
            [0xBB; 32],
            [0xAB; 32],
            [0xBC; 32],
            None,
            None,
        );
        assert!(!d.is_admissible());
        assert!(d.is_error());
    }
    #[test]
    fn test_hash_deterministic() {
        let d1 = DivergencePoint::new(
            1,
            1,
            1,
            100,
            DivergenceType::ConstitutionalFork,
            [0xAA; 32],
            [0xBB; 32],
            [0xAB; 32],
            [0xBC; 32],
            None,
            None,
        );
        let d2 = DivergencePoint::new(
            1,
            1,
            1,
            100,
            DivergenceType::ConstitutionalFork,
            [0xAA; 32],
            [0xBB; 32],
            [0xAB; 32],
            [0xBC; 32],
            None,
            None,
        );
        assert_eq!(d1.divergence_hash, d2.divergence_hash);
    }
}
