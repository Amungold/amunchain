pub struct PeerScorer {
    pub scores: Vec<([u8; 48], i32)>,
    pub eviction_threshold: i32,
}

impl PeerScorer {
    pub fn new() -> Self {
        Self {
            scores: Vec::new(),
            eviction_threshold: -20,
        }
    }

    pub fn reward(&mut self, peer: &[u8; 48]) {
        if let Ok(idx) = self.scores.binary_search_by_key(peer, |(k, _)| *k) {
            self.scores[idx].1 += 1;
        } else {
            let pos = self.scores.binary_search_by_key(peer, |(k, _)| *k)
                .unwrap_or_else(|e| e);
            self.scores.insert(pos, (*peer, 1));
        }
    }

    pub fn penalize(&mut self, peer: &[u8; 48], severity: i32) {
        if let Ok(idx) = self.scores.binary_search_by_key(peer, |(k, _)| *k) {
            self.scores[idx].1 -= severity;
        } else {
            let pos = self.scores.binary_search_by_key(peer, |(k, _)| *k)
                .unwrap_or_else(|e| e);
            self.scores.insert(pos, (*peer, -severity));
        }
    }

    pub fn should_evict(&self, peer: &[u8; 48]) -> bool {
        self.scores.binary_search_by_key(peer, |(k, _)| *k)
            .map(|idx| self.scores[idx].1)
            .unwrap_or(0) < self.eviction_threshold
    }

    pub fn evict(&mut self, peer: &[u8; 48]) {
        if let Ok(idx) = self.scores.binary_search_by_key(peer, |(k, _)| *k) {
            self.scores.remove(idx);
        }
    }

    pub fn decay(&mut self) {
        for (_key, score) in self.scores.iter_mut() {
            if *score > 0 {
                *score = (*score * 9) / 10;
            }
            if *score < 0 {
                *score = (*score * 11) / 10;
            }
        }
    }
}
