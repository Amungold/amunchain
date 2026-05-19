use hashbrown::HashSet;
use crate::constants::DEDUP_CACHE_SIZE;

pub struct DedupCache {
    seen: HashSet<[u8; 32]>,
}

impl DedupCache {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    pub fn is_duplicate(&self, hash: &[u8; 32]) -> bool {
        self.seen.contains(hash)
    }

    pub fn mark_seen(&mut self, hash: [u8; 32]) {
        if self.seen.len() >= DEDUP_CACHE_SIZE {
            self.seen.clear();
        }
        self.seen.insert(hash);
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }
}
