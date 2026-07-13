//! CausalityType — semantic classification of constitutional dependency.
//!
//! NOT all relationships are causal. Some are merely ancestral (historical
//! ordering), others are informational (audit trail), and only some are
//! constitutional (validity depends on the dependency).
//!
//! DISTINCTION:
//!   - Ancestry: "A came before B"
//!   - Lineage: "B belongs to the same chain as A"
//!   - Causality: "B is constitutionally valid BECAUSE of A"
//!   - Dependency: "B cannot be valid WITHOUT A"

use crate::constitutional_hasher::ConstitutionalHasher;
use crate::hash_domains;
use crate::kernel_types::ConstitutionalHash;

/// The constitutional nature of a dependency between artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CausalityType {
    /// B's constitutional validity REQUIRES A.
    /// B is invalid without A. This is the strongest form of dependency.
    ConstitutionalDependency = 0x01,

    /// B's admissibility is CAUSED by A's admissibility.
    /// Example: receipt is admissible because commitment is admissible.
    AdmissibilityCause = 0x02,

    /// B inherits boundary constraints from A.
    /// Example: restored execution inherits the snapshot's boundary.
    BoundaryInheritance = 0x03,

    /// B's state is derived from A's state via replay.
    /// Example: post-transition state is causally derived from pre-transition state.
    StateDerivation = 0x04,

    /// B is caused by a constitutional revision that A was subject to.
    /// Example: post-revision artifacts depend on the revision transition.
    RevisionCause = 0x05,

    /// A certifies/admits B into the constitutional lineage.
    /// Example: certificate admits a state anchor.
    CertificationCause = 0x06,

    /// B supersedes A constitutionally.
    /// Example: new certificate supersedes old certificate.
    SupersessionCause = 0x07,

    /// A is merely an ancestor of B — historical ordering only.
    /// This is NOT a constitutional dependency. B is valid without A.
    AncestralOnly = 0x08,

    /// A provides informational context for B — audit trail only.
    /// This is NOT a constitutional dependency. B is valid without A.
    InformationalOnly = 0x09,
}

impl CausalityType {
    pub fn type_hash(&self) -> ConstitutionalHash {
        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
        h.update_bytes(b"CAUSALITY_TYPE").update_u8(*self as u8);
        h.finalize()
    }

    /// Returns true if this is a constitutional dependency (B requires A for validity).
    pub fn is_constitutional_dependency(&self) -> bool {
        matches!(
            self,
            CausalityType::ConstitutionalDependency
                | CausalityType::AdmissibilityCause
                | CausalityType::BoundaryInheritance
                | CausalityType::StateDerivation
                | CausalityType::RevisionCause
                | CausalityType::CertificationCause
                | CausalityType::SupersessionCause
        )
    }

    /// Returns true if this is non-causal (ancestral or informational only).
    pub fn is_non_causal(&self) -> bool {
        matches!(
            self,
            CausalityType::AncestralOnly | CausalityType::InformationalOnly
        )
    }

    /// Returns true if B would be constitutionally INVALID without A.
    pub fn is_hard_dependency(&self) -> bool {
        matches!(self, CausalityType::ConstitutionalDependency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_constitutional_dependencies() {
        assert!(CausalityType::ConstitutionalDependency.is_constitutional_dependency());
        assert!(CausalityType::AdmissibilityCause.is_constitutional_dependency());
        assert!(CausalityType::BoundaryInheritance.is_constitutional_dependency());
        assert!(CausalityType::StateDerivation.is_constitutional_dependency());
        assert!(CausalityType::RevisionCause.is_constitutional_dependency());
        assert!(CausalityType::CertificationCause.is_constitutional_dependency());
        assert!(CausalityType::SupersessionCause.is_constitutional_dependency());
    }
    #[test]
    fn test_non_causal() {
        assert!(CausalityType::AncestralOnly.is_non_causal());
        assert!(CausalityType::InformationalOnly.is_non_causal());
        assert!(!CausalityType::ConstitutionalDependency.is_non_causal());
    }
    #[test]
    fn test_hard_dependency() {
        assert!(CausalityType::ConstitutionalDependency.is_hard_dependency());
        assert!(!CausalityType::AdmissibilityCause.is_hard_dependency());
        assert!(!CausalityType::AncestralOnly.is_hard_dependency());
    }
}
