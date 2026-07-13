use crate::validator_identity::ValidatorIdentity;
use std::collections::HashMap;

/// Tracks known peers in the network.
#[derive(Debug, Clone, Default)]
pub struct PeerRegistry {
    peers: HashMap<[u8; 32], ValidatorIdentity>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, identity: ValidatorIdentity) {
        self.peers.insert(identity.fingerprint(), identity);
    }

    pub fn remove(&mut self, fingerprint: &[u8; 32]) {
        self.peers.remove(fingerprint);
    }

    pub fn get(&self, fingerprint: &[u8; 32]) -> Option<&ValidatorIdentity> {
        self.peers.get(fingerprint)
    }

    pub fn count(&self) -> usize {
        self.peers.len()
    }

    pub fn list_peers(&self) -> Vec<&ValidatorIdentity> {
        self.peers.values().collect()
    }

    pub fn random_peers(&self, count: usize) -> Vec<&ValidatorIdentity> {
        use rand::seq::SliceRandom;
        let mut peers: Vec<&ValidatorIdentity> = self.peers.values().collect();
        peers.shuffle(&mut rand::thread_rng());
        peers.truncate(count);
        peers
    }
}
