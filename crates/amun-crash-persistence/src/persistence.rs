use amun_wal::WriteAheadLog;

pub struct CrashPersistence {
    wal: WriteAheadLog,
}

impl CrashPersistence {
    pub fn new(wal: WriteAheadLog) -> Self {
        Self { wal }
    }

    pub fn append_event(&mut self, event_type: &str, payload: &str) -> Result<u64, String> {
        self.wal.append(event_type, payload)
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.wal.shutdown()
    }
}
