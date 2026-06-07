use std::collections::HashSet;

/// Anti-replay protection using a sliding window of seen message hashes.
/// Prevents an attacker from resubmitting a previously valid signed message.
#[derive(Debug, Clone)]
pub struct AntiReplayGuard {
    seen_hashes: HashSet<[u8; 32]>,
    max_capacity: usize,
}

impl AntiReplayGuard {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            seen_hashes: HashSet::new(),
            max_capacity,
        }
    }

    /// Check if a message hash has been seen before.
    /// If not, record it and return true (allowed).
    /// If seen, return false (replay detected).
    pub fn check_and_record(&mut self, hash: &[u8; 32]) -> bool {
        if self.seen_hashes.contains(hash) {
            return false; // replay detected
        }
        if self.seen_hashes.len() >= self.max_capacity {
            // Evict oldest entries (simplified: clear half)
            let to_remove: Vec<[u8; 32]> = self.seen_hashes.iter().take(self.max_capacity / 2).cloned().collect();
            for h in to_remove {
                self.seen_hashes.remove(&h);
            }
        }
        self.seen_hashes.insert(*hash);
        true
    }

    /// Number of hashes currently tracked.
    pub fn tracked_count(&self) -> usize {
        self.seen_hashes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n58_allow_first_use() {
        let mut guard = AntiReplayGuard::new(100);
        assert!(guard.check_and_record(&[0xaa; 32]));
    }

    #[test]
    fn n58_detect_replay() {
        let mut guard = AntiReplayGuard::new(100);
        let hash = [0xbb; 32];
        assert!(guard.check_and_record(&hash));
        assert!(!guard.check_and_record(&hash)); // replay
    }

    #[test]
    fn n58_capacity_management() {
        let mut guard = AntiReplayGuard::new(10);
        for i in 0..20u8 {
            guard.check_and_record(&[i; 32]);
        }
        // Should have evicted some entries, staying under capacity
        assert!(guard.tracked_count() <= 10);
    }
}
