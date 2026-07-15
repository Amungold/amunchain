use crate::lifecycle::RuntimeLifecycle;
use crate::state_machine::RuntimeStateMachine;
use amun_validator_api::error::{
    PlatformError, PlatformResult, StateMachineError, StateMachineErrorCode,
};
use amun_validator_api::types::state::RuntimeState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct RuntimeSupervisor {
    lifecycle: RuntimeLifecycle,
    running: AtomicBool,
    healthy: AtomicBool,
}

impl RuntimeSupervisor {
    pub fn new(state_machine: Arc<RuntimeStateMachine>) -> Self {
        RuntimeSupervisor {
            lifecycle: RuntimeLifecycle::new(state_machine),
            running: AtomicBool::new(false),
            healthy: AtomicBool::new(false),
        }
    }

    pub fn start(&self) -> PlatformResult<()> {
        if self.running.load(Ordering::SeqCst) {
            return Err(PlatformError::StateMachine(StateMachineError::new(
                StateMachineErrorCode::IllegalTransition,
                "Already started".into(),
            )));
        }
        self.running.store(true, Ordering::SeqCst);
        self.lifecycle.startup()?;
        self.healthy.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn stop(&self) -> PlatformResult<()> {
        if !self.running.load(Ordering::SeqCst) {
            return Ok(());
        }
        self.healthy.store(false, Ordering::SeqCst);
        self.lifecycle.shutdown()?;
        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn state(&self) -> RuntimeState {
        self.lifecycle.current()
    }
    pub fn is_healthy(&self) -> bool {
        self.healthy.load(Ordering::SeqCst) && self.running.load(Ordering::SeqCst)
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
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
    fn test_start() {
        let s = RuntimeSupervisor::new(Arc::new(RuntimeStateMachine::new(node())));
        s.start().unwrap();
        assert_eq!(s.state(), RuntimeState::Provisioning);
        assert!(s.is_healthy());
    }
    #[test]
    fn test_stop() {
        let s = RuntimeSupervisor::new(Arc::new(RuntimeStateMachine::new(node())));
        s.start().unwrap();
        s.stop().unwrap();
        assert_eq!(s.state(), RuntimeState::Retired);
    }
    #[test]
    fn test_double_start() {
        let s = RuntimeSupervisor::new(Arc::new(RuntimeStateMachine::new(node())));
        s.start().unwrap();
        assert!(s.start().is_err());
    }
    #[test]
    fn test_double_stop() {
        let s = RuntimeSupervisor::new(Arc::new(RuntimeStateMachine::new(node())));
        s.start().unwrap();
        s.stop().unwrap();
        assert!(s.stop().is_ok());
    }
}
