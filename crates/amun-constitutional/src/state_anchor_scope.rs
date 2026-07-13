//! StateAnchorScope — the admissibility surface of a state anchor.
//!
//! Defines the constitutional boundaries within which a state root
//! is attested. Every scope is independently verifiable from prior
//! constitutional artifacts.
//!
//! INVARIANT (Anchor Monotonicity):
//!   Within the same context, revisions, and boundary lineage,
//!   a broader anchor scope either:
//!     - Extends the narrower scope (superset span, same state root)
//!     - Supersedes it constitutionally (different root, explicit reason)
//!     - Diverges explicitly (different context/boundary/revision)
//!   There are NO silent conflicting state roots.

use crate::constitutional_hasher::ConstitutionalHasher;
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;

/// Relationship between two state anchor scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorScopeRelationship {
    /// Scopes are identical — same span, same root.
    Identical,
    /// This scope extends the other (broader span, same root).
    Extends,
    /// This scope covers a different span with a different root.
    StateTransition,
    /// Scopes diverge — different context, revision, or boundary.
    Divergent,
    /// Scopes overlap partially — WARNING for potential conflicts.
    Overlapping,
}

/// The constitutional scope of a state anchor.
///
/// All fields participate in the constitutional hash.
/// The state_root is the attested state at the end of this scope's transcript span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateAnchorScope {
    /// Start of the transcript span this anchor attests to.
    pub transcript_start: u64,
    /// End of the transcript span (inclusive).
    pub transcript_end: u64,
    /// The state root attested for this span.
    pub state_root: ConstitutionalHash,
    /// Context this anchor is scoped to.
    pub context_hash: ConstitutionalHash,
    /// Constitutional revision active for this scope.
    pub constitutional_revision: u32,
    /// Replay revision active for this scope.
    pub replay_revision: u32,
    /// Boundary that was active for this scope.
    pub boundary_hash: ConstitutionalHash,
}

impl StateAnchorScope {
    pub fn scope_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"STATE_ANCHOR_SCOPE")
            .update_u64(self.transcript_start)
            .update_u64(self.transcript_end)
            .update_bytes(&self.state_root)
            .update_bytes(&self.context_hash)
            .update_u32(self.constitutional_revision)
            .update_u32(self.replay_revision)
            .update_bytes(&self.boundary_hash);
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

    /// Determine the relationship between this anchor scope and another.
    pub fn relationship_to(&self, other: &StateAnchorScope) -> AnchorScopeRelationship {
        if self == other {
            return AnchorScopeRelationship::Identical;
        }

        // Different context, revision, or boundary → divergent
        if self.context_hash != other.context_hash
            || self.constitutional_revision != other.constitutional_revision
            || self.replay_revision != other.replay_revision
            || self.boundary_hash != other.boundary_hash
        {
            return AnchorScopeRelationship::Divergent;
        }

        // Same context/revision/boundary — check span relationship
        let self_contains_other = self.transcript_start <= other.transcript_start
            && self.transcript_end >= other.transcript_end;
        let other_contains_self = other.transcript_start <= self.transcript_start
            && other.transcript_end >= self.transcript_end;

        if self_contains_other && self.state_root == other.state_root {
            AnchorScopeRelationship::Extends
        } else if self_contains_other && self.state_root != other.state_root {
            AnchorScopeRelationship::StateTransition
        } else if other_contains_self && other.state_root == self.state_root {
            AnchorScopeRelationship::Extends
        } else if other_contains_self && other.state_root != self.state_root {
            AnchorScopeRelationship::StateTransition
        } else if self.transcript_end < other.transcript_start
            || other.transcript_end < self.transcript_start
        {
            // Non-overlapping — sequential spans
            AnchorScopeRelationship::Extends
        } else {
            // Overlapping but neither contains the other
            AnchorScopeRelationship::Overlapping
        }
    }

    /// Verify anchor monotonicity against a parent scope.
    /// StateTransition is accepted (constitutional evolution),
    /// but Divergent and Overlapping are rejected.
    pub fn verify_against_parent(
        &self,
        parent: &StateAnchorScope,
    ) -> Result<AnchorScopeRelationship, AnchorScopeRelationship> {
        let rel = self.relationship_to(parent);
        match rel {
            AnchorScopeRelationship::Identical
            | AnchorScopeRelationship::Extends
            | AnchorScopeRelationship::StateTransition => Ok(rel),
            AnchorScopeRelationship::Divergent | AnchorScopeRelationship::Overlapping => Err(rel),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(start: u64, end: u64, root: [u8; 32]) -> StateAnchorScope {
        StateAnchorScope {
            transcript_start: start,
            transcript_end: end,
            state_root: root,
            context_hash: [0xAB; 32],
            constitutional_revision: 1,
            replay_revision: 1,
            boundary_hash: [0xBC; 32],
        }
    }

    #[test]
    fn test_identical() {
        let s = ms(0, 99, [0x11; 32]);
        assert_eq!(s.relationship_to(&s), AnchorScopeRelationship::Identical);
    }
    #[test]
    fn test_extends_same_root() {
        let n = ms(0, 49, [0x11; 32]);
        let b = ms(0, 99, [0x11; 32]);
        assert_eq!(b.relationship_to(&n), AnchorScopeRelationship::Extends);
    }
    #[test]
    fn test_state_transition() {
        let n = ms(0, 49, [0x11; 32]);
        let b = ms(0, 99, [0x22; 32]);
        assert_eq!(
            b.relationship_to(&n),
            AnchorScopeRelationship::StateTransition
        );
    }
    #[test]
    fn test_divergent_different_context() {
        let s1 = ms(0, 99, [0x11; 32]);
        let mut s2 = ms(0, 99, [0x11; 32]);
        s2.context_hash = [0xCD; 32];
        assert_eq!(s1.relationship_to(&s2), AnchorScopeRelationship::Divergent);
    }
    #[test]
    fn test_overlapping() {
        let s1 = ms(0, 50, [0x11; 32]);
        let s2 = ms(25, 75, [0x22; 32]);
        assert_eq!(
            s1.relationship_to(&s2),
            AnchorScopeRelationship::Overlapping
        );
    }
    #[test]
    fn test_verify_against_parent_ok() {
        let p = ms(0, 99, [0x11; 32]);
        let c = ms(0, 49, [0x11; 32]);
        assert!(c.verify_against_parent(&p).is_ok());
    }
    #[test]
    fn test_verify_against_parent_state_transition_ok() {
        let p = ms(0, 99, [0x11; 32]);
        let c = ms(0, 49, [0x22; 32]);
        assert!(c.verify_against_parent(&p).is_ok());
    }
    #[test]
    fn test_verify_against_parent_divergent_rejected() {
        let p = ms(0, 99, [0x11; 32]);
        let mut c = ms(0, 49, [0x11; 32]);
        c.context_hash = [0xCD; 32];
        assert!(c.verify_against_parent(&p).is_err());
    }
    #[test]
    fn test_contains_position() {
        let s = ms(10, 20, [0; 32]);
        assert!(s.contains_position(10));
        assert!(!s.contains_position(9));
    }
    #[test]
    fn test_span_length() {
        assert_eq!(ms(0, 99, [0; 32]).span_length(), 100);
    }
}
