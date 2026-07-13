use super::absolute_invariants::AbsoluteInvariant;

/// Meta-Amendment: laws governing how laws themselves may be changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaAmendmentLaw {
    pub law_id: u64,
    pub governs: MetaAmendmentScope,
    pub required_quorum: MetaQuorum,
    pub absolute_invariants_untouchable: Vec<AbsoluteInvariant>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaAmendmentScope {
    /// Can amend operational parameters (quorum thresholds, epoch durations)
    OperationalParameters,
    /// Can amend governance structure (validator sets, voting rules)
    GovernanceStructure,
    /// Can amend constitutional text (Constitution.md)
    ConstitutionalText,
    /// Can amend proof semantics (requires new protocol version)
    ProofSemantics,
    /// Can amend replay guarantees
    ReplayGuarantees,
    /// CANNOT amend - this scope is meta-constitutionally forbidden
    AbsolutelyForbidden,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaQuorum {
    /// Requires 95%+ of all validators
    NearAbsolute { threshold_percent: u8 },
    /// Requires 80%+ of all validators
    SuperMajority { threshold_percent: u8 },
    /// Requires constitutional court + supermajority
    CourtAndSuperMajority,
    /// Cannot be reached - absolutely forbidden
    Unreachable,
}

/// The Meta-Constitutional Amendment Engine.
pub struct MetaAmendmentEngine;

impl MetaAmendmentEngine {
    /// Determine if a proposed amendment is meta-constitutionally valid.
    pub fn validate_amendment_scope(
        scope: &MetaAmendmentScope,
        absolute_invariants: &[AbsoluteInvariant],
    ) -> Result<(), String> {
        match scope {
            MetaAmendmentScope::AbsolutelyForbidden => {
                Err("This scope is absolutely forbidden by meta-constitutional law".to_string())
            }
            MetaAmendmentScope::ProofSemantics | MetaAmendmentScope::ReplayGuarantees => {
                // Must verify that no absolute invariant is touched
                for inv in absolute_invariants {
                    match inv {
                        AbsoluteInvariant::ReplayDeterminismAbsolute => {
                            if matches!(scope, MetaAmendmentScope::ReplayGuarantees) {
                                return Err(
                                    "Cannot amend replay guarantees: ReplayDeterminismAbsolute"
                                        .to_string(),
                                );
                            }
                        }
                        AbsoluteInvariant::ProvableTransitionAbsolute => {
                            if matches!(scope, MetaAmendmentScope::ProofSemantics) {
                                return Err(
                                    "Cannot amend proof semantics: ProvableTransitionAbsolute"
                                        .to_string(),
                                );
                            }
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
