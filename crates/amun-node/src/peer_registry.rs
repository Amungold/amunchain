#![allow(dead_code)]
use crate::peer_handshake::AuthenticatedPeer;
use amun_networking::peer_identity::PeerId;
use std::collections::HashMap;

/// Registry of authenticated peers.
#[derive(Debug, Clone, Default)]
pub struct PeerRegistry {
    peers: HashMap<PeerId, AuthenticatedPeer>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an authenticated peer. Returns false if already registered.
    pub fn register(&mut self, peer: AuthenticatedPeer) -> bool {
        if self.peers.contains_key(&peer.peer_id) {
            return false;
        }
        self.peers.insert(peer.peer_id, peer);
        true
    }

    /// Remove a peer by ID.
    pub fn remove(&mut self, peer_id: &PeerId) -> bool {
        self.peers.remove(peer_id).is_some()
    }

    /// Get a peer by ID.
    pub fn get(&self, peer_id: &PeerId) -> Option<&AuthenticatedPeer> {
        self.peers.get(peer_id)
    }

    /// Number of authenticated peers.
    pub fn len(&self) -> usize {
        self.peers.len()
    }

    /// Check if registry is empty.
    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    /// Get all peer IDs.
    pub fn peer_ids(&self) -> Vec<PeerId> {
        self.peers.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_networking::crypto_identity::PeerKeyPair;

    fn create_test_peer(name: &str, port: u16) -> AuthenticatedPeer {
        let keypair = PeerKeyPair::generate();
        AuthenticatedPeer {
            peer_id: keypair.peer_id(),
            public_key: keypair.verifying_key.to_bytes(),
            node_name: name.into(),
            listen_port: port,
            protocol_version: 1,
        }
    }

    #[test]
    fn n22_6_register_and_lookup_peer() {
        let peer = create_test_peer("validator-1", 4001);
        let peer_id = peer.peer_id;
        let mut registry = PeerRegistry::new();
        assert!(registry.register(peer));
        assert_eq!(registry.len(), 1);
        assert!(registry.get(&peer_id).is_some());
    }

    #[test]
    fn n22_6_duplicate_peer_rejected() {
        let peer = create_test_peer("validator-1", 4001);
        let mut registry = PeerRegistry::new();
        assert!(registry.register(peer.clone()));
        assert!(!registry.register(peer));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn n22_6_remove_peer() {
        let peer = create_test_peer("validator-1", 4001);
        let peer_id = peer.peer_id;
        let mut registry = PeerRegistry::new();
        registry.register(peer);
        assert!(registry.remove(&peer_id));
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn n22_6_multiple_peers() {
        let p1 = create_test_peer("validator-1", 4001);
        let p2 = create_test_peer("validator-2", 4002);
        let p3 = create_test_peer("validator-3", 4003);
        let p4 = create_test_peer("validator-4", 4004);
        let mut registry = PeerRegistry::new();
        registry.register(p1);
        registry.register(p2);
        registry.register(p3);
        registry.register(p4);
        assert_eq!(registry.len(), 4);
    }
}
