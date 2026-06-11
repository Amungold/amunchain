use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Unique identifier for a peer in the network.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub [u8; 32]);

/// Public key bytes for peer authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKeyBytes(pub [u8; 32]);

/// Full peer identity: who they are, their key, and where to reach them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerIdentity {
    pub node_id: NodeId,
    pub public_key: PublicKeyBytes,
    pub address: SocketAddr,
}

impl PeerIdentity {
    pub fn new(node_id: NodeId, public_key: PublicKeyBytes, address: SocketAddr) -> Self {
        Self {
            node_id,
            public_key,
            address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n66_peer_identity_creation() {
        let peer = PeerIdentity::new(
            NodeId([1u8; 32]),
            PublicKeyBytes([2u8; 32]),
            "127.0.0.1:9000".parse().unwrap(),
        );
        assert_eq!(peer.address.port(), 9000);
    }
}
