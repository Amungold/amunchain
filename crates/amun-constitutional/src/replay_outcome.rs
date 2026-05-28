//! ReplayOutcome — constitutional admissibility result.
//!
//! This is NOT a runtime status enum.
//! It records whether the execution was constitutionally admissible
//! given the context, boundary, evidence, and lineage at the time.

use crate::constitutional_hasher::ConstitutionalHasher;
use crate::kernel_types::ConstitutionalHash;

/// The constitutional outcome of a replay.
///
/// INVARIANT: This is determined entirely from prior constitutional artifacts.
/// No runtime state, no external inputs, no nondeterministic factors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayOutcome {
    /// Execution was constitutionally admissible within its boundary
    Admitted = 0x01,

    /// Execution diverged from the canonical transcript
    Divergent = 0x02,

    /// Execution violated its active boundary constraints
    BoundaryViolation = 0x03,

    /// Execution produced a constitutional failure
    ConstitutionalFailure = 0x04,
}

impl ReplayOutcome {
    /// Compute a deterministic hash of this outcome.
    /// Uses the discriminant only — no payload.
    pub fn outcome_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_REPLAY_OUTCOME);
        h.update_u8(*self as u8);
        h.finalize()
    }

    /// Returns true if this outcome represents a successful replay.
    pub fn is_admitted(&self) -> bool {
        matches!(self, ReplayOutcome::Admitted)
    }

    /// Returns true if this outcome represents any form of failure.
    pub fn is_failure(&self) -> bool {
        !self.is_admitted()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outcome_hash_deterministic() {
        assert_eq!(
            ReplayOutcome::Admitted.outcome_hash(),
            ReplayOutcome::Admitted.outcome_hash()
        );
    }

    #[test]
    fn test_different_outcomes_different_hashes() {
        assert_ne!(
            ReplayOutcome::Admitted.outcome_hash(),
            ReplayOutcome::Divergent.outcome_hash()
        );
    }

    #[test]
    fn test_is_admitted() {
        assert!(ReplayOutcome::Admitted.is_admitted());
        assert!(!ReplayOutcome::Divergent.is_admitted());
        assert!(!ReplayOutcome::BoundaryViolation.is_admitted());
        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
    }

    #[test]
    fn test_is_failure() {
        assert!(!ReplayOutcome::Admitted.is_failure());
        assert!(ReplayOutcome::Divergent.is_failure());
        assert!(ReplayOutcome::BoundaryViolation.is_failure());
        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
    }
}
