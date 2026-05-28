use super::historical_invariants::HistoricalInvariantEngine;
use super::invariants::ConstitutionalInvariant;
use super::preconditions::Precondition;
use super::replay_log::ConstitutionalReplayDAG;
use super::states::{ConstitutionalState, StateTag};
use super::transitions::{Transition, TransitionAlgebra, TransitionType};

/// ConstitutionalPolicyEngine: determines WHAT is legal.
/// Separated from execution to prevent hidden sovereign authority.
pub struct ConstitutionalPolicyEngine;

impl ConstitutionalPolicyEngine {
    pub fn is_legal(from: StateTag, transition: TransitionType) -> bool {
        TransitionAlgebra::resolve(from, transition).is_some()
    }

    pub fn forbidden(from: StateTag) -> Vec<TransitionType> {
        TransitionAlgebra::forbidden_transitions(from)
    }
}

/// ConstitutionalExecutionEngine: determines HOW to execute.
pub struct ConstitutionalExecutionEngine {
    pub history: HistoricalInvariantEngine,
    pub replay_log: ConstitutionalReplayDAG,
}

impl Default for ConstitutionalExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstitutionalExecutionEngine {
    pub fn new() -> Self {
        Self {
            history: HistoricalInvariantEngine::new(),
            replay_log: ConstitutionalReplayDAG::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn execute(
        &mut self,
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
        constitutional_delta_hash: [u8; 32],
        causal_transition_hash: Option<[u8; 32]>,
    ) -> Result<(ConstitutionalState, Transition), Vec<String>> {
        if !ConstitutionalPolicyEngine::is_legal(from.state_tag, transition_type) {
            return Err(vec![format!(
                "Illegal transition: {:?} -> {:?}",
                from.state_tag, transition_type
            )]);
        }

        Precondition::verify_all(from, transition_type, preconditions)?;

        let to_tag = TransitionAlgebra::resolve(from.state_tag, transition_type).unwrap();
        let transition = Transition::new(
            transition_type,
            from.clone(),
            to_tag,
            epoch,
            generation,
            previous_transition_hash,
            checkpoint_root,
            lineage_head_hash,
            constitutional_delta_hash,
            causal_transition_hash,
        );

        if let Err(e) = transition.verify_monotonicity() {
            return Err(vec![e]);
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
            return Err(vec!["New state hash verification failed".to_string()]);
        }

        // Update historical engine
        self.history.new_epoch(epoch);
        match transition_type {
            TransitionType::Freeze | TransitionType::Unfreeze => {
                self.history.record_freeze_unfreeze()
            }
            TransitionType::ProposeAmendment => self.history.record_amendment(),
            _ => {}
        }

        // Append to replay log
        self.replay_log
            .append(&transition, vec![transition.transition_id]);

        Ok((new_state, transition))
    }
}

/// ConstitutionalProofEngine: determines WHY a transition is valid.
pub struct ConstitutionalProofEngine;

impl ConstitutionalProofEngine {
    pub fn prove_transition(
        transition: &Transition,
        invariants: &[ConstitutionalInvariant],
        old_state: &ConstitutionalState,
        new_state: &ConstitutionalState,
    ) -> Result<[u8; 32], Vec<String>> {
        let mut errors = Vec::new();
        for inv in invariants {
            match inv {
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
            Ok(transition.transition_id)
        } else {
            Err(errors)
        }
    }
}
