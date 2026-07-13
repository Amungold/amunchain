use crate::snapshot::{CanonicalSnapshot, SnapshotExecutionContext};
use amun_deterministic_allocator::DeterministicMap;
use amun_state_transition::StateMachine;

/// Import a canonical snapshot with context restoration.
pub fn import_snapshot(
    sm: &mut StateMachine,
    snapshot: &CanonicalSnapshot,
) -> Result<SnapshotExecutionContext, &'static str> {
    if !snapshot.verify() {
        return Err("snapshot hash verification failed");
    }

    let mut new_state = DeterministicMap::new();
    for (key, value) in &snapshot.entries {
        let _ = new_state.insert(*key, value.clone());
    }

    let recomputed_root = StateMachine::compute_root(
        &new_state,
        snapshot.context.execution_version,
        snapshot.context.epoch_seal_hash,
    );

    if recomputed_root != snapshot.state_root {
        return Err("snapshot root mismatch - possible semantic forgery");
    }

    sm.state = new_state;
    sm.current_root = snapshot.state_root;
    sm.execution_version = snapshot.context.execution_version;
    sm.epoch_seal_hash = snapshot.context.epoch_seal_hash;

    Ok(snapshot.context.clone())
}
