#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Handshake,
    HandshakeAck,
    SyncRequest,
    SyncResponse,
    BlockProposal,
    Vote,
    PeerDiscovery,
    PeerDiscoveryResponse,
    Ping,
    Pong,
}

impl MessageType {
    pub fn as_byte(&self) -> u8 {
        match self {
            MessageType::Handshake => 0x01,
            MessageType::HandshakeAck => 0x02,
            MessageType::SyncRequest => 0x03,
            MessageType::SyncResponse => 0x04,
            MessageType::BlockProposal => 0x05,
            MessageType::Vote => 0x06,
            MessageType::PeerDiscovery => 0x07,
            MessageType::PeerDiscoveryResponse => 0x08,
            MessageType::Ping => 0x09,
            MessageType::Pong => 0x0A,
        }
    }
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x01 => Some(MessageType::Handshake),
            0x02 => Some(MessageType::HandshakeAck),
            0x03 => Some(MessageType::SyncRequest),
            0x04 => Some(MessageType::SyncResponse),
            0x05 => Some(MessageType::BlockProposal),
            0x06 => Some(MessageType::Vote),
            0x07 => Some(MessageType::PeerDiscovery),
            0x08 => Some(MessageType::PeerDiscoveryResponse),
            0x09 => Some(MessageType::Ping),
            0x0A => Some(MessageType::Pong),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkMessage {
    pub msg_type: MessageType,
    pub sender_id: [u8; 32],
    pub payload: Vec<u8>,
    pub timestamp: u64,
}

impl NetworkMessage {
    pub fn new(msg_type: MessageType, sender_id: [u8; 32], payload: Vec<u8>) -> Self {
        NetworkMessage {
            msg_type,
            sender_id,
            payload,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}
