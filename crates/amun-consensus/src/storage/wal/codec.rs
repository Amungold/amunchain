use crate::ccbf::{CCBFEncoder, CCBFDecoder};
use crate::storage::constants::WAL_MAGIC;

#[derive(Debug, Clone)]
pub struct WALFrame {
    pub magic: u32,
    pub length: u32,
    pub sequence: u64,
    pub prev_hash: [u8; 32],
    pub op: WALOp,
    pub entry_hash: [u8; 32],
}

#[derive(Debug, Clone)]
pub enum WALOp {
    Begin { tx_id: u64 },
    Write { tx_id: u64, key: Vec<u8>, value: Vec<u8> },
    Commit { tx_id: u64, state_root: [u8; 32] },
    Abort { tx_id: u64 },
    Checkpoint { state_root: [u8; 32], version: u64 },
}

impl WALFrame {
    pub fn compute_hash(seq: u64, prev: &[u8; 32], op: &WALOp) -> [u8; 32] {
        let mut enc = CCBFEncoder::new();
        enc.write_u64(seq);
        enc.write_fixed_hash(prev);
        op.encode(&mut enc);
        blake3::hash(&enc.into_bytes()).into()
    }
    
    pub fn new(seq: u64, prev_hash: [u8; 32], op: WALOp) -> Self {
        let entry_hash = Self::compute_hash(seq, &prev_hash, &op);
        Self { magic: WAL_MAGIC, length: 0, sequence: seq, prev_hash, op, entry_hash }
    }
    
    pub fn encode(&self, encoder: &mut CCBFEncoder) {
        encoder.write_u32(self.magic);
        encoder.write_u32(0);
        encoder.write_u64(self.sequence);
        encoder.write_fixed_hash(&self.prev_hash);
        self.op.encode(encoder);
        encoder.write_fixed_hash(&self.entry_hash);
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut enc = CCBFEncoder::new();
        self.encode(&mut enc);
        let mut bytes = enc.into_bytes();
        let len = bytes.len() as u32;
        bytes[4..8].copy_from_slice(&len.to_le_bytes());
        bytes
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut dec = CCBFDecoder::new(bytes);
        let magic = dec.read_u32()?;
        if magic != WAL_MAGIC { return None; }
        let length = dec.read_u32()?;
        let seq = dec.read_u64()?;
        let prev = dec.read_fixed_hash()?;
        let op = WALOp::decode(dec)?;
        let entry_hash = dec.read_fixed_hash()?;
        let computed = Self::compute_hash(seq, &prev, &op);
        if computed != entry_hash { return None; }
        Some(Self { magic, length, sequence: seq, prev_hash: prev, op, entry_hash })
    }
}

impl WALOp {
    pub fn encode(&self, encoder: &mut CCBFEncoder) {
        match self {
            Self::Begin { tx_id } => { encoder.write_u8(0x01); encoder.write_u64(*tx_id); }
            Self::Write { tx_id, key, value } => {
                encoder.write_u8(0x02);
                encoder.write_u64(*tx_id);
                encoder.write_bytes(key);
                encoder.write_bytes(value);
            }
            Self::Commit { tx_id, state_root } => {
                encoder.write_u8(0x03);
                encoder.write_u64(*tx_id);
                encoder.write_fixed_hash(state_root);
            }
            Self::Abort { tx_id } => { encoder.write_u8(0x04); encoder.write_u64(*tx_id); }
            Self::Checkpoint { state_root, version } => {
                encoder.write_u8(0x05);
                encoder.write_fixed_hash(state_root);
                encoder.write_u64(*version);
            }
        }
    }
    
    pub fn decode(decoder: &mut CCBFDecoder) -> Option<Self> {
        let tag = decoder.read_u8()?;
        match tag {
            0x01 => Some(Self::Begin { tx_id: decoder.read_u64()? }),
            0x02 => Some(Self::Write {
                tx_id: decoder.read_u64()?,
                key: decoder.read_bytes()?,
                value: decoder.read_bytes()?,
            }),
            0x03 => Some(Self::Commit { tx_id: decoder.read_u64()?, state_root: decoder.read_fixed_hash()? }),
            0x04 => Some(Self::Abort { tx_id: decoder.read_u64()? }),
            0x05 => Some(Self::Checkpoint { state_root: decoder.read_fixed_hash()?, version: decoder.read_u64()? }),
            _ => None,
        }
    }
}
