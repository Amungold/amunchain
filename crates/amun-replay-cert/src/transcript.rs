use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct ReplayTranscript {
    pub entries: Vec<TranscriptEntry>,
    pub final_root: Option<[u8; 32]>,
}

#[derive(Debug, Clone)]
pub struct TranscriptEntry {
    pub sequence: u64,
    pub from_root: [u8; 32],
    pub to_root: [u8; 32],
    pub payload_hash: [u8; 32],
    pub transition_hash: [u8; 32],
}

impl ReplayTranscript {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            final_root: None,
        }
    }

    pub fn record(
        &mut self,
        sequence: u64,
        from_root: [u8; 32],
        to_root: [u8; 32],
        payload: &[u8],
    ) {
        let mut payload_hasher = Hasher::new();
        payload_hasher.update(payload);
        let mut payload_hash = [0u8; 32];
        payload_hash.copy_from_slice(&payload_hasher.finalize().as_bytes()[..32]);

        let mut transition_hasher = Hasher::new();
        transition_hasher.update(b"AMUN_TRANSITION_V1");
        transition_hasher.update(&sequence.to_le_bytes());
        transition_hasher.update(&from_root);
        transition_hasher.update(&to_root);
        let mut transition_hash = [0u8; 32];
        transition_hash.copy_from_slice(&transition_hasher.finalize().as_bytes()[..32]);

        self.entries.push(TranscriptEntry {
            sequence,
            from_root,
            to_root,
            payload_hash,
            transition_hash,
        });
    }

    pub fn finalize(&mut self, root: [u8; 32]) {
        self.final_root = Some(root);
    }

    pub fn verify_continuity(&self) -> bool {
        if self.entries.is_empty() {
            return true;
        }

        for i in 1..self.entries.len() {
            if self.entries[i].from_root != self.entries[i - 1].to_root {
                return false;
            }
        }
        true
    }

    pub fn reset(&mut self) {
        self.entries.clear();
        self.final_root = None;
    }
}
