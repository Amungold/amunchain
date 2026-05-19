use crate::snapshot::{CanonicalSnapshot, SnapshotExecutionContext, SnapshotError};
use amun_state_transition::StateMachine;
use amun_chain_position::ChainPosition;

pub fn export_snapshot(
    sm: &StateMachine,
    position: ChainPosition,
    current_epoch: u64,
    genesis_root: [u8; 32],
    epoch_seal_hash: Option<[u8; 32]>,
    execution_version: u64,
    sealed_epochs: Vec<u64>,
) -> Result<CanonicalSnapshot, SnapshotError> {
    let entries: Vec<([u8; 32], Vec<u8>)> = sm.state
        .iter()
        .map(|(k, v)| (*k, v.clone()))
        .collect();

    let context = SnapshotExecutionContext {
        genesis_root,
        current_position: position,
        current_epoch,
        epoch_seal_hash,
        execution_version,
        sealed_epochs,
    };

    CanonicalSnapshot::new(position, sm.current_root, entries, context)
}
