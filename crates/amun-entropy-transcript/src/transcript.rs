#[derive(Debug, Clone)]
pub struct EntropyTranscript {
    seed: [u8; 32],
    counter: u64,
    entries: Vec<([u8; 32], u64)>,
}

impl EntropyTranscript {
    pub fn new(seed: [u8; 32]) -> Self {
        Self {
            seed,
            counter: 0,
            entries: Vec::new(),
        }
    }

    pub fn seed(&self) -> [u8; 32] {
        self.seed
    }

    pub fn counter(&self) -> u64 {
        self.counter
    }

    pub fn increment(&mut self) {
        self.counter = self.counter.wrapping_add(1);
    }

    pub fn record(&mut self, value: [u8; 32]) {
        self.entries.push((value, self.counter));
    }

    pub fn verify_replay(&self, other: &EntropyTranscript) -> bool {
        self.entries == other.entries
    }
}
