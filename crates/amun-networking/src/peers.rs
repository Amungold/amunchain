/// A peer in the constitutional network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerIdentity {
    pub identity_hash: [u8; 32],
    pub civilization_id: [u8; 32],
    pub trust_level: TrustLevel,
    pub successful_syncs: u64,
    pub failed_verifications: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Unknown = 0,
    IdentityVerified = 1,
    ManifestExchanged = 2,
    SnapshotVerified = 3,
    FullySynchronized = 4,
    ConstitutionalPeer = 5,
}

/// Peer trust management.
pub struct PeerTrust;

impl PeerTrust {
    pub fn can_request_sync(peer: &PeerIdentity) -> bool {
        peer.trust_level >= TrustLevel::IdentityVerified
    }

    pub fn can_accept_manifest(peer: &PeerIdentity) -> bool {
        peer.trust_level >= TrustLevel::ManifestExchanged
    }

    pub fn upgrade_trust(peer: &mut PeerIdentity, new_level: TrustLevel) {
        if new_level > peer.trust_level {
            peer.trust_level = new_level;
        }
    }

    pub fn degrade_trust(peer: &mut PeerIdentity, _reason: &str) {
        peer.trust_level = TrustLevel::Unknown;
        peer.failed_verifications += 1;
    }
}
