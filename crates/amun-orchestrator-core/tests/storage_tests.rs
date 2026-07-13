use amun_orchestrator_core::state::{OrchestratorState, RuntimeState};
use amun_orchestrator_core::StateStore;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_save_and_load_state() {
    let dir = TempDir::new().unwrap();
    let store = StateStore::new(&PathBuf::from(dir.path()));
    let mut s = OrchestratorState::new();
    s.runtime = RuntimeState::Booting;
    store.save_state(&s).unwrap();
    assert_eq!(store.load_state().unwrap().runtime, RuntimeState::Booting);
}

#[test]
fn test_load_returns_default_when_no_file() {
    let dir = TempDir::new().unwrap();
    let store = StateStore::new(&PathBuf::from(dir.path()));
    assert_eq!(store.load_state().unwrap().runtime, RuntimeState::Stopped);
}
