use super::legitimacy_guards::LegitimacyGuard;
use super::phase_transitions::PhaseTransition;
use super::self_modeling::SelfModel;

/// Meta-constitutional consistency verification.
/// Ensures the constitution does not evolve into a state
/// where it can no longer prove its own legitimacy.
#[derive(Debug, Clone)]
pub struct MetaConsistency {
    pub self_model: SelfModel,
    pub active_guards: Vec<LegitimacyGuard>,
    pub consistency_violations: Vec<ConsistencyViolation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsistencyViolation {
    /// The constitution has entered a state where it cannot
    /// prove its own legitimacy (Gödel-like incompleteness)
    LegitimacyUnprovable { at_state: [u8; 32] },
    /// A self-reference paradox has been detected
    SelfReferenceParadox { depth: u64 },
    /// A guard protecting absolute invariants has been exhausted
    GuardExhausted {
        invariant: super::legitimacy_guards::GuardedInvariant,
    },
    /// A catastrophic phase transition has occurred
    CatastrophicTransition { from: [u8; 32], to: [u8; 32] },
    /// The amendment process itself has been invalidated
    AmendmentProcessInvalidated,
}

impl MetaConsistency {
    pub fn new(constitution_hash: [u8; 32]) -> Self {
        Self {
            self_model: SelfModel::new(constitution_hash),
            active_guards: Vec::new(),
            consistency_violations: Vec::new(),
        }
    }

    /// Add a legitimacy guard to protect a constitutional invariant.
    pub fn add_guard(&mut self, guard: LegitimacyGuard) {
        self.active_guards.push(guard);
    }

    /// Verify meta-consistency after a constitutional evolution step.
    pub fn verify_consistency(
        &mut self,
        _new_state: [u8; 32],
        transition: Option<&PhaseTransition>,
    ) -> Result<(), Vec<ConsistencyViolation>> {
        let mut violations = Vec::new();

        // Check self-reference paradoxes
        if self.self_model.detect_paradox() {
            violations.push(ConsistencyViolation::SelfReferenceParadox {
                depth: self.self_model.self_reference_depth,
            });
        }

        // Check catastrophic transitions
        if let Some(t) = transition {
            if t.is_catastrophic() {
                violations.push(ConsistencyViolation::CatastrophicTransition {
                    from: t.before_state,
                    to: t.after_state,
                });
            }
        }

        // Check guard status
        for guard in &self.active_guards {
            if !guard.is_active {
                violations.push(ConsistencyViolation::GuardExhausted {
                    invariant: guard.protects.clone(),
                });
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            self.consistency_violations.extend(violations.clone());
            Err(violations)
        }
    }

    /// The ultimate meta-consistency check: can the constitution
    /// still prove its own legitimacy?
    pub fn can_prove_legitimacy(&self) -> bool {
        !self.self_model.paradox_detected
            && self.active_guards.iter().all(|g| g.is_active)
            && self.consistency_violations.is_empty()
    }
}
