#![forbid(unsafe_code)]

use sha2::{Sha256, Digest};
use amun_state_machine::{Event, CanonicalOrdering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BlockVersion {
    V1 = 1,
}

impl BlockVersion {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub version: u32,
    pub height: u64,
    pub parent_block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub receipts_root: [u8; 32],
    pub execution_trace_root: [u8; 32],
    pub event_root: [u8; 32],
    pub snapshot_root: [u8; 32],
    pub timestamp_logical: u64,  // Logical time, not wall-clock
    pub proposer_id: u64,
}

impl BlockHeader {
    pub fn new(
        height: u64,
        parent_hash: [u8; 32],
        state_root: [u8; 32],
        receipts_root: [u8; 32],
        trace_root: [u8; 32],
        event_root: [u8; 32],
        snapshot_root: [u8; 32],
        proposer_id: u64,
    ) -> Self {
        Self {
            version: BlockVersion::V1.to_u32(),
            height,
            parent_block_hash: parent_hash,
            state_root,
            receipts_root,
            execution_trace_root: trace_root,
            event_root,
            snapshot_root,
            timestamp_logical: height,  // Logical time = block height
            proposer_id,
        }
    }
    
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + 8 + 32 + 32 + 32 + 32 + 32 + 32 + 8 + 8);
        bytes.extend_from_slice(&self.version.to_be_bytes());
        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.extend_from_slice(&self.parent_block_hash);
        bytes.extend_from_slice(&self.state_root);
        bytes.extend_from_slice(&self.receipts_root);
        bytes.extend_from_slice(&self.execution_trace_root);
        bytes.extend_from_slice(&self.event_root);
        bytes.extend_from_slice(&self.snapshot_root);
        bytes.extend_from_slice(&self.timestamp_logical.to_be_bytes());
        bytes.extend_from_slice(&self.proposer_id.to_be_bytes());
        bytes
    }
    
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.canonical_bytes());
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone)]
pub struct BlockBody {
    pub events: Vec<Event>,
}

impl BlockBody {
    pub fn new(events: Vec<Event>) -> Self {
        Self { events }
    }
    
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.events.len() as u64).to_be_bytes());
        for event in &self.events {
            bytes.extend_from_slice(&event.to_bytes());
        }
        bytes
    }
    
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.canonical_bytes());
        hasher.finalize().into()
    }
    
    pub fn sort_canonical(&mut self, block_height: u64) {
        CanonicalOrdering::sort_events(&mut self.events, block_height);
    }
    
    pub fn is_canonically_ordered(&self, block_height: u64) -> bool {
        CanonicalOrdering::is_canonically_ordered(&self.events, block_height)
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub body: BlockBody,
}

impl Block {
    pub fn new(header: BlockHeader, body: BlockBody) -> Self {
        Self { header, body }
    }
    
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.header.canonical_bytes());
        hasher.update(&self.body.canonical_bytes());
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_state_machine::{EventType, Fixed};
    
    #[test]
    fn test_block_header_hash() {
        let header = BlockHeader::new(
            1,
            [0; 32],
            [1; 32],
            [2; 32],
            [3; 32],
            [4; 32],
            [5; 32],
            100,
        );
        let hash = header.hash();
        assert_ne!(hash, [0; 32]);
    }
    
    #[test]
    fn test_block_body_sorting() {
        let mut events = vec![
            Event::new(EventType::Transfer, 2, 1, Fixed::from_int(100).raw(), 1),
            Event::new(EventType::Mint, 0, 1, Fixed::from_int(1000).raw(), 0),
            Event::new(EventType::Transfer, 1, 2, Fixed::from_int(50).raw(), 0),
        ];
        
        let mut body = BlockBody::new(events);
        body.sort_canonical(100);
        
        assert!(body.is_canonically_ordered(100));
        assert_eq!(body.events[0].event_type, EventType::Mint);
    }
}
