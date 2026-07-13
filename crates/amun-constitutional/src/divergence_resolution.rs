//! DivergenceResolution — how a constitutional divergence was resolved.
//!
//! Records the resolution of a divergence: which lineage was accepted,
//! which was superseded, and under what constitutional authority.

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;

/// How a divergence was resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionType {
    /// The expected lineage was accepted; observed was rejected.
    ExpectedAccepted = 0x01,
    /// The observed lineage was accepted; expected was superseded.
    ObservedAccepted = 0x02,
    /// Both lineages are retained as constitutional forks.
    DualAcceptance = 0x03,
    /// Divergence is still unresolved.
    Unresolved = 0x04,
}

impl ResolutionType {
    pub fn type_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"RESOLUTION_TYPE").update_u8(*self as u8);
        h.finalize()
    }
}

/// The resolution of a constitutional divergence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivergenceResolution {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub resolution_id: u64,
    pub resolution_hash: ConstitutionalHash,

    /// The divergence point being resolved.
    pub divergence_hash: ConstitutionalHash,

    /// How the divergence was resolved.
    pub resolution_type: ResolutionType,

    /// The accepted lineage after resolution.
    pub accepted_lineage_hash: ConstitutionalHash,

    /// The superseded lineage (if any).
    pub superseded_lineage_hash: Option<ConstitutionalHash>,

    /// The context where resolution occurred.
    pub context_hash: ConstitutionalHash,

    /// Reference to a constitutional failure if resolution involved one.
    pub failure_hash: Option<ConstitutionalHash>,

    /// Previous resolution in this lineage.
    pub parent_resolution_hash: Option<ConstitutionalHash>,
}

impl ConstitutionalIdentity for DivergenceResolution {
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

impl ConstitutionalObject for DivergenceResolution {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"DIVERGENCE_RESOLUTION")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.resolution_id)
            .update_bytes(&self.divergence_hash)
            .update_u8(self.resolution_type as u8)
            .update_bytes(&self.accepted_lineage_hash)
            .update_optional_hash(self.superseded_lineage_hash.as_ref())
            .update_bytes(&self.context_hash)
            .update_optional_hash(self.failure_hash.as_ref())
            .update_optional_hash(self.parent_resolution_hash.as_ref());
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0013 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.resolution_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid resolution schema",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.resolution_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.resolution_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Resolution hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.resolution_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }
}

impl DivergenceResolution {
    pub fn new(
        resolution_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        divergence_hash: ConstitutionalHash,
        resolution_type: ResolutionType,
        accepted_lineage_hash: ConstitutionalHash,
        superseded_lineage_hash: Option<ConstitutionalHash>,
        context_hash: ConstitutionalHash,
        failure_hash: Option<ConstitutionalHash>,
        parent_resolution_hash: Option<ConstitutionalHash>,
    ) -> Self {
        let mut r = Self {
            schema_id: 0x0013,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            resolution_id,
            resolution_hash: [0; 32],
            divergence_hash,
            resolution_type,
            accepted_lineage_hash,
            superseded_lineage_hash,
            context_hash,
            failure_hash,
            parent_resolution_hash,
        };
        r.resolution_hash = r.constitutional_hash();
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resolution_verifies() {
        let r = DivergenceResolution::new(
            1,
            1,
            1,
            [0xAA; 32],
            ResolutionType::ExpectedAccepted,
            [0xBB; 32],
            Some([0xCC; 32]),
            [0xAB; 32],
            None,
            None,
        );
        assert!(r.verify().is_ok());
    }
    #[test]
    fn test_hash_deterministic() {
        let r1 = DivergenceResolution::new(
            1,
            1,
            1,
            [0xAA; 32],
            ResolutionType::ExpectedAccepted,
            [0xBB; 32],
            Some([0xCC; 32]),
            [0xAB; 32],
            None,
            None,
        );
        let r2 = DivergenceResolution::new(
            1,
            1,
            1,
            [0xAA; 32],
            ResolutionType::ExpectedAccepted,
            [0xBB; 32],
            Some([0xCC; 32]),
            [0xAB; 32],
            None,
            None,
        );
        assert_eq!(r1.resolution_hash, r2.resolution_hash);
    }
}
