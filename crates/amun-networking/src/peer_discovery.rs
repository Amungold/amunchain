use crate::crypto_identity::{PeerKeyPair, SignedMessage};
use crate::peer_identity::{PeerId, PeerIdentity};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Announcement message broadcast by a peer to declare its presence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerAnnouncement {
    pub peer_id: PeerId,
    pub public_key: [u8; 32],
    pub address: String,
    pub timestamp: u64,
}

impl PeerAnnouncement {
    /// Create a signed peer announcement.
    pub fn sign(keypair: &PeerKeyPair, address: String, timestamp: u64) -> (Self, SignedMessage) {
        let announcement = Self {
            peer_id: keypair.peer_id(),
            public_key: keypair.verifying_key.to_bytes(),
            address,
            timestamp,
        };
        let payload = serde_json::to_vec(&announcement).unwrap_or_default();
        let signed = SignedMessage::sign(keypair, payload);
        (announcement, signed)
    }

    /// Verify a signed announcement.
    pub fn verify(announcement: &Self, signed: &SignedMessage) -> bool {
        signed.verify() && signed.sender_peer_id() == announcement.peer_id
    }
}

/// A registry of known peers with expiration.
#[derive(Debug, Clone, Default)]
pub struct PeerRegistry {
    peers: BTreeMap<PeerId, PeerIdentity>,
    last_seen: BTreeMap<PeerId, u64>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register or refresh a peer.
    pub fn register(&mut self, identity: PeerIdentity, timestamp: u64) {
        let peer_id = identity.peer_id;
        self.peers.insert(peer_id, identity);
        self.last_seen.insert(peer_id, timestamp);
    }

    /// Remove peers not seen since the given timestamp.
    pub fn expire(&mut self, before_timestamp: u64) -> usize {
        let expired: Vec<PeerId> = self
            .last_seen
            .iter()
            .filter(|(_, &ts)| ts < before_timestamp)
            .map(|(id, _)| *id)
            .collect();
        let count = expired.len();
        for id in &expired {
            self.peers.remove(id);
            self.last_seen.remove(id);
        }
        count
    }

    /// Get a peer by ID.
    pub fn get(&self, peer_id: &PeerId) -> Option<&PeerIdentity> {
        self.peers.get(peer_id)
    }

    /// Number of known peers.
    pub fn len(&self) -> usize {
        self.peers.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    /// Get all known peer IDs.
    pub fn peer_ids(&self) -> Vec<PeerId> {
        self.peers.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n20_7_peer_announcement_sign_and_verify() {
        let keypair = PeerKeyPair::generate();
        let (announcement, signed) =
            PeerAnnouncement::sign(&keypair, "127.0.0.1:7001".into(), 1000);
        assert!(PeerAnnouncement::verify(&announcement, &signed));
    }

    #[test]
    fn n20_7_tampered_announcement_rejected() {
        let keypair = PeerKeyPair::generate();
        let (announcement, mut signed) =
            PeerAnnouncement::sign(&keypair, "127.0.0.1:7001".into(), 1000);
        // Tamper with the signed payload
        signed.payload = b"tampered data".to_vec();
        assert!(!PeerAnnouncement::verify(&announcement, &signed));
    }

    #[test]
    fn n20_7_peer_registry_register_and_lookup() {
        let keypair = PeerKeyPair::generate();
        let peer_id = keypair.peer_id();
        let identity = PeerIdentity::new(
            peer_id,
            keypair.verifying_key.to_bytes(),
            "127.0.0.1:7001".parse().unwrap(),
        );

        let mut registry = PeerRegistry::new();
        registry.register(identity, 1000);
        assert_eq!(registry.len(), 1);
        assert!(registry.get(&peer_id).is_some());
    }

    #[test]
    fn n20_7_peer_expiration() {
        let keypair = PeerKeyPair::generate();
        let identity = PeerIdentity::new(
            keypair.peer_id(),
            keypair.verifying_key.to_bytes(),
            "127.0.0.1:7001".parse().unwrap(),
        );

        let mut registry = PeerRegistry::new();
        registry.register(identity, 1000);
        assert_eq!(registry.len(), 1);

        // Expire peers not seen since timestamp 2000
        let expired = registry.expire(2000);
        assert_eq!(expired, 1);
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn n20_7_duplicate_peer_updates_timestamp() {
        let keypair = PeerKeyPair::generate();
        let identity = PeerIdentity::new(
            keypair.peer_id(),
            keypair.verifying_key.to_bytes(),
            "127.0.0.1:7001".parse().unwrap(),
        );

        let mut registry = PeerRegistry::new();
        registry.register(identity.clone(), 1000);
        registry.register(identity, 2000);

        // Should still be 1 peer
        assert_eq!(registry.len(), 1);
        // Expire at 1500 should not remove it (last seen at 2000)
        let expired = registry.expire(1500);
        assert_eq!(expired, 0);
    }

    #[test]
    fn n20_7_registry_deterministic() {
        let kp1 = PeerKeyPair::generate();
        let kp2 = PeerKeyPair::generate();

        let id1 = PeerIdentity::new(
            kp1.peer_id(),
            kp1.verifying_key.to_bytes(),
            "127.0.0.1:7001".parse().unwrap(),
        );
        let id2 = PeerIdentity::new(
            kp2.peer_id(),
            kp2.verifying_key.to_bytes(),
            "127.0.0.1:7002".parse().unwrap(),
        );

        let mut registry = PeerRegistry::new();
        registry.register(id1, 1000);
        registry.register(id2, 1000);

        let ids = registry.peer_ids();
        assert_eq!(ids.len(), 2);
        // BTreeMap guarantees deterministic ordering
        assert!(ids[0] < ids[1]);
    }
}
