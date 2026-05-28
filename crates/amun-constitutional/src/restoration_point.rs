//! RestorationPoint — the constitutional position where replay resumes.
//!
//! A restoration point defines exactly where in the replay lineage
//! execution continues after restoration from a snapshot.
//!
//! INVARIANT: Restoration does NOT create a new lineage.
//! It continues the existing lineage from the snapshot position.

use crate::constitutional_failure::{
    failure_domain, failure_type, severity, ConstitutionalFailure,
};
use crate::constitutional_hasher::ConstitutionalHasher;
use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;
use crate::snapshot::ConstitutionalSnapshot;

/// Where replay resumes after restoration.
///
/// The restoration point binds a snapshot to the exact transcript
/// position where execution continues. This ensures that the
/// post-restoration journal is a CONTINUATION of the pre-restoration
/// journal, not a new branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestorationPoint {
    pub schema_id: u16,
    pub schema_version: u16,
    pub constitutional_revision: u32,
    pub replay_revision: u32,

    pub point_id: u64,
    pub point_hash: ConstitutionalHash,

    /// The snapshot from which restoration occurs.
    pub snapshot_hash: ConstitutionalHash,

    /// The state anchor being restored to.
    pub anchor_hash: ConstitutionalHash,

    /// The transcript position where replay resumes.
    /// This is the NEXT position after the snapshot's transcript_end.
    pub resume_transcript_position: u64,

    /// The context that continues after restoration.
    pub context_hash: ConstitutionalHash,

    /// The boundary active at the restoration point.
    pub boundary_hash: ConstitutionalHash,

    /// The journal entry hash that immediately precedes restoration.
    /// This links the post-restoration journal to the pre-restoration journal.
    pub preceding_entry_hash: Option<ConstitutionalHash>,

    /// Previous restoration point in this lineage (for multi-hop restoration).
    pub parent_restoration_hash: Option<ConstitutionalHash>,
}

impl ConstitutionalIdentity for RestorationPoint {
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

impl ConstitutionalObject for RestorationPoint {
    fn constitutional_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"RESTORATION_POINT")
            .update_schema(self.schema_id, self.schema_version)
            .update_revision(self.constitutional_revision, self.replay_revision)
            .update_u64(self.point_id)
            .update_bytes(&self.snapshot_hash)
            .update_bytes(&self.anchor_hash)
            .update_u64(self.resume_transcript_position)
            .update_bytes(&self.context_hash)
            .update_bytes(&self.boundary_hash)
            .update_optional_hash(self.preceding_entry_hash.as_ref())
            .update_optional_hash(self.parent_restoration_hash.as_ref());
        h.finalize()
    }

    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
        if self.schema_id != 0x0010 || self.schema_version == 0 {
            return Err(ConstitutionalFailure::new(
                self.point_id,
                failure_type::INVALID_SCHEMA,
                failure_domain::STRUCTURAL,
                severity::HARD_FAILURE,
                "Invalid restoration point schema",
            ));
        }
        Ok(())
    }

    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
        if self.point_hash != self.constitutional_hash() {
            return Err(ConstitutionalFailure::new(
                self.point_id,
                failure_type::HASH_MISMATCH,
                failure_domain::SEMANTIC,
                severity::HARD_FAILURE,
                "Restoration point hash mismatch",
            ));
        }
        Ok(())
    }

    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
        if self.constitutional_revision == 0 || self.replay_revision == 0 {
            return Err(ConstitutionalFailure::new(
                self.point_id,
                failure_type::PROVENANCE_INVALID,
                failure_domain::PROVENANCE,
                severity::HARD_FAILURE,
                "Missing revision lineage",
            ));
        }
        Ok(())
    }
}

impl RestorationPoint {
    pub fn new(
        point_id: u64,
        constitutional_revision: u32,
        replay_revision: u32,
        snapshot: &ConstitutionalSnapshot,
        context_hash: ConstitutionalHash,
        boundary_hash: ConstitutionalHash,
        preceding_entry_hash: Option<ConstitutionalHash>,
        parent_restoration_hash: Option<ConstitutionalHash>,
    ) -> Self {
        let resume_position = snapshot.scope.transcript_end.saturating_add(1);
        let mut r = Self {
            schema_id: 0x0010,
            schema_version: 1,
            constitutional_revision,
            replay_revision,
            point_id,
            point_hash: [0; 32],
            snapshot_hash: snapshot.snapshot_hash,
            anchor_hash: snapshot.anchor_hash,
            resume_transcript_position: resume_position,
            context_hash,
            boundary_hash,
            preceding_entry_hash,
            parent_restoration_hash,
        };
        r.point_hash = r.constitutional_hash();
        r
    }

    /// Returns true if the snapshot is admissible for restoration.
    /// (Delegates to the snapshot's is_restorable check.)
    pub fn is_valid_restoration(&self, snapshot: &ConstitutionalSnapshot) -> bool {
        snapshot.is_restorable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::ConstitutionalSnapshot;
    use crate::snapshot_scope::RestorationOutcome;

    fn make_snapshot() -> ConstitutionalSnapshot {
        ConstitutionalSnapshot::new(
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
        )
    }

    #[test]
    fn test_restoration_point_verifies() {
        let s = make_snapshot();
        let rp = RestorationPoint::new(1, 1, 1, &s, [0xAB; 32], [0xBC; 32], None, None);
        assert!(rp.verify().is_ok());
    }
    #[test]
    fn test_resume_position_is_after_snapshot_end() {
        let s = make_snapshot();
        let rp = RestorationPoint::new(1, 1, 1, &s, [0xAB; 32], [0xBC; 32], None, None);
        assert_eq!(rp.resume_transcript_position, 100);
    }
    #[test]
    fn test_hash_deterministic() {
        let s = make_snapshot();
        let r1 = RestorationPoint::new(1, 1, 1, &s, [0xAB; 32], [0xBC; 32], None, None);
        let r2 = RestorationPoint::new(1, 1, 1, &s, [0xAB; 32], [0xBC; 32], None, None);
        assert_eq!(r1.point_hash, r2.point_hash);
    }
    #[test]
    fn test_preceding_entry_affects_hash() {
        let s = make_snapshot();
        let r1 = RestorationPoint::new(1, 1, 1, &s, [0xAB; 32], [0xBC; 32], None, None);
        let r2 = RestorationPoint::new(1, 1, 1, &s, [0xAB; 32], [0xBC; 32], Some([0xFF; 32]), None);
        assert_ne!(r1.point_hash, r2.point_hash);
    }
}
