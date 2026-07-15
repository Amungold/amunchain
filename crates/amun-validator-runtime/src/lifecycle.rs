use crate::state_machine::RuntimeStateMachine;
use amun_validator_api::error::{
    PlatformError, PlatformResult, StateMachineError, StateMachineErrorCode,
};
use amun_validator_api::types::state::RuntimeState;
use std::sync::Arc;

pub struct RuntimeLifecycle {
    state_machine: Arc<RuntimeStateMachine>,
}

impl RuntimeLifecycle {
    pub fn new(state_machine: Arc<RuntimeStateMachine>) -> Self {
        RuntimeLifecycle { state_machine }
    }
    pub fn current(&self) -> RuntimeState {
        self.state_machine.current()
    }
    pub fn next_states(&self) -> Vec<RuntimeState> {
        self.state_machine.allowed_next_states()
    }
    pub fn is_terminal(&self) -> bool {
        matches!(self.current(), RuntimeState::Retired)
    }
    pub fn is_active(&self) -> bool {
        matches!(
            self.current(),
            RuntimeState::Voting | RuntimeState::Candidate
        )
    }
    pub fn is_transitioning(&self) -> bool {
        matches!(
            self.current(),
            RuntimeState::Provisioning
                | RuntimeState::Bootstrapping
                | RuntimeState::Discovering
                | RuntimeState::Syncing
                | RuntimeState::Verifying
                | RuntimeState::Recovering
                | RuntimeState::Upgrading
                | RuntimeState::Draining
        )
    }

    pub(crate) fn startup(&self) -> PlatformResult<()> {
        if self.is_terminal() {
            return Err(PlatformError::StateMachine(StateMachineError::new(
                StateMachineErrorCode::IllegalTransition,
                "Cannot start from Retired".into(),
            )));
        }
        self.state_machine.transition(RuntimeState::Provisioning)?;
        Ok(())
    }

    pub(crate) fn shutdown(&self) -> PlatformResult<()> {
        if self.is_terminal() {
            return Ok(());
        }
        let current = self.current();
        if current != RuntimeState::Draining && current != RuntimeState::Retired {
            self.state_machine.force_transition(RuntimeState::Draining);
        }
        if self.current() != RuntimeState::Retired {
            self.state_machine.force_transition(RuntimeState::Retired);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_validator_api::types::id::ValidatorId;
    fn node() -> ValidatorId {
        ValidatorId([1u8; 32])
    }
    #[test]
    fn test_startup() {
        let sm = Arc::new(RuntimeStateMachine::new(node()));
        let l = RuntimeLifecycle::new(sm);
        l.startup().unwrap();
        assert_eq!(l.current(), RuntimeState::Provisioning);
    }
    #[test]
    fn test_startup_retired_fails() {
        let sm = Arc::new(RuntimeStateMachine::new(node()));
        sm.force_transition(RuntimeState::Retired);
        let l = RuntimeLifecycle::new(sm);
        assert!(l.startup().is_err());
    }
    #[test]
    fn test_shutdown_anywhere() {
        let sm = Arc::new(RuntimeStateMachine::new(node()));
        sm.transition(RuntimeState::Provisioning).unwrap();
        let l = RuntimeLifecycle::new(sm);
        l.shutdown().unwrap();
        assert_eq!(l.current(), RuntimeState::Retired);
    }
}
