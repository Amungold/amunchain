use blake3::Hasher;

const NETWORK_FRAME_VERSION: u8 = 1;
const MAX_FRAME_PAYLOAD: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkMessageType {
    ConsensusVote,
    BlockProposal,
    QuorumCertificate,
    ValidatorAttestation,
    SnapshotChunk,
    Heartbeat,
}

impl NetworkMessageType {
    pub fn tag(&self) -> u8 {
        match self {
            NetworkMessageType::ConsensusVote => 0,
            NetworkMessageType::BlockProposal => 1,
            NetworkMessageType::QuorumCertificate => 2,
            NetworkMessageType::ValidatorAttestation => 3,
            NetworkMessageType::SnapshotChunk => 4,
            NetworkMessageType::Heartbeat => 5,
        }
    }

    pub fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            0 => Some(NetworkMessageType::ConsensusVote),
            1 => Some(NetworkMessageType::BlockProposal),
            2 => Some(NetworkMessageType::QuorumCertificate),
            3 => Some(NetworkMessageType::ValidatorAttestation),
            4 => Some(NetworkMessageType::SnapshotChunk),
            5 => Some(NetworkMessageType::Heartbeat),
            _ => None,
        }
    }

    /// Scheduling priority for each message type.
    /// Lower number = higher priority (processed first).
    pub fn scheduling_priority(&self) -> u8 {
        match self {
            NetworkMessageType::ConsensusVote => 0,
            NetworkMessageType::BlockProposal => 1,
            NetworkMessageType::QuorumCertificate => 2,
            NetworkMessageType::ValidatorAttestation => 3,
            NetworkMessageType::Heartbeat => 4,
            NetworkMessageType::SnapshotChunk => 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkFrame {
    pub msg_type: NetworkMessageType,
    pub payload: Vec<u8>,
    pub frame_hash: [u8; 32],
}

impl NetworkFrame {
    pub fn encode(msg_type: NetworkMessageType, payload: &[u8]) -> Option<Vec<u8>> {
        if payload.len() > MAX_FRAME_PAYLOAD {
            return None;
        }
        let total_len = (7 + payload.len()) as u32;
        let mut buf = Vec::with_capacity(total_len as usize);
        buf.extend_from_slice(&total_len.to_le_bytes());
        buf.push(NETWORK_FRAME_VERSION);
        buf.push(msg_type.tag());
        buf.push(0u8);
        buf.extend_from_slice(payload);
        Some(buf)
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None;
        }
        let frame_len = u32::from_le_bytes(data[..4].try_into().ok()?) as usize;
        if frame_len != data.len() {
            return None;
        }
        let version = data[4];
        if version != NETWORK_FRAME_VERSION {
            return None;
        }
        if data[6] != 0 {
            return None;
        }
        let msg_type = NetworkMessageType::from_tag(data[5])?;
        let payload = data[7..].to_vec();
        if payload.len() > MAX_FRAME_PAYLOAD {
            return None;
        }

        let mut h = Hasher::new();
        h.update(b"AMUN_NET_FRAME_V1");
        h.update(&frame_len.to_le_bytes());
        h.update(&[version]);
        h.update(&[data[5]]);
        h.update(&payload);
        let mut frame_hash = [0u8; 32];
        frame_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Some(Self {
            msg_type,
            payload,
            frame_hash,
        })
    }
}
