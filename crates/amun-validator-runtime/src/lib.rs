pub mod lifecycle;
pub mod observer;
pub mod state_machine;
pub mod supervisor;

pub use lifecycle::RuntimeLifecycle;
pub use observer::TransitionObserver;
pub use state_machine::{RuntimeStateMachine, TransitionResult};
pub use supervisor::RuntimeSupervisor;

use amun_validator_api::error::PlatformResult;
use amun_validator_api::types::id::ValidatorId;
use amun_validator_api::types::state::RuntimeState;
use std::sync::Arc;

pub struct ValidatorRuntime {
    state_machine: Arc<RuntimeStateMachine>,
    supervisor: Arc<RuntimeSupervisor>,
    lifecycle: RuntimeLifecycle,
}

impl ValidatorRuntime {
    pub fn new(node_id: ValidatorId) -> PlatformResult<Self> {
        let state_machine = Arc::new(RuntimeStateMachine::new(node_id));
        let supervisor = Arc::new(RuntimeSupervisor::new(state_machine.clone()));
        let lifecycle = RuntimeLifecycle::new(state_machine.clone());
        Ok(ValidatorRuntime {
            state_machine,
            supervisor,
            lifecycle,
        })
    }

    pub fn attach_observer<T: TransitionObserver + 'static>(&self, observer: T) {
        self.state_machine.attach_observer(Arc::new(observer));
    }

    pub fn start(&self) -> PlatformResult<()> {
        self.supervisor.start()
    }
    pub fn stop(&self) -> PlatformResult<()> {
        self.supervisor.stop()
    }
    pub fn state(&self) -> RuntimeState {
        self.state_machine.current()
    }
    pub fn is_healthy(&self) -> bool {
        self.supervisor.is_healthy()
    }
    pub fn lifecycle(&self) -> &RuntimeLifecycle {
        &self.lifecycle
    }
    pub fn transition_count(&self) -> u64 {
        self.state_machine.transition_count()
    }
    pub fn previous_state(&self) -> Option<RuntimeState> {
        self.state_machine.previous()
    }
}
