//! SnapshotScope — the restoration admissibility surface of a snapshot.
//!
//! Defines the constitutional boundaries within which a snapshot
//! can be used for state restoration. Every scope is independently
//! verifiable from prior constitutional artifacts.
//!
//! Snapshot ≠ Backup: A snapshot is a constitutionally restorable
//! replay surface, not a serialized storage image.
//!
//! INVARIANT (Snapshot Monotonicity):
//!   Within the same context, revisions, and boundary lineage,
//!   a broader snapshot scope either:
//!     - Extends the narrower scope (superset span, same anchor)
//!     - Supersedes it constitutionally (different anchor, explicit reason)
//!     - Diverges explicitly (different context/boundary/revision)
//!   There are NO silent conflicting restoration surfaces.

use crate::constitutional_hasher::ConstitutionalHasher;
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;

/// Relationship between two snapshot scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotScopeRelationship {
    /// Identical scopes — same span, same anchor, same outcome.
    Identical,
    /// This scope extends the other (broader span, same anchor).
    Extends,
    /// This scope supersedes the other (different restoration outcome).
    Supersedes,
    /// Scopes diverge — different context, revision, or boundary.
    Divergent,
    /// Scopes overlap partially — WARNING for potential conflicts.
    Overlapping,
}

/// The outcome of a snapshot restoration attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestorationOutcome {
    /// Snapshot is admissible for restoration.
    Admissible = 0x01,
    /// Snapshot is from a divergent replay lineage.
    LineageDivergent = 0x02,
    /// Snapshot scope violates boundary constraints.
    BoundaryViolation = 0x03,
    /// Snapshot is superseded by a newer constitutional revision.
    ConstitutionallySuperseded = 0x04,
}

impl RestorationOutcome {
    pub fn is_admissible(&self) -> bool {
        matches!(self, RestorationOutcome::Admissible)
    }
    pub fn outcome_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"SNAPSHOT_OUTCOME").update_u8(*self as u8);
        h.finalize()
    }
}

/// The constitutional scope of a snapshot.
///
/// All fields participate in the constitutional hash.
/// The scope defines the restoration surface — what replay window
/// produced this snapshot and under what constitutional conditions
/// it can be used for restoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotScope {
    /// Start of the transcript span covered by this snapshot.
    pub transcript_start: u64,
    /// End of the transcript span (inclusive).
    pub transcript_end: u64,
    /// The state anchor this snapshot restores to.
    pub anchor_hash: ConstitutionalHash,
    /// Context this snapshot belongs to.
    pub context_hash: ConstitutionalHash,
    /// Constitutional revision active for this scope.
    pub constitutional_revision: u32,
    /// Replay revision active for this scope.
    pub replay_revision: u32,
    /// Boundary that was active for this scope.
    pub boundary_hash: ConstitutionalHash,
    /// Restoration admissibility outcome.
    pub restoration_outcome: RestorationOutcome,
}

impl SnapshotScope {
    pub fn scope_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"SNAPSHOT_SCOPE")
            .update_u64(self.transcript_start)
            .update_u64(self.transcript_end)
            .update_bytes(&self.anchor_hash)
            .update_bytes(&self.context_hash)
            .update_u32(self.constitutional_revision)
            .update_u32(self.replay_revision)
            .update_bytes(&self.boundary_hash)
            .update_u8(self.restoration_outcome as u8);
        h.finalize()
    }

    pub fn span_length(&self) -> u64 {
        self.transcript_end
            .saturating_sub(self.transcript_start)
            .saturating_add(1)
    }

    pub fn contains_position(&self, position: u64) -> bool {
        position >= self.transcript_start && position <= self.transcript_end
    }

    pub fn is_restorable(&self) -> bool {
        self.restoration_outcome.is_admissible()
    }

    /// Determine the relationship between this snapshot scope and another.
    pub fn relationship_to(&self, other: &SnapshotScope) -> SnapshotScopeRelationship {
        if self == other {
            return SnapshotScopeRelationship::Identical;
        }

        if self.context_hash != other.context_hash
            || self.constitutional_revision != other.constitutional_revision
            || self.replay_revision != other.replay_revision
            || self.boundary_hash != other.boundary_hash
        {
            return SnapshotScopeRelationship::Divergent;
        }

        let self_contains_other = self.transcript_start <= other.transcript_start
            && self.transcript_end >= other.transcript_end;
        let other_contains_self = other.transcript_start <= self.transcript_start
            && other.transcript_end >= self.transcript_end;

        if self_contains_other && self.anchor_hash == other.anchor_hash {
            SnapshotScopeRelationship::Extends
        } else if self_contains_other && self.anchor_hash != other.anchor_hash {
            SnapshotScopeRelationship::Supersedes
        } else if other_contains_self && other.anchor_hash == self.anchor_hash {
            SnapshotScopeRelationship::Extends
        } else if other_contains_self && other.anchor_hash != self.anchor_hash {
            SnapshotScopeRelationship::Supersedes
        } else if self.transcript_end < other.transcript_start
            || other.transcript_end < self.transcript_start
        {
            SnapshotScopeRelationship::Extends
        } else {
            SnapshotScopeRelationship::Overlapping
        }
    }

    /// Verify snapshot monotonicity against a parent scope.
    pub fn verify_against_parent(
        &self,
        parent: &SnapshotScope,
    ) -> Result<SnapshotScopeRelationship, SnapshotScopeRelationship> {
        let rel = self.relationship_to(parent);
        match rel {
            SnapshotScopeRelationship::Identical
            | SnapshotScopeRelationship::Extends
            | SnapshotScopeRelationship::Supersedes => Ok(rel),
            SnapshotScopeRelationship::Divergent | SnapshotScopeRelationship::Overlapping => {
                Err(rel)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(start: u64, end: u64, anchor: [u8; 32], outcome: RestorationOutcome) -> SnapshotScope {
        SnapshotScope {
            transcript_start: start,
            transcript_end: end,
            anchor_hash: anchor,
            context_hash: [0xAB; 32],
            constitutional_revision: 1,
            replay_revision: 1,
            boundary_hash: [0xBC; 32],
            restoration_outcome: outcome,
        }
    }

    #[test]
    fn test_identical() {
        let s = ms(0, 99, [0x11; 32], RestorationOutcome::Admissible);
        assert_eq!(s.relationship_to(&s), SnapshotScopeRelationship::Identical);
    }
    #[test]
    fn test_extends_same_anchor() {
        let n = ms(0, 49, [0x11; 32], RestorationOutcome::Admissible);
        let b = ms(0, 99, [0x11; 32], RestorationOutcome::Admissible);
        assert_eq!(b.relationship_to(&n), SnapshotScopeRelationship::Extends);
    }
    #[test]
    fn test_supersedes_different_anchor() {
        let n = ms(0, 49, [0x11; 32], RestorationOutcome::Admissible);
        let b = ms(0, 99, [0x22; 32], RestorationOutcome::Admissible);
        assert_eq!(b.relationship_to(&n), SnapshotScopeRelationship::Supersedes);
    }
    #[test]
    fn test_divergent_different_context() {
        let s1 = ms(0, 99, [0x11; 32], RestorationOutcome::Admissible);
        let mut s2 = ms(0, 99, [0x11; 32], RestorationOutcome::Admissible);
        s2.context_hash = [0xCD; 32];
        assert_eq!(
            s1.relationship_to(&s2),
            SnapshotScopeRelationship::Divergent
        );
    }
    #[test]
    fn test_verify_against_parent_ok() {
        let p = ms(0, 99, [0x11; 32], RestorationOutcome::Admissible);
        let c = ms(0, 49, [0x11; 32], RestorationOutcome::Admissible);
        assert!(c.verify_against_parent(&p).is_ok());
    }
    #[test]
    fn test_verify_against_parent_divergent_rejected() {
        let p = ms(0, 99, [0x11; 32], RestorationOutcome::Admissible);
        let mut c = ms(0, 49, [0x11; 32], RestorationOutcome::Admissible);
        c.context_hash = [0xCD; 32];
        assert!(c.verify_against_parent(&p).is_err());
    }
    #[test]
    fn test_is_restorable() {
        assert!(ms(0, 99, [0; 32], RestorationOutcome::Admissible).is_restorable());
        assert!(!ms(0, 99, [0; 32], RestorationOutcome::LineageDivergent).is_restorable());
    }
}
