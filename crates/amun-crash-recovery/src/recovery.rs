use amun_wal::WriteAheadLog;

pub struct CrashRecovery {
    wal: WriteAheadLog,
}

impl CrashRecovery {
    pub fn new(wal: WriteAheadLog) -> Self {
        Self { wal }
    }

    pub fn recover_entries(&self) -> Result<Vec<amun_wal::WALEntry>, String> {
        self.wal.read_all()
    }

    pub fn verify_recovery(&self) -> Result<bool, String> {
        let entries = self.wal.read_all()?;
        if entries.is_empty() {
            return Ok(true); // Empty WAL is valid
        }
        WriteAheadLog::verify_chain_continuity(&entries)?;
        let integrity = self.wal.check_integrity()?;
        Ok(integrity.is_clean)
    }
}
