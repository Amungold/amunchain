use super::states::{ConstitutionalState, StateTag};
use super::transitions::{TransitionAlgebra, TransitionType};

/// A precondition that must be satisfied before a transition can occur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Precondition {
    /// The current state must allow this transition type.
    LegalTransition { from: StateTag, to: TransitionType },
    /// Required quorum must be met for governance actions.
    QuorumMet { required: u64, actual: u64 },
    /// Evolution proof must be valid for amendments.
    EvolutionProofValid { proof_hash: [u8; 32] },
    /// Freeze boundary check must pass for any state change.
    FreezeBoundariesIntact,
    /// Replay continuity must be preserved.
    ReplayContinuityPreserved,
    /// Constitutional identity must remain unchanged unless amendment.
    IdentityPreserved,
}

impl Precondition {
    /// Verify that the transition is legal from the current state.
    pub fn verify_legal(
        from: &ConstitutionalState,
        transition: TransitionType,
    ) -> Result<(), String> {
        let legal = TransitionAlgebra::legal_transitions(from.state_tag);
        if !legal.contains(&transition) {
            return Err(format!(
                "Illegal transition: {:?} -> {:?}. Legal: {:?}",
                from.state_tag, transition, legal
            ));
        }
        Ok(())
    }

    /// Verify all preconditions for a transition.
    pub fn verify_all(
        from: &ConstitutionalState,
        transition: TransitionType,
        preconditions: &[Precondition],
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        for pc in preconditions {
            if let Err(e) = match pc {
                Precondition::LegalTransition { .. } => Self::verify_legal(from, transition),
                _ => Ok(()),
            } {
                errors.push(e);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
