use amun_chain_position::ChainPosition;
use amun_wal::WriteAheadLog;
use amun_protocol_event::ProtocolEvent;
use blake3::Hasher;
use std::fmt;

pub struct CrashPersistence {
    wal: WriteAheadLog,
    last_persisted_position: ChainPosition,
    last_persisted_hash: [u8; 32],
}

impl fmt::Debug for CrashPersistence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CrashPersistence")
            .field("last_persisted_position", &self.last_persisted_position)
            .field("last_persisted_hash", &hex::encode(self.last_persisted_hash))
            .finish()
    }
}

impl CrashPersistence {
    pub fn new(wal: WriteAheadLog) -> Self {
        Self {
            wal,
            last_persisted_position: ChainPosition::genesis(),
            last_persisted_hash: [0u8; 32],
        }
    }

    pub fn persist_event(&mut self, event: &ProtocolEvent) -> Result<(), &'static str> {
        self.wal.append_event(event).map_err(|_| "WAL write failed")?;
        self.last_persisted_position = event.position();
        
        let mut h = Hasher::new();
        h.update(b"AMUN_PERSISTENCE_V1");
        h.update(&self.last_persisted_position.hash());
        h.update(&event.hash());
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&h.finalize().as_bytes()[..32]);
        self.last_persisted_hash = hash;

        Ok(())
    }

    pub fn last_position(&self) -> ChainPosition { self.last_persisted_position }
    pub fn persistence_hash(&self) -> [u8; 32] { self.last_persisted_hash }
}
