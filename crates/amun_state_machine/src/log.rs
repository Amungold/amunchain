#![forbid(unsafe_code)]

use sha2::{Sha256, Digest};
use crate::event::Event;

/// Causal execution log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub event: Event,
    pub event_hash: [u8; 32],
    pub causal_parent: [u8; 32],
    pub timestamp: u64,
}

impl LogEntry {
    pub fn new(event: Event, causal_parent: [u8; 32], timestamp: u64) -> Self {
        let event_hash = event.hash();
        Self {
            event,
            event_hash,
            causal_parent,
            timestamp,
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.event.to_bytes();
        bytes.extend_from_slice(&self.causal_parent);
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes
    }
    
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.to_bytes());
        hasher.finalize().into()
    }
}

/// Causal execution log (for replay)
#[derive(Debug, Clone)]
pub struct CausalLog {
    entries: Vec<LogEntry>,
    root_hash: [u8; 32],
}

impl CausalLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            root_hash: [0; 32],
        }
    }
    
    pub fn push(&mut self, entry: LogEntry) {
        self.entries.push(entry);
        self.recompute_root();
    }
    
    pub fn recompute_root(&mut self) {
        let mut hasher = Sha256::new();
        for entry in &self.entries {
            hasher.update(entry.hash());
        }
        self.root_hash = hasher.finalize().into();
    }
    
    pub fn root_hash(&self) -> [u8; 32] {
        self.root_hash
    }
    
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }
    
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for CausalLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventType;
    
    #[test]
    fn test_log_root_hash() {
        let mut log = CausalLog::new();
        let event = Event::new(EventType::Transfer, 1, 2, 100_000_000, 0);
        let entry = LogEntry::new(event, [0; 32], 1);
        log.push(entry);
        
        assert_eq!(log.len(), 1);
        assert_ne!(log.root_hash(), [0; 32]);
    }
}
