use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct RecoveryCheckpoint {
    pub epoch: u64,
    pub sequence: u64,
    pub state_root: [u8; 32],
    pub wal_frame_count: u64,
    pub checkpoint_hash: [u8; 32],
}

impl RecoveryCheckpoint {
    pub fn new(epoch: u64, sequence: u64, state_root: [u8; 32], wal_frame_count: u64) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_CHECKPOINT_V1");
        h.update(&epoch.to_le_bytes());
        h.update(&sequence.to_le_bytes());
        h.update(&state_root);
        h.update(&wal_frame_count.to_le_bytes());
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&h.finalize().as_bytes()[..32]);
        Self { epoch, sequence, state_root, wal_frame_count, checkpoint_hash: hash }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_CHECKPOINT_V1");
        h.update(&self.epoch.to_le_bytes());
        h.update(&self.sequence.to_le_bytes());
        h.update(&self.state_root);
        h.update(&self.wal_frame_count.to_le_bytes());
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.checkpoint_hash
    }
}
