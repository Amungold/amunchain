use amun_chain_position::ChainPosition;
use blake3::Hasher;

#[derive(Debug)]
pub enum SnapshotError {
    DuplicateKeys,
    UnsortedKeys,
    PositionContextMismatch,
    EpochContextMismatch,
    HashVerificationFailed,
    RootMismatch,
    GenesisMismatch,
}

#[derive(Debug, Clone)]
pub struct SnapshotExecutionContext {
    pub genesis_root: [u8; 32],
    pub current_position: ChainPosition,
    pub current_epoch: u64,
    pub epoch_seal_hash: Option<[u8; 32]>,
    pub execution_version: u64,
    pub sealed_epochs: Vec<u64>,
}

impl SnapshotExecutionContext {
    pub fn hash(&self) -> [u8; 32] {
        let mut sorted_epochs = self.sealed_epochs.clone();
        sorted_epochs.sort_unstable();

        let mut h = Hasher::new();
        h.update(b"AMUN_SNAPSHOT_CTX_V3");
        h.update(&self.genesis_root);
        h.update(&self.current_position.hash());
        h.update(&self.current_epoch.to_le_bytes());
        if let Some(seal) = self.epoch_seal_hash {
            h.update(&seal);
        }
        h.update(&self.execution_version.to_le_bytes());
        for epoch in &sorted_epochs {
            h.update(&epoch.to_le_bytes());
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.finalize().as_bytes()[..32]);
        out
    }
}

#[derive(Debug, Clone)]
pub struct CanonicalSnapshot {
    pub position: ChainPosition,
    pub epoch: u64,
    pub state_root: [u8; 32],
    pub entries: Vec<([u8; 32], Vec<u8>)>,
    pub snapshot_hash: [u8; 32],
    pub entry_count: u64,
    pub context: SnapshotExecutionContext,
}

impl CanonicalSnapshot {
    pub fn new(
        position: ChainPosition,
        state_root: [u8; 32],
        entries: Vec<([u8; 32], Vec<u8>)>,
        context: SnapshotExecutionContext,
    ) -> Result<Self, SnapshotError> {
        // Validate position/context consistency
        if position != context.current_position {
            return Err(SnapshotError::PositionContextMismatch);
        }
        if position.epoch != context.current_epoch {
            return Err(SnapshotError::EpochContextMismatch);
        }

        let mut sorted = entries;
        sorted.sort_by_key(|a| a.0);

        // Validate strictly increasing unique keys (no panic)
        for i in 1..sorted.len() {
            if sorted[i].0 <= sorted[i - 1].0 {
                return Err(SnapshotError::DuplicateKeys);
            }
        }

        let count = sorted.len() as u64;
        let ctx_hash = context.hash();

        let mut h = Hasher::new();
        h.update(b"AMUN_SNAPSHOT_V5");
        h.update(&position.hash());
        h.update(&state_root);
        h.update(&count.to_le_bytes());
        h.update(&ctx_hash);
        for (key, value) in &sorted {
            h.update(key);
            h.update(&(value.len() as u32).to_le_bytes());
            h.update(value);
        }
        let mut snapshot_hash = [0u8; 32];
        snapshot_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Ok(Self {
            position,
            epoch: position.epoch,
            state_root,
            entries: sorted,
            snapshot_hash,
            entry_count: count,
            context,
        })
    }

    pub fn verify(&self) -> bool {
        // Verify metadata consistency
        if self.position != self.context.current_position {
            return false;
        }
        if self.epoch != self.context.current_epoch {
            return false;
        }

        let ctx_hash = self.context.hash();
        let mut h = Hasher::new();
        h.update(b"AMUN_SNAPSHOT_V5");
        h.update(&self.position.hash());
        h.update(&self.state_root);
        h.update(&self.entry_count.to_le_bytes());
        h.update(&ctx_hash);
        for (key, value) in &self.entries {
            h.update(key);
            h.update(&(value.len() as u32).to_le_bytes());
            h.update(value);
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.snapshot_hash
    }
}
