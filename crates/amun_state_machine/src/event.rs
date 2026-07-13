#![forbid(unsafe_code)]

use sha2::{Sha256, Digest};

/// Constitutional event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
    Transfer = 1,
    Mint = 2,
    Burn = 3,
    Delegate = 4,
    Undelegate = 5,
    Slash = 6,
    Reward = 7,
}

impl EventType {
    pub fn to_opcode(self) -> u8 {
        self as u8
    }
    
    pub fn from_opcode(opcode: u8) -> Option<Self> {
        match opcode {
            1 => Some(Self::Transfer),
            2 => Some(Self::Mint),
            3 => Some(Self::Burn),
            4 => Some(Self::Delegate),
            5 => Some(Self::Undelegate),
            6 => Some(Self::Slash),
            7 => Some(Self::Reward),
            _ => None,
        }
    }
}

/// Constitutional event with deterministic serialization
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub event_type: EventType,
    pub source: u64,
    pub target: u64,
    pub amount: i64,
    pub nonce: u64,
}

impl Event {
    pub fn new(event_type: EventType, source: u64, target: u64, amount: i64, nonce: u64) -> Self {
        Self {
            event_type,
            source,
            target,
            amount,
            nonce,
        }
    }
    
    /// Canonical binary serialization (no ambiguity)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + 8 + 8 + 8 + 8);
        bytes.push(self.event_type.to_opcode());
        bytes.extend_from_slice(&self.source.to_be_bytes());
        bytes.extend_from_slice(&self.target.to_be_bytes());
        bytes.extend_from_slice(&self.amount.to_be_bytes());
        bytes.extend_from_slice(&self.nonce.to_be_bytes());
        bytes
    }
    
    /// Compute event hash (for causal ordering)
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.to_bytes());
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_serialization() {
        let event = Event::new(EventType::Transfer, 1, 2, 100_000_000, 42);
        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), 1 + 8 + 8 + 8 + 8);
    }
}
