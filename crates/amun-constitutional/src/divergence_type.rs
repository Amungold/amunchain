//! DivergenceType — constitutional classification of replay divergence.
//!
//! Not all divergences are equal. Some are constitutional (explicit forks,
//! governance transitions), others are violations (replay errors, boundary
//! breaches), and some are warnings (lineage ambiguity).

use crate::constitutional_hasher::ConstitutionalHasher;
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;

/// The constitutional classification of a replay divergence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivergenceType {
    /// A constitutionally sanctioned fork.
    /// Example: governance-approved protocol upgrade with explicit fork point.
    ConstitutionalFork = 0x01,

    /// A divergence caused by replay error — should be investigated.
    /// Example: nondeterministic execution, corrupted transcript.
    ReplayError = 0x02,

    /// A divergence caused by boundary violation.
    /// Example: execution exceeded admissibility limits.
    BoundaryViolation = 0x03,

    /// A divergence caused by constitutional revision change.
    /// Example: new constitutional rules produce different outcomes.
    RevisionTransition = 0x04,

    /// A lineage ambiguity — overlapping attestations with conflicting outcomes.
    /// Example: two certificates claim different state roots for same span.
    LineageAmbiguity = 0x05,

    /// An intentional constitutional supersession.
    /// Example: governance explicitly supersedes prior attestation.
    ConstitutionalSupersession = 0x06,
}

impl DivergenceType {
    pub fn type_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"DIVERGENCE_TYPE").update_u8(*self as u8);
        h.finalize()
    }

    /// Returns true if this divergence is constitutionally admissible.
    pub fn is_admissible(&self) -> bool {
        matches!(
            self,
            DivergenceType::ConstitutionalFork
                | DivergenceType::RevisionTransition
                | DivergenceType::ConstitutionalSupersession
        )
    }

    /// Returns true if this divergence indicates a replay error.
    pub fn is_error(&self) -> bool {
        matches!(
            self,
            DivergenceType::ReplayError | DivergenceType::BoundaryViolation
        )
    }

    /// Returns true if this divergence creates lineage ambiguity.
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, DivergenceType::LineageAmbiguity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_admissible_divergences() {
        assert!(DivergenceType::ConstitutionalFork.is_admissible());
        assert!(DivergenceType::RevisionTransition.is_admissible());
        assert!(DivergenceType::ConstitutionalSupersession.is_admissible());
    }
    #[test]
    fn test_error_divergences() {
        assert!(DivergenceType::ReplayError.is_error());
        assert!(DivergenceType::BoundaryViolation.is_error());
        assert!(!DivergenceType::ConstitutionalFork.is_error());
    }
    #[test]
    fn test_ambiguous() {
        assert!(DivergenceType::LineageAmbiguity.is_ambiguous());
        assert!(!DivergenceType::ReplayError.is_ambiguous());
    }
}
