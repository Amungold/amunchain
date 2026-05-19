use blake3::Hasher;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ChainPosition { pub epoch: u64, pub sequence: u64 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochSeal {
    pub epoch: u64,
    pub epoch_root: [u8; 32],
    pub journal_root: [u8; 32],
    pub replay_root: [u8; 32],
    pub seal_hash: [u8; 32],
}

impl EpochSeal {
    pub fn new(epoch: u64, epoch_root: [u8; 32], journal_root: [u8; 32], replay_root: [u8; 32]) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_EPOCH_SEAL_V1");
        h.update(&epoch.to_le_bytes()); h.update(&epoch_root); h.update(&journal_root); h.update(&replay_root);
        let mut seal_hash = [0u8; 32];
        seal_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);
        Self { epoch, epoch_root, journal_root, replay_root, seal_hash }
    }
    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_EPOCH_SEAL_V1");
        h.update(&self.epoch.to_le_bytes()); h.update(&self.epoch_root); h.update(&self.journal_root); h.update(&self.replay_root);
        let mut c = [0u8; 32]; c.copy_from_slice(&h.finalize().as_bytes()[..32]);
        c == self.seal_hash
    }
}

impl ChainPosition {
    pub const fn genesis() -> Self { Self { epoch: 0, sequence: 0 } }
    pub const fn new(epoch: u64, sequence: u64) -> Self { Self { epoch, sequence } }
    pub fn next_sequence(&self) -> Option<Self> { self.sequence.checked_add(1).map(|s| Self { epoch: self.epoch, sequence: s }) }
    pub fn next_epoch(&self) -> Option<Self> { self.epoch.checked_add(1).map(|e| Self { epoch: e, sequence: 0 }) }
    pub fn is_genesis(&self) -> bool { self.epoch == 0 && self.sequence == 0 }
    pub fn hash(&self) -> [u8; 32] {
        let mut h = Hasher::new(); h.update(b"AMUN_CHAIN_POS_V1"); h.update(&self.epoch.to_le_bytes()); h.update(&self.sequence.to_le_bytes());
        let mut out = [0u8; 32]; out.copy_from_slice(&h.finalize().as_bytes()[..32]); out
    }
}
