use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// A unique peer identifier. Derived from the validator public key.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PeerId(pub [u8; 32]);

impl PeerId {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// Full identity of a peer on the network.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PeerIdentity {
    pub peer_id: PeerId,
    pub public_key: [u8; 32],
    pub address: SocketAddr,
}

impl PeerIdentity {
    pub fn new(peer_id: PeerId, public_key: [u8; 32], address: SocketAddr) -> Self {
        Self {
            peer_id,
            public_key,
            address,
        }
    }
}
