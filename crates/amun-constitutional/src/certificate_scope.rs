//! CertificateScope — the admissibility envelope of a replay certificate.
//!
//! A certificate scope defines the exact constitutional boundaries
//! that the certificate attests to. Every scope is independently
//! verifiable from prior constitutional artifacts.
//!
//! INVARIANT (Scope Monotonicity):
//!   A broader certificate cannot contradict a narrower certificate
//!   within the same scope lineage. It can only:
//!     - Extend the scope
//!     - Supersede it constitutionally (with explicit revision)
//!     - Diverge explicitly (with documented reason)
//!   This prevents: replay ambiguity, overlapping admissibility conflicts,
//!   and partial lineage contradictions.

use crate::constitutional_hasher::ConstitutionalHasher;
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;
use crate::replay_outcome::ReplayOutcome;

/// The relationship between two certificate scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeRelationship {
    /// Scopes are identical in all dimensions.
    Identical,

    /// This scope extends the other (broader transcript span, same context).
    Extends,

    /// This scope supersedes the other (different revision, explicit upgrade).
    Supersedes,

    /// The scopes diverge — they cover different contexts or boundaries.
    Divergent,

    /// The scopes overlap partially — this is a WARNING condition.
    /// Overlapping but non-identical scopes may indicate conflicting attestations.
    Overlapping,
}

/// The constitutional scope of a replay certificate.
///
/// All fields are INCLUDED in the constitutional hash.
/// This is the admissibility envelope — it defines what the certificate attests to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateScope {
    /// Start of the transcript span covered by this certificate.
    pub transcript_start: u64,

    /// End of the transcript span (inclusive).
    pub transcript_end: u64,

    /// Context this certificate is scoped to.
    pub context_hash: ConstitutionalHash,

    /// Constitutional revision this certificate was issued under.
    pub constitutional_revision: u32,

    /// Replay revision this certificate was issued under.
    pub replay_revision: u32,

    /// Boundary that was active for this certificate's scope.
    pub boundary_hash: ConstitutionalHash,

    /// The admissibility outcome for this scope.
    pub outcome: ReplayOutcome,
}

impl CertificateScope {
    /// Compute the constitutional hash of this scope.
    pub fn scope_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"SCOPE")
            .update_u64(self.transcript_start)
            .update_u64(self.transcript_end)
            .update_bytes(&self.context_hash)
            .update_u32(self.constitutional_revision)
            .update_u32(self.replay_revision)
            .update_bytes(&self.boundary_hash)
            .update_u8(self.outcome as u8);
        h.finalize()
    }

    /// Returns the transcript span length.
    pub fn span_length(&self) -> u64 {
        self.transcript_end
            .saturating_sub(self.transcript_start)
            .saturating_add(1)
    }

    /// Returns true if the given transcript position is within this scope.
    pub fn contains_position(&self, position: u64) -> bool {
        position >= self.transcript_start && position <= self.transcript_end
    }

    /// Determine the relationship between this scope and another.
    ///
    /// INVARIANT (Scope Monotonicity):
    ///   If two scopes share the same context, revision, and boundary,
    ///   the broader scope (wider transcript span) must either:
    ///     - EXTEND the narrower scope (superset span, same outcome)
    ///     - SUPERSEDE it (different outcome with explicit reason)
    ///   It must NOT silently overlap with conflicting outcomes.
    pub fn relationship_to(&self, other: &CertificateScope) -> ScopeRelationship {
        // Identical scopes
        if self == other {
            return ScopeRelationship::Identical;
        }

        // Different contexts or revisions → divergent
        if self.context_hash != other.context_hash
            || self.constitutional_revision != other.constitutional_revision
            || self.replay_revision != other.replay_revision
        {
            return ScopeRelationship::Divergent;
        }

        // Different boundaries → divergent
        if self.boundary_hash != other.boundary_hash {
            return ScopeRelationship::Divergent;
        }

        // Same context/revision/boundary — check span relationship
        let self_contains_other = self.transcript_start <= other.transcript_start
            && self.transcript_end >= other.transcript_end;
        let other_contains_self = other.transcript_start <= self.transcript_start
            && other.transcript_end >= self.transcript_end;

        if self_contains_other && self.outcome == other.outcome {
            // Self extends other
            ScopeRelationship::Extends
        } else if self_contains_other && self.outcome != other.outcome {
            // Self supersedes other (broader scope, different outcome)
            ScopeRelationship::Supersedes
        } else if other_contains_self && other.outcome == self.outcome {
            // Other extends self — from our perspective, we are extended
            ScopeRelationship::Extends
        } else if other_contains_self && other.outcome != self.outcome {
            // Other supersedes self — from our perspective, we are superseded
            ScopeRelationship::Supersedes
        } else if self.transcript_end < other.transcript_start
            || other.transcript_end < self.transcript_start
        {
            // Non-overlapping — different transcript segments, acceptable
            ScopeRelationship::Extends
        } else {
            // Overlapping but neither contains the other with same outcome
            // This is a WARNING — potential conflicting attestations
            ScopeRelationship::Overlapping
        }
    }

    /// Verify that this scope is consistent with a parent scope.
    /// Returns Ok if the relationship is valid (Identical, Extends, Supersedes).
    /// Returns Err if the relationship is Divergent or Overlapping.
    pub fn verify_against_parent(
        &self,
        parent: &CertificateScope,
    ) -> Result<ScopeRelationship, ScopeRelationship> {
        let rel = self.relationship_to(parent);
        match rel {
            ScopeRelationship::Identical
            | ScopeRelationship::Extends
            | ScopeRelationship::Supersedes => Ok(rel),
            ScopeRelationship::Divergent | ScopeRelationship::Overlapping => Err(rel),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_scope(start: u64, end: u64, outcome: ReplayOutcome) -> CertificateScope {
        CertificateScope {
            transcript_start: start,
            transcript_end: end,
            context_hash: [0xAB; 32],
            constitutional_revision: 1,
            replay_revision: 1,
            boundary_hash: [0xBC; 32],
            outcome,
        }
    }

    #[test]
    fn test_identical_scopes() {
        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
        let s2 = make_scope(0, 99, ReplayOutcome::Admitted);
        assert_eq!(s1.relationship_to(&s2), ScopeRelationship::Identical);
        assert_eq!(s1.scope_hash(), s2.scope_hash());
    }

    #[test]
    fn test_extends() {
        let narrow = make_scope(0, 49, ReplayOutcome::Admitted);
        let broad = make_scope(0, 99, ReplayOutcome::Admitted);
        assert_eq!(broad.relationship_to(&narrow), ScopeRelationship::Extends);
    }

    #[test]
    fn test_supersedes() {
        let narrow = make_scope(0, 49, ReplayOutcome::Admitted);
        let broad = make_scope(0, 99, ReplayOutcome::Divergent);
        assert_eq!(
            broad.relationship_to(&narrow),
            ScopeRelationship::Supersedes
        );
    }

    #[test]
    fn test_divergent_different_context() {
        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
        let mut s2 = make_scope(0, 99, ReplayOutcome::Admitted);
        s2.context_hash = [0xCD; 32];
        assert_eq!(s1.relationship_to(&s2), ScopeRelationship::Divergent);
    }

    #[test]
    fn test_overlapping_conflicting() {
        // Two scopes that overlap but don't fully contain each other
        let s1 = make_scope(0, 50, ReplayOutcome::Admitted);
        let s2 = make_scope(25, 75, ReplayOutcome::Divergent);
        assert_eq!(s1.relationship_to(&s2), ScopeRelationship::Overlapping);
    }

    #[test]
    fn test_contains_position() {
        let scope = make_scope(10, 20, ReplayOutcome::Admitted);
        assert!(scope.contains_position(10));
        assert!(scope.contains_position(15));
        assert!(scope.contains_position(20));
        assert!(!scope.contains_position(9));
        assert!(!scope.contains_position(21));
    }

    #[test]
    fn test_span_length() {
        assert_eq!(
            make_scope(0, 99, ReplayOutcome::Admitted).span_length(),
            100
        );
        assert_eq!(
            make_scope(10, 20, ReplayOutcome::Admitted).span_length(),
            11
        );
    }

    #[test]
    fn test_verify_against_parent_ok() {
        let parent = make_scope(0, 99, ReplayOutcome::Admitted);
        let child = make_scope(0, 49, ReplayOutcome::Admitted);
        assert!(child.verify_against_parent(&parent).is_ok());
    }

    #[test]
    fn test_verify_against_parent_divergent() {
        let parent = make_scope(0, 99, ReplayOutcome::Admitted);
        let mut child = make_scope(0, 49, ReplayOutcome::Admitted);
        child.context_hash = [0xCD; 32];
        assert!(child.verify_against_parent(&parent).is_err());
    }

    #[test]
    fn test_scope_hash_deterministic() {
        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
        let s2 = make_scope(0, 99, ReplayOutcome::Admitted);
        assert_eq!(s1.scope_hash(), s2.scope_hash());
    }

    #[test]
    fn test_different_span_different_hash() {
        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
        let s2 = make_scope(0, 49, ReplayOutcome::Admitted);
        assert_ne!(s1.scope_hash(), s2.scope_hash());
    }
}
