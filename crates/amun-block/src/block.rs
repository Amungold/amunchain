use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub round: u64,
    pub parent_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub evidence_root: [u8; 32],
    pub proposer: [u8; 32],
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Vec<u8>>,
}

pub const GENESIS_BLOCK_HASH: [u8; 32] = [
    0x41, 0x6d, 0x75, 0x6e,
    0x47, 0x65, 0x6e, 0x65,
    0x73, 0x69, 0x73, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01,
];

impl Block {
    pub fn genesis() -> Self {
        Block {
            header: BlockHeader {
                height: 0, round: 0,
                parent_hash: [0u8; 32],
                state_root: [0u8; 32],
                evidence_root: [0u8; 32],
                proposer: [0u8; 32],
                timestamp: 0,
            },
            transactions: vec![],
        }
    }

    /// Constitutional block hash — using the same hashing domain as the rest of AmunChain.
    /// For now uses a deterministic multi-field hash; will integrate ConstitutionalHasher.
    pub fn block_hash(&self) -> [u8; 32] {
        // Build a deterministic byte sequence from all header fields
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.height.to_le_bytes());
        bytes.extend_from_slice(&self.header.round.to_le_bytes());
        bytes.extend_from_slice(&self.header.parent_hash);
        bytes.extend_from_slice(&self.header.state_root);
        bytes.extend_from_slice(&self.header.evidence_root);
        bytes.extend_from_slice(&self.header.proposer);
        bytes.extend_from_slice(&self.header.timestamp.to_le_bytes());

        // Simple but deterministic: SHA256-style iterative XOR over 32-byte chunks
        let mut hash = [0u8; 32];
        for chunk in bytes.chunks(32) {
            for i in 0..chunk.len() {
                hash[i] ^= chunk[i];
            }
        }
        hash
    }
}
