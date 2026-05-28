//! ConstitutionalSnapshot — constitutionally restorable replay surface.
//!
//! A snapshot is NOT a backup. It is a constitutional restoration surface:
//!   "This replay lineage, within this scope, produced this state anchor,
//!    and restoration to this anchor is constitutionally admissible."
//!
//! Snapshot identity is REPLAY-DERIVED, not storage-derived.
//! Snapshot restoration validity is LINEAGE-DERIVED, not storage-derived.
//!
//! ARCHITECTURAL RELATIONSHIP:
//!   Replay → StateAnchor → Snapshot
//!   NOT: Database → Snapshot → Recovery

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;
use crate::prelude::*;
use crate::snapshot_scope::{RestorationOutcome, SnapshotScope};

/// A constitutionally restorable replay surface.
///
/// The snapshot attests that a specific state anchor, produced by a specific
/// replay lineage, is admissible for restoration under specific constitutional
/// conditions.
///
/// The snapshot does NOT contain:
///   - Serialized state data
///   - Account information
///   - Trie nodes
///   - Storage engine artifacts
///
/// It contains only the constitutional metadata required to verify
/// restoration admissibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalSnapshot {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub snapshot_id: u64,
    pub snapshot_hash: ConstitutionalHash,

    /// The restoration scope — defines when/where/how restoration is admissible.
    pub scope: SnapshotScope,

    /// The state anchor this snapshot restores to.
    pub anchor_hash: ConstitutionalHash,

    /// Certificate attesting admissibility of the anchor.
    pub certificate_hash: ConstitutionalHash,

    /// Previous snapshot in this context's lineage.
    pub parent_snapshot_hash: Option<ConstitutionalHash>,

    /// Informational note (CIR-001) — not in constitutional hash.
    pub restoration_note: Option<Vec<u8>>,
}

impl ConstitutionalIdentity for ConstitutionalSnapshot {
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

impl ConstitutionalObject for ConstitutionalSnapshot {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"SNAPSHOT")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.snapshot_id)
            // Scope fields
            .update_u64(self.scope.transcript_start)
            .update_u64(self.scope.transcript_end)
            .update_bytes(&self.scope.anchor_hash)
            .update_bytes(&self.scope.context_hash)
            .update_u32(self.scope.constitutional_revision)
            .update_u32(self.scope.replay_revision)
            .update_bytes(&self.scope.boundary_hash)
            .update_u8(self.scope.restoration_outcome as u8)
            // Anchoring
            .update_bytes(&self.anchor_hash)
            .update_bytes(&self.certificate_hash)
            // Lineage
            .update_optional_hash(self.parent_snapshot_hash.as_ref());
        // restoration_note excluded per CIR-001
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x000F || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.snapshot_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid snapshot schema",
            ));
        }
        if self.scope.transcript_end < self.scope.transcript_start {
            return Err(ConstitutionalFailure::new(
                self.snapshot_id,
                failure_type::BOUNDARY_VIOLATION,
                failure_domain::BOUNDARY,
                severity::HARD_FAILURE,
                "Snapshot scope end before start",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.snapshot_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.snapshot_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Snapshot hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.snapshot_id,
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
                self.snapshot_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::CONSTITUTIONAL,
                severity::FATAL_FAILURE,
                "Scope revision mismatch",
            ));
        }
        // Anchor hash in scope must match the snapshot's anchor_hash
        if self.scope.anchor_hash != self.anchor_hash {
            return Err(ConstitutionalFailure::new(
                self.snapshot_id,
                failure_type::HASH_MISMATCH,
                failure_domain::CONSTITUTIONAL,
                severity::FATAL_FAILURE,
                "Scope anchor hash mismatch",
            ));
        }
        Ok(())
    }
}

impl ConstitutionalSnapshot {
    pub fn new(
        snapshot_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        transcript_start: u64,
        transcript_end: u64,
        anchor_hash: ConstitutionalHash,
        context_hash: ConstitutionalHash,
        boundary_hash: ConstitutionalHash,
        restoration_outcome: RestorationOutcome,
        certificate_hash: ConstitutionalHash,
        parent_snapshot_hash: Option<ConstitutionalHash>,
    ) -> Self {
        let scope = SnapshotScope {
            transcript_start,
            transcript_end,
            anchor_hash,
            context_hash,
            constitutional_revision,
            replay_revision,
            boundary_hash,
            restoration_outcome,
        };
        let mut s = Self {
            schema_id: 0x000F,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            snapshot_id,
            snapshot_hash: [0; 32],
            scope,
            anchor_hash,
            certificate_hash,
            parent_snapshot_hash,
            restoration_note: None,
        };
        s.snapshot_hash = s.constitutional_hash();
        s
    }

    /// Returns true if this snapshot is admissible for restoration.
    pub fn is_restorable(&self) -> bool {
        self.scope.is_restorable()
    }

    /// Verify snapshot monotonicity against a parent snapshot.
    pub fn verify_against_parent(
        &self,
        parent: &ConstitutionalSnapshot,
    ) -> Result<(), ConstitutionalFailure> {
        match self.scope.verify_against_parent(&parent.scope) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConstitutionalFailure::new(
                self.snapshot_id,
                failure_type::BOUNDARY_VIOLATION,
                failure_domain::CONSTITUTIONAL,
                severity::HARD_FAILURE,
                "Snapshot monotonicity violation",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(
        id: u64,
        start: u64,
        end: u64,
        anchor: [u8; 32],
        outcome: RestorationOutcome,
        parent: Option<ConstitutionalHash>,
    ) -> ConstitutionalSnapshot {
        ConstitutionalSnapshot::new(
            id, 1, 1, start, end, anchor, [0xAB; 32], [0xBC; 32], outcome, [0xCD; 32], parent,
        )
    }

    #[test]
    fn test_snapshot_verifies() {
        assert!(
            ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None)
                .verify()
                .is_ok()
        );
    }
    #[test]
    fn test_hash_deterministic() {
        assert_eq!(
            ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None).snapshot_hash,
            ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None).snapshot_hash
        );
    }
    #[test]
    fn test_anchor_affects_hash() {
        assert_ne!(
            ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None).snapshot_hash,
            ms(1, 0, 99, [0x22; 32], RestorationOutcome::Admissible, None).snapshot_hash
        );
    }
    #[test]
    fn test_outcome_affects_hash() {
        assert_ne!(
            ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None).snapshot_hash,
            ms(
                1,
                0,
                99,
                [0x11; 32],
                RestorationOutcome::LineageDivergent,
                None
            )
            .snapshot_hash
        );
    }
    #[test]
    fn test_is_restorable() {
        assert!(ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None).is_restorable());
        assert!(!ms(
            1,
            0,
            99,
            [0x11; 32],
            RestorationOutcome::BoundaryViolation,
            None
        )
        .is_restorable());
    }
    #[test]
    fn test_monotonicity_ok() {
        let p = ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None);
        let c = ms(
            2,
            0,
            49,
            [0x11; 32],
            RestorationOutcome::Admissible,
            Some(p.snapshot_hash),
        );
        assert!(c.verify_against_parent(&p).is_ok());
    }
    #[test]
    fn test_monotonicity_violated() {
        let p = ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None);
        let c = ConstitutionalSnapshot::new(
            2,
            1,
            1,
            0,
            49,
            [0x11; 32],
            [0xCD; 32],
            [0xBC; 32],
            RestorationOutcome::Admissible,
            [0xCD; 32],
            Some(p.snapshot_hash),
        );
        assert!(c.verify_against_parent(&p).is_err());
    }
    #[test]
    fn test_scope_anchor_mismatch_rejected() {
        let mut s = ms(1, 0, 99, [0x11; 32], RestorationOutcome::Admissible, None);
        s.scope.anchor_hash = [0xFF; 32];
        s.snapshot_hash = s.constitutional_hash();
        assert!(s.verify_constitutional().is_err());
    }
}
