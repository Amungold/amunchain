use crate::dedup::DedupCache;
use crate::topics::Topic;

pub struct Receiver {
    dedup: DedupCache,
    received_count: u64,
}

impl Receiver {
    pub fn new() -> Self {
        Self {
            dedup: DedupCache::new(),
            received_count: 0,
        }
    }
    pub fn receive(&mut self, topic: Topic, payload: &[u8]) -> Result<bool, &'static str> {
        let mut hash_input = [0u8; 32];
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[topic.as_byte()]);
        hasher.update(payload);
        let h = hasher.finalize();
        hash_input.copy_from_slice(&h.as_bytes()[..32]);
        if self.dedup.is_duplicate(&hash_input) {
            return Ok(false);
        }
        self.dedup.mark_seen(hash_input);
        self.received_count = self.received_count.saturating_add(1);
        Ok(true)
    }
    pub fn received_count(&self) -> u64 {
        self.received_count
    }
}

impl Default for Receiver {
    fn default() -> Self {
        Self::new()
    }
}
