//! ContinuationChain — links a snapshot to resumed execution.
//!
//! The continuation chain verifies that the post-restoration replay
//! is a constitutional CONTINUATION of the pre-restoration lineage.
//!
//! INVARIANT (Restoration Continuation):
//!   Restoration does not create a new replay lineage. The chain
//!   verifies that the resumed execution continues from the exact
//!   transcript position where the snapshot ended, preserving
//!   lineage continuity.

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;
use crate::restoration_point::RestorationPoint;

/// Verifies that post-restoration execution continues the pre-restoration lineage.
///
/// The continuation chain binds:
///   - The restoration point (where replay resumes)
///   - The first journal entry after restoration
///   - The boundary inherited at the restoration point
///
/// This creates a verifiable link across the restoration boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuationChain {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub chain_id: u64,
    pub chain_hash: ConstitutionalHash,

    /// The restoration point this chain continues from.
    pub restoration_point_hash: ConstitutionalHash,

    /// Hash of the first journal entry after restoration.
    /// This MUST have previous_entry_hash == restoration_point.preceding_entry_hash.
    pub first_entry_after_restoration: ConstitutionalHash,

    /// Hash of the first receipt produced after restoration.
    pub first_receipt_after_restoration: ConstitutionalHash,

    /// The context that continues.
    pub context_hash: ConstitutionalHash,

    /// The boundary inherited at the restoration point.
    pub inherited_boundary_hash: ConstitutionalHash,

    /// Previous continuation chain (for multi-hop restoration).
    pub parent_chain_hash: Option<ConstitutionalHash>,
}

impl ConstitutionalIdentity for ContinuationChain {
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

impl ConstitutionalObject for ContinuationChain {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"CONTINUATION_CHAIN")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.chain_id)
            .update_bytes(&self.restoration_point_hash)
            .update_bytes(&self.first_entry_after_restoration)
            .update_bytes(&self.first_receipt_after_restoration)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.inherited_boundary_hash)
            .update_optional_hash(self.parent_chain_hash.as_ref());
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0011 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.chain_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid continuation chain schema",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.chain_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.chain_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Continuation chain hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.chain_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }
}

impl ContinuationChain {
    pub fn new(
        chain_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        restoration_point: &RestorationPoint,
        first_entry_hash: ConstitutionalHash,
        first_receipt_hash: ConstitutionalHash,
        context_hash: ConstitutionalHash,
        inherited_boundary_hash: ConstitutionalHash,
        parent_chain_hash: Option<ConstitutionalHash>,
    ) -> Self {
        let mut c = Self {
            schema_id: 0x0011,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            chain_id,
            chain_hash: [0; 32],
            restoration_point_hash: restoration_point.point_hash,
            first_entry_after_restoration: first_entry_hash,
            first_receipt_after_restoration: first_receipt_hash,
            context_hash,
            inherited_boundary_hash,
            parent_chain_hash,
        };
        c.chain_hash = c.constitutional_hash();
        c
    }

    /// Verify that this chain continues from a given restoration point.
    /// The context and boundary must match.
    pub fn verify_continuation(
        &self,
        point: &RestorationPoint,
    ) -> Result<(), ConstitutionalFailure> {
        if self.restoration_point_hash != point.point_hash {
            return Err(ConstitutionalFailure::new(
                self.chain_id,
                failure_type::HASH_MISMATCH,
                failure_domain::REPLAY,
                severity::HARD_FAILURE,
                "Chain does not reference the given restoration point",
            ));
        }
        if self.context_hash != point.context_hash {
            return Err(ConstitutionalFailure::new(
                self.chain_id,
                failure_type::BOUNDARY_VIOLATION,
                failure_domain::REPLAY,
                severity::HARD_FAILURE,
                "Context mismatch in continuation chain",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::ConstitutionalSnapshot;
    use crate::snapshot_scope::RestorationOutcome;

    fn make_restoration_point() -> RestorationPoint {
        let s = ConstitutionalSnapshot::new(
            1,
            1,
            1,
            0,
            99,
            [0x11; 32],
            [0xAB; 32],
            [0xBC; 32],
            RestorationOutcome::Admissible,
            [0xCD; 32],
            None,
        );
        RestorationPoint::new(1, 1, 1, &s, [0xAB; 32], [0xBC; 32], Some([0xEE; 32]), None)
    }

    #[test]
    fn test_chain_verifies() {
        let rp = make_restoration_point();
        let chain = ContinuationChain::new(
            1, 1, 1, &rp, [0xAA; 32], [0xBB; 32], [0xAB; 32], [0xBC; 32], None,
        );
        assert!(chain.verify().is_ok());
    }
    #[test]
    fn test_verify_continuation_ok() {
        let rp = make_restoration_point();
        let chain = ContinuationChain::new(
            1, 1, 1, &rp, [0xAA; 32], [0xBB; 32], [0xAB; 32], [0xBC; 32], None,
        );
        assert!(chain.verify_continuation(&rp).is_ok());
    }
    #[test]
    fn test_context_mismatch_rejected() {
        let rp = make_restoration_point();
        let chain = ContinuationChain::new(
            1, 1, 1, &rp, [0xAA; 32], [0xBB; 32], [0xCD; 32], [0xBC; 32], None,
        );
        assert!(chain.verify_continuation(&rp).is_err());
    }
    #[test]
    fn test_hash_deterministic() {
        let rp = make_restoration_point();
        let c1 = ContinuationChain::new(
            1, 1, 1, &rp, [0xAA; 32], [0xBB; 32], [0xAB; 32], [0xBC; 32], None,
        );
        let c2 = ContinuationChain::new(
            1, 1, 1, &rp, [0xAA; 32], [0xBB; 32], [0xAB; 32], [0xBC; 32], None,
        );
        assert_eq!(c1.chain_hash, c2.chain_hash);
    }
}
