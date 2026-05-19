use crate::topics::Topic;
use crate::fanout::Fanout;
use crate::dedup::DedupCache;
use crate::constants::GOSSIP_ROUNDS;

pub struct Broadcaster {
    fanout: Fanout,
    dedup: DedupCache,
}

impl Broadcaster {
    pub fn new() -> Self {
        Self { fanout: Fanout::new(), dedup: DedupCache::new() }
    }
    pub fn broadcast(&mut self, topic: Topic, payload: &[u8], peer_count: usize) -> Result<usize, &'static str> {
        let mut hash_input = [0u8; 32];
        let mut hasher = blake3::Hasher::new();
        hasher.update(&[topic.as_byte()]);
        hasher.update(payload);
        let h = hasher.finalize();
        hash_input.copy_from_slice(&h.as_bytes()[..32]);
        if self.dedup.is_duplicate(&hash_input) { return Ok(0); }
        self.dedup.mark_seen(hash_input);
        let count = self.fanout.select_peers(peer_count, true);
        Ok(count)
    }
    pub fn max_rounds(&self) -> u8 { GOSSIP_ROUNDS }
}
