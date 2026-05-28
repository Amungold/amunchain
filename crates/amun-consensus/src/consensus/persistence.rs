//! WAL Persistence & Crash Recovery
//!
//! Persistent storage for locked QC, finalized QC, and vote journal.

use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use super::state::{ConstitutionalQC, ConsensusState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentState {
    pub locked_qc: Option<ConstitutionalQC>,
    pub justified_qc: Option<ConstitutionalQC>,
    pub finalized_qc: Option<ConstitutionalQC>,
    pub current_round: u64,
    pub last_checkpoint: u64,
}

pub struct WALJournal {
    path: PathBuf,
    journal: Vec<u8>,
}

impl WALJournal {
    pub fn new(data_dir: &str) -> Self {
        let path = PathBuf::from(data_dir).join("consensus.wal");
        Self {
            path,
            journal: Vec::new(),
        }
    }

    pub fn append(&mut self, entry: &[u8]) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        file.write_all(entry)?;
        file.sync_all()?;
        Ok(())
    }

    pub fn append_state(&mut self, state: &PersistentState) -> std::io::Result<()> {
        let bytes = bincode::serialize(state).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        self.append(&bytes)
    }

    pub fn recover(&mut self) -> std::io::Result<Option<PersistentState>> {
        let mut file = match File::open(&self.path) {
            Ok(f) => f,
            Err(_) => return Ok(None),
        };
        
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        
        bincode::deserialize(&bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)).map(Some)
    }

    pub fn truncate(&mut self) -> std::io::Result<()> {
        std::fs::write(&self.path, [])?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PersistentConsensusState {
    pub state: ConsensusState,
    wal: WALJournal,
}

impl PersistentConsensusState {
    pub fn new(data_dir: &str) -> Self {
        let wal = WALJournal::new(data_dir);
        let mut instance = Self {
            state: ConsensusState::new(),
            wal,
        };
        instance.recover();
        instance
    }

    fn recover(&mut self) {
        if let Ok(Some(persistent)) = self.wal.recover() {
            self.state.current_round = persistent.current_round;
            self.state.locked_qc = persistent.locked_qc;
            self.state.justified_qc = persistent.justified_qc;
            self.state.finalized_qc = persistent.finalized_qc;
        }
    }

    pub fn persist(&mut self) -> std::io::Result<()> {
        let persistent = PersistentState {
            locked_qc: self.state.locked_qc.clone(),
            justified_qc: self.state.justified_qc.clone(),
            finalized_qc: self.state.finalized_qc.clone(),
            current_round: self.state.current_round,
            last_checkpoint: self.state.justified_height().unwrap_or(0),
        };
        self.wal.truncate()?;
        self.wal.append_state(&persistent)?;
        Ok(())
    }

    pub fn update_qc(&mut self, qc: ConstitutionalQC) -> bool {
        let result = self.state.update_qc(qc);
        if result {
            let _ = self.persist();
        }
        result
    }

    pub fn advance_round(&mut self) {
        self.state.current_round += 1;
        let _ = self.persist();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_persistence() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        
        let mut persistent = PersistentConsensusState::new(path);
        let qc = ConstitutionalQC::new(1, 3);
        
        persistent.update_qc(qc);
        assert!(persistent.state.justified_qc.is_some());
        
        // New instance should recover
        let persistent2 = PersistentConsensusState::new(path);
        assert!(persistent2.state.justified_qc.is_some());
    }
}
