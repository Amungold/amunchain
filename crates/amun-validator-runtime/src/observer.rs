use crate::state_machine::TransitionResult;

pub trait TransitionObserver: Send + Sync {
    /// Called after the state has been committed within the critical section.
    /// The `event.from` reflects the old state, `event.to` reflects the new state.
    /// Calling `state_machine.current()` will return the NEW state at this point.
    fn on_transition_committed(&self, event: &TransitionResult);

    /// Called when a transition attempt is rejected by the state machine.
    fn on_transition_failed(
        &self,
        from: amun_validator_api::types::state::RuntimeState,
        to: amun_validator_api::types::state::RuntimeState,
        reason: &str,
    );
}
