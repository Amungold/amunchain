use amun_orchestrator_core::state::{DeploymentState, OrchestratorState, RuntimeState};

#[test]
fn test_valid_runtime_transitions() {
    let state = OrchestratorState::new();
    assert!(state.transition_runtime(RuntimeState::Booting).is_ok());
}

#[test]
fn test_invalid_runtime_transition_rejected() {
    let state = OrchestratorState::new();
    assert!(state.transition_runtime(RuntimeState::Running).is_err());
}

#[test]
fn test_force_stop_always_allowed() {
    let state = OrchestratorState::new();
    let booting = state.transition_runtime(RuntimeState::Booting).unwrap();
    let s = OrchestratorState {
        runtime: booting,
        deployment: DeploymentState::Idle,
    };
    let running = s.transition_runtime(RuntimeState::Running).unwrap();
    let s2 = OrchestratorState {
        runtime: running,
        deployment: DeploymentState::Idle,
    };
    assert!(s2.transition_runtime(RuntimeState::Stopped).is_ok());
}

#[test]
fn test_valid_deployment_transitions() {
    let state = OrchestratorState::new();
    assert!(state
        .transition_deployment(DeploymentState::Building)
        .is_ok());
}

#[test]
fn test_reset_to_idle_always_allowed() {
    let state = OrchestratorState::new();
    let building = state
        .transition_deployment(DeploymentState::Building)
        .unwrap();
    let s = OrchestratorState {
        runtime: RuntimeState::Stopped,
        deployment: building,
    };
    assert!(s.transition_deployment(DeploymentState::Idle).is_ok());
}
