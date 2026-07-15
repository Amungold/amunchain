use crate::message::{MessageType, NetworkMessage};
use amun_validator_api::error::{NetworkError, NetworkErrorCode, PlatformError, PlatformResult};

const FRAME_MAGIC: u32 = 0x414D554E;
pub const MAX_PAYLOAD_SIZE: usize = 1_048_576;

pub struct BinaryCodec;

impl BinaryCodec {
    pub fn encode(msg: &NetworkMessage) -> PlatformResult<Vec<u8>> {
        if msg.payload.len() > MAX_PAYLOAD_SIZE {
            return Err(PlatformError::Network(NetworkError::new(
                NetworkErrorCode::HandshakeFailed,
                format!(
                    "Payload size {} exceeds max {}",
                    msg.payload.len(),
                    MAX_PAYLOAD_SIZE
                ),
            )));
        }
        let payload = Self::encode_message(msg);
        let checksum = Self::fnv1a(&payload);
        let body_len = 4 + 2 + 2 + 4 + payload.len() + 4;
        let mut frame = Vec::with_capacity(4 + body_len);
        frame.extend_from_slice(&(body_len as u32).to_le_bytes());
        frame.extend_from_slice(&FRAME_MAGIC.to_le_bytes());
        frame.extend_from_slice(&1u16.to_le_bytes());
        frame.extend_from_slice(&0u16.to_le_bytes());
        frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        frame.extend_from_slice(&payload);
        frame.extend_from_slice(&checksum.to_le_bytes());
        Ok(frame)
    }

    pub fn decode(data: &[u8]) -> Option<NetworkMessage> {
        if data.len() < 4 {
            return None;
        }
        let frame_len = u32::from_le_bytes(data[0..4].try_into().ok()?) as usize;
        if data.len() < 4 + frame_len {
            return None;
        }
        let body = &data[4..4 + frame_len];
        if body.len() < 16 {
            return None;
        }
        let magic = u32::from_le_bytes(body[0..4].try_into().ok()?);
        if magic != FRAME_MAGIC {
            return None;
        }
        let payload_len = u32::from_le_bytes(body[8..12].try_into().ok()?) as usize;
        if 12 + payload_len + 4 > body.len() {
            return None;
        }
        let payload = &body[12..12 + payload_len];
        let expected_checksum = u32::from_le_bytes(
            body[12 + payload_len..12 + payload_len + 4]
                .try_into()
                .ok()?,
        );
        if Self::fnv1a(payload) != expected_checksum {
            return None;
        }
        Self::decode_message(payload)
    }

    fn encode_message(msg: &NetworkMessage) -> Vec<u8> {
        let mut buf = Vec::with_capacity(1 + 32 + 4 + msg.payload.len() + 8);
        buf.push(msg.msg_type.as_byte());
        buf.extend_from_slice(&msg.sender_id);
        buf.extend_from_slice(&(msg.payload.len() as u32).to_le_bytes());
        buf.extend_from_slice(&msg.payload);
        buf.extend_from_slice(&msg.timestamp.to_le_bytes());
        buf
    }

    fn decode_message(data: &[u8]) -> Option<NetworkMessage> {
        if data.len() < 45 {
            return None;
        }
        let msg_type = MessageType::from_byte(data[0])?;
        let sender_id: [u8; 32] = data[1..33].try_into().ok()?;
        let payload_len = u32::from_le_bytes(data[33..37].try_into().ok()?) as usize;
        if 37 + payload_len + 8 > data.len() {
            return None;
        }
        let payload = data[37..37 + payload_len].to_vec();
        let timestamp = u64::from_le_bytes(
            data[37 + payload_len..37 + payload_len + 8]
                .try_into()
                .ok()?,
        );
        Some(NetworkMessage {
            msg_type,
            sender_id,
            payload,
            timestamp,
        })
    }

    fn fnv1a(data: &[u8]) -> u32 {
        let mut hash: u32 = 0x811C9DC5;
        for &byte in data {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(0x01000193);
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::MessageType;

    #[test]
    fn test_roundtrip() {
        let msg = NetworkMessage::new(MessageType::Ping, [1u8; 32], vec![1, 2, 3]);
        let encoded = BinaryCodec::encode(&msg).unwrap();
        let decoded = BinaryCodec::decode(&encoded).unwrap();
        assert_eq!(decoded.sender_id, [1u8; 32]);
    }

    #[test]
    fn test_corrupt_frame_rejected() {
        let msg = NetworkMessage::new(MessageType::Ping, [1u8; 32], vec![1, 2, 3]);
        let mut data = BinaryCodec::encode(&msg).unwrap();
        if !data.is_empty() {
            data[4] ^= 0xFF;
        }
        assert!(BinaryCodec::decode(&data).is_none());
    }

    #[test]
    fn test_truncated_frame_rejected() {
        let msg = NetworkMessage::new(MessageType::Ping, [1u8; 32], vec![1, 2, 3]);
        let data = BinaryCodec::encode(&msg).unwrap();
        assert!(BinaryCodec::decode(&data[..data.len() / 2]).is_none());
    }

    #[test]
    fn test_max_size_rejected() {
        let msg = NetworkMessage::new(
            MessageType::Ping,
            [1u8; 32],
            vec![0u8; MAX_PAYLOAD_SIZE + 1],
        );
        assert!(BinaryCodec::encode(&msg).is_err());
    }
}
