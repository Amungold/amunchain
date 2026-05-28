//! ConstitutionalStateAnchor — immutable, replay-derived state attestation.
//!
//! ARCHITECTURAL PRINCIPLE:
//!   State anchoring is constitutional attestation, NOT runtime state management.
//!
//! A StateAnchor attests that a specific replay lineage, within a specific
//! transcript span, under specific constitutional revisions, deterministically
//! produced a specific state root.
//!
//! The anchor is CONTENT-ADDRESSED: its identity IS its hash.
//! It is REPLAY-DERIVED: the state root is an output of replay, not storage.
//!
//! INVARIANT (Replay-Derived State Identity):
//!   Constitutional state identity is derived exclusively from deterministic
//!   replay lineage and attested transcript scope, never from mutable storage
//!   state or runtime execution environment.

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;
use crate::state_anchor_scope::StateAnchorScope;

/// An immutable constitutional attestation of replay-derived state.
///
/// The anchor says: "Within this scope, replay produced this state root."
/// It does NOT say: "This is the current network state."
/// It does NOT contain: accounts, balances, trie nodes, or mutable data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalStateAnchor {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub anchor_id: u64,
    pub anchor_hash: ConstitutionalHash,

    /// The scope this anchor attests to.
    pub scope: StateAnchorScope,

    /// Certificate that attests to the admissibility of this state.
    pub certificate_hash: ConstitutionalHash,

    /// Journal root at the end of the attested span.
    pub journal_root: ConstitutionalHash,

    /// Previous anchor in this context's lineage.
    pub parent_anchor_hash: Option<ConstitutionalHash>,

    /// Informational note (CIR-001) — not in constitutional hash.
    pub attestation_note: Option<Vec<u8>>,
}

impl ConstitutionalIdentity for ConstitutionalStateAnchor {
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

impl ConstitutionalObject for ConstitutionalStateAnchor {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"STATE_ANCHOR")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.anchor_id)
            // Scope fields
            .update_u64(self.scope.transcript_start)
            .update_u64(self.scope.transcript_end)
            .update_bytes(&self.scope.state_root)
            .update_bytes(&self.scope.context_hash)
            .update_u32(self.scope.constitutional_revision)
            .update_u32(self.scope.replay_revision)
            .update_bytes(&self.scope.boundary_hash)
            // Anchoring
            .update_bytes(&self.certificate_hash)
            .update_bytes(&self.journal_root)
            // Lineage
            .update_optional_hash(self.parent_anchor_hash.as_ref());
        // attestation_note excluded per CIR-001
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x000E || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.anchor_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid state anchor schema",
            ));
        }
        if self.scope.transcript_end < self.scope.transcript_start {
            return Err(ConstitutionalFailure::new(
                self.anchor_id,
                failure_type::BOUNDARY_VIOLATION,
                failure_domain::BOUNDARY,
                severity::HARD_FAILURE,
                "Anchor scope end before start",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.anchor_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.anchor_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Anchor hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.anchor_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }

    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
        if self.scope.constitutional_revision != self.constitutional_revision
            || self.scope.replay_revision != self.replay_revision
        {
            return Err(ConstitutionalFailure::new(
                self.anchor_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::CONSTITUTIONAL,
                severity::FATAL_FAILURE,
                "Scope revision mismatch",
            ));
        }
        Ok(())
    }
}

impl ConstitutionalStateAnchor {
    pub fn new(
        anchor_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        transcript_start: u64,
        transcript_end: u64,
        state_root: ConstitutionalHash,
        context_hash: ConstitutionalHash,
        boundary_hash: ConstitutionalHash,
        certificate_hash: ConstitutionalHash,
        journal_root: ConstitutionalHash,
        parent_anchor_hash: Option<ConstitutionalHash>,
    ) -> Self {
        let scope = StateAnchorScope {
            transcript_start,
            transcript_end,
            state_root,
            context_hash,
            constitutional_revision,
            replay_revision,
            boundary_hash,
        };
        let mut a = Self {
            schema_id: 0x000E,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            anchor_id,
            anchor_hash: [0; 32],
            scope,
            certificate_hash,
            journal_root,
            parent_anchor_hash,
            attestation_note: None,
        };
        a.anchor_hash = a.constitutional_hash();
        a
    }

    /// The state root this anchor attests to.
    pub fn state_root(&self) -> ConstitutionalHash {
        self.scope.state_root
    }

    /// Verify anchor monotonicity against a parent anchor.
    pub fn verify_against_parent(
        &self,
        parent: &ConstitutionalStateAnchor,
    ) -> Result<(), ConstitutionalFailure> {
        match self.scope.verify_against_parent(&parent.scope) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConstitutionalFailure::new(
                self.anchor_id,
                failure_type::BOUNDARY_VIOLATION,
                failure_domain::CONSTITUTIONAL,
                severity::HARD_FAILURE,
                "Anchor monotonicity violation",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ma(
        id: u64,
        start: u64,
        end: u64,
        root: [u8; 32],
        parent: Option<ConstitutionalHash>,
    ) -> ConstitutionalStateAnchor {
        ConstitutionalStateAnchor::new(
            id, 1, 1, start, end, root, [0xAB; 32], [0xBC; 32], [0xCD; 32], [0xDE; 32], parent,
        )
    }

    #[test]
    fn test_anchor_verifies() {
        assert!(ma(1, 0, 99, [0x11; 32], None).verify().is_ok());
    }
    #[test]
    fn test_hash_deterministic() {
        assert_eq!(
            ma(1, 0, 99, [0x11; 32], None).anchor_hash,
            ma(1, 0, 99, [0x11; 32], None).anchor_hash
        );
    }
    #[test]
    fn test_state_root_affects_hash() {
        assert_ne!(
            ma(1, 0, 99, [0x11; 32], None).anchor_hash,
            ma(1, 0, 99, [0x22; 32], None).anchor_hash
        );
    }
    #[test]
    fn test_span_affects_hash() {
        assert_ne!(
            ma(1, 0, 49, [0x11; 32], None).anchor_hash,
            ma(1, 0, 99, [0x11; 32], None).anchor_hash
        );
    }
    #[test]
    fn test_monotonicity_ok() {
        let p = ma(1, 0, 99, [0x11; 32], None);
        let c = ma(2, 0, 49, [0x11; 32], Some(p.anchor_hash));
        assert!(c.verify_against_parent(&p).is_ok());
    }
    #[test]
    fn test_state_transition_ok() {
        let p = ma(1, 0, 99, [0x11; 32], None);
        let c = ma(2, 0, 49, [0x22; 32], Some(p.anchor_hash));
        assert!(c.verify_against_parent(&p).is_ok());
    }
    #[test]
    fn test_monotonicity_violated() {
        let p = ma(1, 0, 99, [0x11; 32], None);
        let c = ConstitutionalStateAnchor::new(
            2,
            1,
            1,
            0,
            49,
            [0x11; 32],
            [0xCD; 32],
            [0xBC; 32],
            [0xCD; 32],
            [0xDE; 32],
            Some(p.anchor_hash),
        );
        assert!(c.verify_against_parent(&p).is_err());
    }
    #[test]
    fn test_invalid_span_rejected() {
        let mut a = ma(1, 100, 50, [0x11; 32], None);
        a.anchor_hash = a.constitutional_hash();
        assert!(a.verify_structure().is_err());
    }
    #[test]
    fn test_revision_mismatch_rejected() {
        let mut a = ma(1, 0, 99, [0x11; 32], None);
        a.scope.constitutional_revision = 2;
        a.anchor_hash = a.constitutional_hash();
        assert!(a.verify_constitutional().is_err());
    }
}
