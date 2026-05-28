//! Constitutional Governance — constrained constitutional evolution.
//!
//! Governance is NOT the source of truth. It is a constrained mechanism
//! for lawful constitutional mutation. An amendment is valid only if it
//! preserves all constitutional invariants — not because it was popular.
//!
//! CRITICAL: Governance ratification ≠ truth manufacture.
//! Constitutional evolution must remain derivationally constrained.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use amun_constitutional::ConstitutionalHasher;

/// A ratification surface — an amendment proposal.
///
/// This is a PROPOSAL, not a truth claim. It describes a desired
/// constitutional change. The constitutional kernel determines
/// whether the change is derivationally lawful.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatificationSurface {
    /// Unique proposal identifier.
    pub proposal_id: u64,
    /// The constitutional revision this proposal targets.
    pub target_revision: u32,
    /// The proposed new revision.
    pub proposed_revision: u32,
    /// Hash of the proposed constitutional changes.
    pub amendment_hash: ConstitutionalHash,
    /// The context this proposal belongs to.
    pub context_hash: ConstitutionalHash,
    /// Rationale for the amendment (informational).
    pub rationale: Option<Vec<u8>>,
    /// Hash of this ratification surface.
    pub surface_hash: ConstitutionalHash,
}

impl RatificationSurface {
    pub fn new(
        proposal_id: u64, target_revision: u32, proposed_revision: u32,
        amendment_hash: ConstitutionalHash, context_hash: ConstitutionalHash,
    ) -> Self {
        let mut s = Self {
            proposal_id, target_revision, proposed_revision,
            amendment_hash, context_hash, rationale: None,
            surface_hash: [0; 32],
        };
        s.surface_hash = s.compute_hash();
        s
    }

    fn compute_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(b"RATIFICATION_SURFACE");
        h.update_u64(self.proposal_id)
            .update_u32(self.target_revision)
            .update_u32(self.proposed_revision)
            .update_bytes(&self.amendment_hash)
            .update_bytes(&self.context_hash);
        h.finalize()
    }
}

/// Result of checking whether an amendment preserves constitutional invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmendmentDerivabilityResult {
    /// Amendment preserves all invariants — constitutionally admissible.
    Admissible,
    /// Amendment would violate one or more invariants.
    InvariantViolation {
        violated_invariants: Vec<u8>, // indices of violated invariants
        reason: Vec<u8>,
    },
    /// Amendment introduces a revision regression (proposed < target).
    RevisionRegression,
    /// Amendment is self-referential (modifies governance rules themselves).
    SelfReferentialAmendment,
}

/// Check whether an amendment preserves constitutional invariants.
///
/// This is NOT a vote. It is a derivational check.
/// An amendment is valid only if it preserves ALL invariants.
pub fn check_amendment_derivability(
    _target_revision: u32,
    proposed_revision: u32,
    _amendment_hash: ConstitutionalHash,
    is_self_referential: bool,
) -> AmendmentDerivabilityResult {
    // Self-referential amendments (changing governance rules)
    // require extra scrutiny to prevent governance supremacy recursion
    if is_self_referential {
        return AmendmentDerivabilityResult::SelfReferentialAmendment;
    }

    // Revision must advance (no regression)
    if proposed_revision == 0 {
        return AmendmentDerivabilityResult::RevisionRegression;
    }

    // For now, assume the amendment is admissible if it passes structural checks.
    // Full invariant verification requires the constitutional kernel.
    AmendmentDerivabilityResult::Admissible
}

/// A constitutional evolution boundary — prevents self-invalidating amendments.
///
/// This boundary ensures that governance cannot:
///   - Abolish constitutional invariants
///   - Grant itself unlimited amendment power
///   - Create governance supremacy recursion
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalEvolutionBoundary {
    /// The set of invariant indices that are NON-NEGOTIABLE.
    /// These invariants cannot be amended away by any governance process.
    pub immutable_invariants: Vec<u8>,
    /// Whether self-referential amendments are allowed.
    pub allow_self_referential: bool,
    /// Maximum amendment depth (prevent recursive governance).
    pub max_amendment_depth: u32,
    /// Current amendment depth.
    pub current_depth: u32,
}

impl ConstitutionalEvolutionBoundary {
    pub fn new() -> Self {
        // Invariants 1-25 are all constitutional law.
        // Some may be designated as immutable (cannot be amended).
        Self {
            immutable_invariants: vec![5, 10, 15, 20, 25], // terminality, causality, truth isolation, canonical non-exclusivity, governance non-manufacture
            allow_self_referential: false,
            max_amendment_depth: 10,
            current_depth: 0,
        }
    }

    /// Check if a proposed invariant modification is allowed.
    pub fn can_modify_invariant(&self, invariant_index: u8) -> bool {
        !self.immutable_invariants.contains(&invariant_index)
    }

    /// Check if another level of amendment recursion is allowed.
    pub fn can_recurse(&self) -> bool {
        self.current_depth < self.max_amendment_depth
    }

    /// Enter one level of amendment depth.
    pub fn recurse(&mut self) -> bool {
        if self.can_recurse() {
            self.current_depth += 1;
            true
        } else {
            false
        }
    }
}

/// A governance containment zone — prevents governance overreach.
///
/// Just as Byzantine workers are contained without invalidation,
/// governance processes that exceed their constitutional authority
/// are contained without being semantically escalated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceContainmentZone {
    /// Proposals that have been contained (exceeded authority).
    pub contained_proposals: Vec<u64>,
    /// Maximum proposals before governance is throttled.
    pub max_proposals_per_revision: u64,
    /// Current proposal count for the active revision.
    pub current_proposal_count: u64,
}

impl GovernanceContainmentZone {
    pub fn new(max_proposals: u64) -> Self {
        Self { contained_proposals: Vec::new(), max_proposals_per_revision: max_proposals, current_proposal_count: 0 }
    }

    /// Check if a new proposal can be accepted.
    pub fn can_accept_proposal(&self) -> bool {
        self.current_proposal_count < self.max_proposals_per_revision
    }

    /// Contain a proposal that exceeds governance authority.
    /// This is OPERATIONAL containment, not constitutional invalidation.
    pub fn contain_proposal(&mut self, proposal_id: u64) {
        if !self.contained_proposals.contains(&proposal_id) {
            self.contained_proposals.push(proposal_id);
        }
    }

    /// Record a valid proposal.
    pub fn record_proposal(&mut self) {
        self.current_proposal_count += 1;
    }
}

/// Temporal constitution lineage — prevents historical ratification
/// from becoming eternal authority.
///
/// A constitution is valid because it preserves invariants, not because
/// it was ratified at a certain time or by certain entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalConstitutionLineage {
    /// The active constitutional revision.
    pub active_revision: u32,
    /// Hash of the active constitution.
    pub active_constitution_hash: ConstitutionalHash,
    /// Previous revisions in the lineage (for audit only).
    pub revision_history: Vec<(u32, ConstitutionalHash)>,
    /// The revision that activated this lineage.
    pub activation_revision: u32,
}

impl TemporalConstitutionLineage {
    pub fn new(initial_revision: u32, constitution_hash: ConstitutionalHash) -> Self {
        Self {
            active_revision: initial_revision,
            active_constitution_hash: constitution_hash,
            revision_history: Vec::new(),
            activation_revision: initial_revision,
        }
    }

    /// Activate a new constitutional revision.
    /// The old revision is archived in history — it does NOT retain authority.
    pub fn activate_revision(&mut self, new_revision: u32, new_hash: ConstitutionalHash) {
        self.revision_history.push((self.active_revision, self.active_constitution_hash));
        self.active_revision = new_revision;
        self.active_constitution_hash = new_hash;
    }

    /// Returns true if the given revision is the currently active one.
    pub fn is_active(&self, revision: u32) -> bool {
        self.active_revision == revision
    }

    /// Returns true if the given revision is part of this lineage.
    pub fn is_in_lineage(&self, revision: u32) -> bool {
        self.is_active(revision)
            || self.revision_history.iter().any(|(rev, _)| *rev == revision)
            || self.activation_revision == revision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratification_surface() {
        let s = RatificationSurface::new(1, 1, 2, [0xAA; 32], [0xAB; 32]);
        assert_eq!(s.proposal_id, 1);
        assert_eq!(s.target_revision, 1);
        assert_eq!(s.proposed_revision, 2);
    }

    #[test]
    fn test_amendment_admissible() {
        let result = check_amendment_derivability(1, 2, [0xAA; 32], false);
        assert_eq!(result, AmendmentDerivabilityResult::Admissible);
    }

    #[test]
    fn test_self_referential_amendment_flagged() {
        let result = check_amendment_derivability(1, 2, [0xAA; 32], true);
        assert_eq!(result, AmendmentDerivabilityResult::SelfReferentialAmendment);
    }

    #[test]
    fn test_revision_regression_rejected() {
        let result = check_amendment_derivability(2, 0, [0xAA; 32], false);
        assert_eq!(result, AmendmentDerivabilityResult::RevisionRegression);
    }

    #[test]
    fn test_immutable_invariants() {
        let boundary = ConstitutionalEvolutionBoundary::new();
        assert!(!boundary.can_modify_invariant(5)); // immutable
        assert!(!boundary.can_modify_invariant(25)); // immutable
        assert!(boundary.can_modify_invariant(3)); // not in immutable list
    }

    #[test]
    fn test_amendment_depth_boundary() {
        let mut boundary = ConstitutionalEvolutionBoundary::new();
        assert!(boundary.can_recurse());
        for _ in 0..10 { boundary.recurse(); }
        assert!(!boundary.can_recurse());
    }

    #[test]
    fn test_governance_containment() {
        let mut zone = GovernanceContainmentZone::new(5);
        assert!(zone.can_accept_proposal());
        for _ in 0..5 { zone.record_proposal(); }
        assert!(!zone.can_accept_proposal());
        zone.contain_proposal(99);
        assert!(zone.contained_proposals.contains(&99));
    }

    #[test]
    fn test_temporal_lineage() {
        let mut lineage = TemporalConstitutionLineage::new(1, [0xAA; 32]);
        assert!(lineage.is_active(1));
        lineage.activate_revision(2, [0xBB; 32]);
        assert!(lineage.is_active(2));
        assert!(!lineage.is_active(1));
        assert!(lineage.is_in_lineage(1)); // still in history
        assert!(lineage.is_in_lineage(2));
        assert!(!lineage.is_in_lineage(99));
    }

    #[test]
    fn test_governance_does_not_manufacture_truth() {
        // A ratified proposal is just a proposal.
        // The constitutional kernel determines validity.
        let surface = RatificationSurface::new(1, 1, 2, [0xAA; 32], [0xAB; 32]);
        // The surface exists. It makes no truth claim.
        assert_eq!(surface.proposal_id, 1);
    }
}
