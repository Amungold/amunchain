use amun_state_sync::sync_package::ConstitutionalSyncPackage;

use serde::{Deserialize, Serialize};

/// Messages exchanged between validators over the network.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// Request a state sync package at a specific height.
    StateSyncRequest {
        request_id: [u8; 32],
        height: u64,
        requester_id: [u8; 32],
    },
    /// Response containing the sync package.
    StateSyncResponse {
        request_id: [u8; 32],
        package: Box<ConstitutionalSyncPackage>,
        responder_id: [u8; 32],
    },
    /// Announce that a validator has a new block available.
    BlockAnnounce {
        block_height: u64,
        block_hash: [u8; 32],
        validator_id: [u8; 32],
    },
    /// Request a specific block by hash.
    BlockRequest {
        block_hash: [u8; 32],
        requester_id: [u8; 32],
    },
    /// Peer discovery ping.
    Ping {
        sender_id: [u8; 32],
        sender_address: String,
        sender_port: u16,
    },
    /// Peer discovery pong.
    Pong {
        sender_id: [u8; 32],
        peer_list: Vec<(String, u16)>,
    },
}

impl NetworkMessage {
    pub fn message_type(&self) -> &str {
        match self {
            NetworkMessage::StateSyncRequest { .. } => "STATE_SYNC_REQUEST",
            NetworkMessage::StateSyncResponse { .. } => "STATE_SYNC_RESPONSE",
            NetworkMessage::BlockAnnounce { .. } => "BLOCK_ANNOUNCE",
            NetworkMessage::BlockRequest { .. } => "BLOCK_REQUEST",
            NetworkMessage::Ping { .. } => "PING",
            NetworkMessage::Pong { .. } => "PONG",
        }
    }
}
