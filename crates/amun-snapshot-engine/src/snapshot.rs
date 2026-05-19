use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct StateSnapshot {
    pub epoch: u64,
    pub sequence: u64,
    pub state_root: [u8; 32],
    pub state_entries: Vec<([u8; 32], Vec<u8>)>,
    pub snapshot_hash: [u8; 32],
}

impl StateSnapshot {
    pub fn new(
        epoch: u64, sequence: u64, state_root: [u8; 32],
        mut state_entries: Vec<([u8; 32], Vec<u8>)>,
    ) -> Self {
        state_entries.sort_by(|a, b| a.0.cmp(&b.0));
        let hash = Self::compute_hash(epoch, sequence, state_root, &state_entries);
        Self { epoch, sequence, state_root, state_entries, snapshot_hash: hash }
    }

    fn compute_hash(epoch: u64, sequence: u64, state_root: [u8; 32], entries: &[([u8; 32], Vec<u8>)]) -> [u8; 32] {
        let mut h = Hasher::new();
        h.update(b"AMUN_SNAPSHOT_V2");
        h.update(&epoch.to_le_bytes());
        h.update(&sequence.to_le_bytes());
        h.update(&state_root);
        for (key, value) in entries {
            h.update(key);
            h.update(&(value.len() as u32).to_le_bytes());
            h.update(value);
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&h.finalize().as_bytes()[..32]);
        hash
    }

    pub fn verify(&self) -> bool {
        // Canonicalize before verifying: sort a clone, then hash
        let mut sorted = self.state_entries.clone();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        let computed = Self::compute_hash(self.epoch, self.sequence, self.state_root, &sorted);
        computed == self.snapshot_hash
    }

    pub fn entry_count(&self) -> usize { self.state_entries.len() }
}
