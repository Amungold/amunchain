use crate::peer_identity::PeerId;
use std::collections::BTreeMap;

/// A registry of trusted constitutional authorities.
///
/// Trust anchors are the root of cryptographic trust in the network.
/// They issue ValidatorCertificates that bind peer identities to public keys.
#[derive(Debug, Clone, Default)]
pub struct TrustAnchorRegistry {
    anchors: BTreeMap<PeerId, [u8; 32]>, // peer_id -> public_key
}

impl TrustAnchorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a trust anchor.
    pub fn register(&mut self, peer_id: PeerId, public_key: [u8; 32]) {
        self.anchors.insert(peer_id, public_key);
    }

    /// Revoke a trust anchor.
    pub fn revoke(&mut self, peer_id: &PeerId) -> bool {
        self.anchors.remove(peer_id).is_some()
    }

    /// Check if a peer is a trusted authority.
    pub fn is_trusted(&self, peer_id: &PeerId) -> bool {
        self.anchors.contains_key(peer_id)
    }

    /// Get the public key of a trust anchor.
    pub fn get_key(&self, peer_id: &PeerId) -> Option<&[u8; 32]> {
        self.anchors.get(peer_id)
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.anchors.is_empty()
    }

    /// Number of registered trust anchors.
    pub fn len(&self) -> usize {
        self.anchors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_identity::PeerKeyPair;

    #[test]
    fn n21_register_and_lookup_trust_anchor() {
        let authority = PeerKeyPair::generate();
        let peer_id = authority.peer_id();

        let mut registry = TrustAnchorRegistry::new();
        registry.register(peer_id, authority.verifying_key.to_bytes());

        assert!(registry.is_trusted(&peer_id));
        assert_eq!(registry.len(), 1);
        assert!(registry.get_key(&peer_id).is_some());
    }

    #[test]
    fn n21_revoke_trust_anchor() {
        let authority = PeerKeyPair::generate();
        let peer_id = authority.peer_id();

        let mut registry = TrustAnchorRegistry::new();
        registry.register(peer_id, authority.verifying_key.to_bytes());
        assert!(registry.is_trusted(&peer_id));

        assert!(registry.revoke(&peer_id));
        assert!(!registry.is_trusted(&peer_id));
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn n21_untrusted_peer_rejected() {
        let unknown = PeerKeyPair::generate();
        let registry = TrustAnchorRegistry::new();
        assert!(!registry.is_trusted(&unknown.peer_id()));
    }
}
