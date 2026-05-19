use amun_chain_position::ChainPosition;
use amun_wal::WriteAheadLog;
use amun_truth_engine::TruthEngine;
use amun_crash_recovery::CrashRecovery;

/// Recovery state after a crash.
#[derive(Debug)]
pub struct RecoveryState {
    pub recovered_position: ChainPosition,
    pub recovered_root: [u8; 32],
    pub events_replayed: u64,
    pub verified: bool,
}

/// Recover consensus state from WAL after a crash.
pub fn recover_from_crash(
    wal_path: &str,
    genesis_root: [u8; 32],
) -> Result<RecoveryState, &'static str> {
    let wal = WriteAheadLog::open(std::path::PathBuf::from(wal_path))
        .map_err(|_| "WAL open failed")?;
    
    let engine = TruthEngine::new(genesis_root);
    let mut recovery = CrashRecovery::new(wal, engine);
    let result = recovery.recover()?;

    Ok(RecoveryState {
        recovered_position: ChainPosition::new(0, result.frames_replayed),
        recovered_root: result.final_root,
        events_replayed: result.frames_replayed,
        verified: result.verified,
    })
}
