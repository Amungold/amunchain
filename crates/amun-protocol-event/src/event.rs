use amun_chain_position::ChainPosition;
use amun_binary_codec::{CanonicalEncoder, CanonicalDecoder};
use blake3::Hasher;

/// Constitutional schema version. Incremented when the binary format changes.
const EVENT_SCHEMA_VERSION: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolEvent {
    ExecuteTransaction {
        position: ChainPosition,
        payload: Vec<u8>,
        expected_root: [u8; 32],
    },
    SealEpoch {
        position: ChainPosition,
        epoch: u64,
        seal_hash: [u8; 32],
        expected_root: [u8; 32],
    },
    CreateSnapshot {
        position: ChainPosition,
        epoch: u64,
        expected_root: [u8; 32],
    },
}

impl ProtocolEvent {
    pub fn position(&self) -> ChainPosition {
        match self {
            ProtocolEvent::ExecuteTransaction { position, .. } => *position,
            ProtocolEvent::SealEpoch { position, .. } => *position,
            ProtocolEvent::CreateSnapshot { position, .. } => *position,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut h = Hasher::new();
        h.update(b"AMUN_PROTOCOL_EVENT_V4");
        let encoded = self.encode();
        h.update(&encoded);
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.finalize().as_bytes()[..32]);
        out
    }

    /// Canonical binary encode with schema version.
    /// Format: [SCHEMA_VERSION: u8][EVENT_TYPE: u8][BODY...]
    pub fn encode(&self) -> Vec<u8> {
        let mut enc = CanonicalEncoder::new();
        enc.write_u8(EVENT_SCHEMA_VERSION);
        match self {
            ProtocolEvent::ExecuteTransaction { position, payload, expected_root } => {
                enc.write_u8(0);
                enc.write_u64(position.epoch);
                enc.write_u64(position.sequence);
                let _ = enc.write_bytes(payload);
                enc.write_fixed_bytes(expected_root);
            }
            ProtocolEvent::SealEpoch { position, epoch, seal_hash, expected_root } => {
                enc.write_u8(1);
                enc.write_u64(position.epoch);
                enc.write_u64(position.sequence);
                enc.write_u64(*epoch);
                enc.write_fixed_bytes(seal_hash);
                enc.write_fixed_bytes(expected_root);
            }
            ProtocolEvent::CreateSnapshot { position, epoch, expected_root } => {
                enc.write_u8(2);
                enc.write_u64(position.epoch);
                enc.write_u64(position.sequence);
                enc.write_u64(*epoch);
                enc.write_fixed_bytes(expected_root);
            }
        }
        enc.finish()
    }

    /// Canonical binary decode with schema version check.
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut dec = CanonicalDecoder::new(data);
        
        // Constitutional: verify schema version
        let schema_version = dec.read_u8()?;
        if schema_version != EVENT_SCHEMA_VERSION {
            return None;
        }

        let event_type = dec.read_u8()?;
        let epoch = dec.read_u64()?;
        let sequence = dec.read_u64()?;
        let position = ChainPosition::new(epoch, sequence);

        let event = match event_type {
            0 => {
                let payload = dec.read_bytes()?.to_vec();
                let expected_root = dec.read_fixed_bytes::<32>()?;
                Some(ProtocolEvent::ExecuteTransaction { position, payload, expected_root })
            }
            1 => {
                let seal_epoch = dec.read_u64()?;
                let seal_hash = dec.read_fixed_bytes::<32>()?;
                let expected_root = dec.read_fixed_bytes::<32>()?;
                Some(ProtocolEvent::SealEpoch { position, epoch: seal_epoch, seal_hash, expected_root })
            }
            2 => {
                let snap_epoch = dec.read_u64()?;
                let expected_root = dec.read_fixed_bytes::<32>()?;
                Some(ProtocolEvent::CreateSnapshot { position, epoch: snap_epoch, expected_root })
            }
            _ => None,
        };

        if dec.remaining() != 0 {
            return None;
        }
        event
    }
}
