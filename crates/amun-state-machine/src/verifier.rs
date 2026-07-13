use super::invariants::ConstitutionalInvariant;
use super::preconditions::Precondition;
use super::states::{ConstitutionalState, StateTag};
use super::transitions::{Transition, TransitionAlgebra, TransitionType};

pub struct TransitionVerifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationResult {
    Accepted {
        new_state: ConstitutionalState,
        transition_id: [u8; 32],
    },
    Rejected {
        reasons: Vec<String>,
    },
}

impl TransitionVerifier {
    #[allow(clippy::too_many_arguments)]
    pub fn verify_transition(
        from: &ConstitutionalState,
        transition_type: TransitionType,
        preconditions: &[Precondition],
        epoch: u64,
        generation: u64,
        civilization_id: [u8; 32],
        constitution_hash: [u8; 32],
        previous_transition_hash: Option<[u8; 32]>,
        checkpoint_root: [u8; 32],
        lineage_head_hash: [u8; 32],
    ) -> VerificationResult {
        let to_tag = match TransitionAlgebra::resolve(from.state_tag, transition_type) {
            Some(tag) => tag,
            None => {
                let forbidden = TransitionAlgebra::forbidden_transitions(from.state_tag);
                return VerificationResult::Rejected {
                    reasons: vec![format!(
                        "Illegal transition: {:?} -> {:?}. Forbidden from {:?}: {:?}",
                        from.state_tag, transition_type, from.state_tag, forbidden
                    )],
                };
            }
        };

        if let Err(errors) = Precondition::verify_all(from, transition_type, preconditions) {
            return VerificationResult::Rejected { reasons: errors };
        }

        let transition = Transition::new(
            transition_type,
            from.clone(),
            to_tag,
            epoch,
            generation,
            previous_transition_hash,
            checkpoint_root,
            lineage_head_hash,
            [0u8; 32],
            None,
        );

        if let Err(e) = transition.verify_monotonicity() {
            return VerificationResult::Rejected { reasons: vec![e] };
        }

        let new_state = ConstitutionalState::new(
            to_tag,
            civilization_id,
            constitution_hash,
            epoch,
            generation,
            Some(from.state_tag),
            Some(transition.transition_id),
        );

        if !new_state.verify() {
            return VerificationResult::Rejected {
                reasons: vec!["New state hash verification failed".to_string()],
            };
        }

        VerificationResult::Accepted {
            new_state,
            transition_id: transition.transition_id,
        }
    }

    pub fn verify_invariants(
        old_state: &ConstitutionalState,
        new_state: &ConstitutionalState,
        invariants: &[ConstitutionalInvariant],
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        for invariant in invariants {
            match invariant {
                ConstitutionalInvariant::NoImpossibleState => {
                    if new_state.state_tag == StateTag::Extinct
                        && old_state.state_tag == StateTag::Genesis
                    {
                        errors.push("Cannot go from Genesis directly to Extinct".to_string());
                    }
                }
                ConstitutionalInvariant::TransitionHistoryAcyclic => {
                    if old_state.state_hash == new_state.state_hash {
                        errors.push("Self-loop detected".to_string());
                    }
                }
                ConstitutionalInvariant::LineageIntact
                    if new_state.civilization_id != old_state.civilization_id =>
                {
                    errors.push("Civilization ID changed".to_string());
                }
                _ => {}
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
