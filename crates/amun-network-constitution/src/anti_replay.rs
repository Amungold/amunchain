use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct AntiReplayGuard {
    seen_nonces: BTreeMap<u64, u64>,
    max_seen_per_sender: usize,
}

impl AntiReplayGuard {
    pub fn new() -> Self {
        Self {
            seen_nonces: BTreeMap::new(),
            max_seen_per_sender: 1000,
        }
    }

    pub fn check_and_record(
        &mut self,
        sender_id: u64,
        nonce: u64,
    ) -> Result<(), &'static str> {
        let key = sender_id;
        if let Some(&last_nonce) = self.seen_nonces.get(&key) {
            if nonce <= last_nonce {
                return Err("replay detected: nonce too low");
            }
        }

        if self.seen_nonces.len() >= self.max_seen_per_sender * 100 {
            self.prune_oldest();
        }

        self.seen_nonces.insert(key, nonce);
        Ok(())
    }

    fn prune_oldest(&mut self) {
        let mut entries: Vec<(u64, u64)> = self.seen_nonces.iter()
            .map(|(k, v)| (*k, *v))
            .collect();
        entries.sort_by(|a, b| a.1.cmp(&b.1));
        let remove_count = entries.len() / 2;
        for (key, _) in entries.iter().take(remove_count) {
            self.seen_nonces.remove(key);
        }
    }

    pub fn highest_nonce(&self, sender_id: u64) -> Option<u64> {
        self.seen_nonces.get(&sender_id).copied()
    }
}

impl Default for AntiReplayGuard {
    fn default() -> Self { Self::new() }
}
