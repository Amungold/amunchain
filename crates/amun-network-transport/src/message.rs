use amun_resource_core::ResourceId;
use serde::{Serialize, Deserialize};

/// Unified network message envelope for all AmunChain protocol messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// A newly produced block.
    BlockAnnounce(BlockAnnounce),
    /// A finality certificate for a block.
    CertificateAnnounce(CertificateAnnounce),
    /// State sync request (snapshot or delta).
    StateSyncRequest(StateSyncRequest),
    /// State sync response (snapshot or delta).
    StateSyncResponse(StateSyncResponse),
    /// Heartbeat ping.
    Ping(Ping),
    /// Heartbeat pong.
    Pong(Pong),
}

/// Block announcement: validator broadcasts a new block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockAnnounce {
    pub validator_id: ResourceId,
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub parent_hash: [u8; 32],
    pub timestamp: u64,
}

/// Certificate announcement: finality reached for a block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateAnnounce {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub certificate_hash: [u8; 32],
    pub quorum_signers: Vec<ResourceId>,
    pub timestamp: u64,
}

/// Request for state sync from a peer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSyncRequest {
    pub requester_id: [u8; 32],
    pub current_height: u64,
    pub current_state_root: [u8; 32],
    pub target_height: u64,
}

/// Response to a state sync request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateSyncResponse {
    FullSnapshot(FullSnapshotData),
    DeltaSync(DeltaSyncData),
    AlreadySynced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSnapshotData {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub history_root: [u8; 32],
    /// Serialized chunk data (canonical encoding).
    pub chunks: Vec<Vec<u8>>,
    pub chunk_root: [u8; 32],
    pub total_resources: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaSyncData {
    pub start_height: u64,
    pub end_height: u64,
    /// Serialized delta blocks.
    pub blocks: Vec<Vec<u8>>,
}

/// Heartbeat ping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ping {
    pub sender_id: [u8; 32],
    pub sequence: u64,
    pub timestamp: u64,
}

/// Heartbeat pong.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pong {
    pub sender_id: [u8; 32],
    pub sequence: u64,
    pub timestamp: u64,
    pub current_height: u64,
    pub state_root: [u8; 32],
}

impl NetworkMessage {
    /// Canonical encoding of the message for wire transmission.
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        postcard::to_stdvec(self)
            .map_err(|e| format!("Encode error: {}", e))
    }

    /// Decode a message from wire bytes.
    pub fn decode(data: &[u8]) -> Result<Self, String> {
        postcard::from_bytes(data)
            .map_err(|e| format!("Decode error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n66_message_roundtrip_block_announce() {
        let msg = NetworkMessage::BlockAnnounce(BlockAnnounce {
            validator_id: ResourceId([1u8; 32]),
            height: 42,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            parent_hash: [0xCC; 32],
            timestamp: 1000,
        });
        let encoded = msg.encode().unwrap();
        let decoded = NetworkMessage::decode(&encoded).unwrap();
        match decoded {
            NetworkMessage::BlockAnnounce(b) => {
                assert_eq!(b.height, 42);
                assert_eq!(b.block_hash, [0xAA; 32]);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn n66_message_roundtrip_certificate() {
        let msg = NetworkMessage::CertificateAnnounce(CertificateAnnounce {
            height: 10,
            block_hash: [0x11; 32],
            state_root: [0x22; 32],
            certificate_hash: [0x33; 32],
            quorum_signers: vec![ResourceId([1u8; 32]), ResourceId([2u8; 32])],
            timestamp: 2000,
        });
        let encoded = msg.encode().unwrap();
        let decoded = NetworkMessage::decode(&encoded).unwrap();
        match decoded {
            NetworkMessage::CertificateAnnounce(c) => {
                assert_eq!(c.height, 10);
                assert_eq!(c.quorum_signers.len(), 2);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn n66_message_roundtrip_ping_pong() {
        let ping = NetworkMessage::Ping(Ping {
            sender_id: [7u8; 32],
            sequence: 1,
            timestamp: 500,
        });
        let pong = NetworkMessage::Pong(Pong {
            sender_id: [8u8; 32],
            sequence: 1,
            timestamp: 501,
            current_height: 100,
            state_root: [0x99; 32],
        });
        assert!(ping.encode().is_ok());
        assert!(pong.encode().is_ok());
    }

    #[test]
    fn n66_reject_malformed_frame() {
        // Garbage bytes should not decode
        let result = NetworkMessage::decode(&[0xFF; 10]);
        assert!(result.is_err());
    }

    #[test]
    fn n66_reject_truncated_frame() {
        let msg = NetworkMessage::Ping(Ping {
            sender_id: [1u8; 32],
            sequence: 0,
            timestamp: 0,
        });
        let encoded = msg.encode().unwrap();
        // Truncate the last byte
        let result = NetworkMessage::decode(&encoded[..encoded.len()-1]);
        assert!(result.is_err());
    }
}
