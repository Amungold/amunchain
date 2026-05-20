use crate::law::StorageLaw;
use crate::wal::{WalPayload, WriteAheadLog};
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::PublicHash32;
use heapless::Vec;

pub struct PersistentStore {
    pub wal: WriteAheadLog,
    pub state_root: PublicHash32,
    pub block_height: u64,
    pending_count: usize,
}

impl PersistentStore {
    pub fn new() -> Self {
        Self {
            wal: WriteAheadLog::new(),
            state_root: PublicHash32::default(),
            block_height: 0,
            pending_count: 0,
        }
    }

    pub fn set(&mut self, key: Vec<u8, 32>, value: Vec<u8, 64>) -> AmunResult<()> {
        if self.pending_count >= StorageLaw::MAX_ENTRIES_PER_COMMIT {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x000B,
                0x0010,
            ));
        }
        self.wal.append(WalPayload::Set { key, value })?;
        self.pending_count = self.pending_count.saturating_add(1);
        Ok(())
    }

    pub fn commit(&mut self) -> AmunResult<PublicHash32> {
        let new_root = self.compute_root();
        let seq = self.wal.next_sequence;
        self.wal.append(WalPayload::Commit {
            sequence: seq,
            state_root: new_root,
        })?;
        self.state_root = new_root;
        self.block_height = self.block_height.saturating_add(1);
        self.pending_count = 0;
        Ok(self.state_root)
    }

    fn compute_root(&self) -> PublicHash32 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.state_root.0);
        hasher.update(&self.block_height.to_le_bytes());
        let h = hasher.finalize();
        let mut r = PublicHash32::default();
        r.0.copy_from_slice(&h.as_bytes()[..32]);
        r
    }

    pub fn apply_replay(&mut self, payload: &WalPayload) -> AmunResult<()> {
        match payload {
            WalPayload::Set { .. } | WalPayload::Delete { .. } => Ok(()),
            WalPayload::Commit { state_root, .. } => {
                self.state_root = *state_root;
                Ok(())
            }
        }
    }
}

impl Default for PersistentStore {
    fn default() -> Self {
        Self::new()
    }
}
