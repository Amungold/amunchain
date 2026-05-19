use amun_wal::WriteAheadLog;
use amun_truth_engine::TruthEngine;

pub struct CrashRecovery { wal: WriteAheadLog, engine: TruthEngine }

impl CrashRecovery {
    pub fn new(wal: WriteAheadLog, engine: TruthEngine) -> Self { Self { wal, engine } }

    pub fn recover(&mut self) -> Result<RecoveryResult, &'static str> {
        let events = self.wal.iter_events().map_err(|_| "WAL read failed")?;
        let mut last_root = self.engine.genesis_root();
        for event in &events {
            last_root = self.engine.apply_event(event)?;
        }
        let replay_root = self.engine.compute_chain_root(events.len() as u64)
            .map_err(|_| "replay verification failed")?;
        if replay_root != last_root { return Err("replay root mismatch after recovery"); }
        Ok(RecoveryResult { frames_replayed: events.len() as u64, final_root: last_root, verified: true })
    }
}

pub struct RecoveryResult { pub frames_replayed: u64, pub final_root: [u8; 32], pub verified: bool }
