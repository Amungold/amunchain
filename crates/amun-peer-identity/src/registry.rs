use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use super::certificate::PeerCertificate;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PeerRegistry {
    peers: BTreeMap<String, PeerCertificate>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self { peers: BTreeMap::new() }
    }

    pub fn register(&mut self, cert: PeerCertificate) {
        self.peers.insert(cert.peer_id.peer_id.clone(), cert);
    }

    pub fn is_registered(&self, peer_id: &str) -> bool {
        self.peers.contains_key(peer_id)
    }

    pub fn is_empty(&self) -> bool { self.peers.is_empty() }

    pub fn len(&self) -> usize {
        self.peers.len()
    }
}
