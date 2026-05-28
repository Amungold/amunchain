#![forbid(unsafe_code)]

//! Canonical Event Ordering Protocol
//! 
//! This module defines the deterministic ordering of events
//! that all validators MUST follow to achieve consensus.

use amun_state_machine::{Event, EventType};

/// Canonical ordering key for events
/// Order: (block_height, sender_id, nonce, event_type_priority, event_hash)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanonicalEventKey {
    pub block_height: u64,
    pub sender_id: u64,
    pub nonce: u64,
    pub event_type_priority: u8,
    pub event_hash: [u8; 32],
}

impl CanonicalEventKey {
    /// Create canonical key from event and block context
    pub fn from_event(event: &Event, block_height: u64) -> Self {
        let priority = match event.event_type {
            EventType::Mint => 1,
            EventType::Reward => 2,
            EventType::Transfer => 3,
            EventType::Delegate => 4,
            EventType::Undelegate => 5,
            EventType::Slash => 6,
            EventType::Burn => 7,
        };
        
        Self {
            block_height,
            sender_id: event.source,
            nonce: event.nonce,
            event_type_priority: priority,
            event_hash: event.hash(),
        }
    }
    
    /// Convert to bytes for hashing
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + 8 + 8 + 1 + 32);
        bytes.extend_from_slice(&self.block_height.to_be_bytes());
        bytes.extend_from_slice(&self.sender_id.to_be_bytes());
        bytes.extend_from_slice(&self.nonce.to_be_bytes());
        bytes.push(self.event_type_priority);
        bytes.extend_from_slice(&self.event_hash);
        bytes
    }
}

/// Canonical Event Ordering Protocol
/// Guarantees deterministic ordering across all validators
pub struct CanonicalEventOrdering;

impl CanonicalEventOrdering {
    /// Sort events according to canonical protocol
    pub fn sort_events(events: &mut [Event], block_height: u64) {
        events.sort_by(|a, b| {
            let key_a = CanonicalEventKey::from_event(a, block_height);
            let key_b = CanonicalEventKey::from_event(b, block_height);
            key_a.cmp(&key_b)
        });
    }
    
    /// Verify events are in canonical order
    pub fn is_ordered(events: &[Event], block_height: u64) -> bool {
        for i in 1..events.len() {
            let key_prev = CanonicalEventKey::from_event(&events[i-1], block_height);
            let key_curr = CanonicalEventKey::from_event(&events[i], block_height);
            if key_prev > key_curr {
                return false;
            }
        }
        true
    }
    
    /// Get canonical order specification for constitution
    pub fn specification() -> &'static str {
        r#"
        ============================================================
        CANONICAL EVENT ORDERING CONSTITUTION v1.0
        ============================================================
        
        ORDERING RULES (applied in sequence):
        
        1. PRIMARY KEY: block_height (ascending)
           - Events from lower blocks come first
        
        2. SECONDARY KEY: sender_id (ascending)
           - Events from lower sender IDs come first
        
        3. TERTIARY KEY: nonce (ascending)
           - Events with lower nonce come first
        
        4. QUATERNARY KEY: event_type_priority (ascending)
           - Priority order:
             1 = Mint
             2 = Reward
             3 = Transfer
             4 = Delegate
             5 = Undelegate
             6 = Slash
             7 = Burn
        
        5. TERTIARY KEY: event_hash (lexicographic)
           - Final deterministic tie-breaker
        
        CONSTITUTIONAL REQUIREMENTS:
        
        - ALL validators MUST use EXACTLY this ordering
        - Deviation from this order = CONSENSUS VIOLATION
        - Blocks with unordered events MUST be REJECTED
        
        ENFORCEMENT:
        
        - Block producers MUST sort events before inclusion
        - Validators MUST verify ordering during validation
        - Nodes MUST reject blocks with incorrect ordering
        
        ============================================================
        "#
    }
}

/// Ordering validator for blocks
pub struct OrderingValidator;

impl OrderingValidator {
    /// Validate that all events in a block are canonically ordered
    pub fn validate(events: &[Event], block_height: u64) -> OrderingResult {
        if !CanonicalEventOrdering::is_ordered(events, block_height) {
            return OrderingResult::Invalid("Events not in canonical order".to_string());
        }
        
        // Check for duplicate nonces from same sender
        let mut seen: std::collections::BTreeMap<(u64, u64), u64> = std::collections::BTreeMap::new();
        for event in events {
            let key = (event.source, event.nonce);
            if let Some(existing) = seen.get(&key) {
                return OrderingResult::Invalid(
                    format!("Duplicate nonce {} from sender {} at position {}", 
                            event.nonce, event.source, existing)
                );
            }
            seen.insert(key, event.nonce);
        }
        
        OrderingResult::Valid
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderingResult {
    Valid,
    Invalid(String),
}

impl OrderingResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_state_machine::{Event, EventType};
    
    #[test]
    fn test_canonical_ordering() {
        let mut events = vec![
            Event::new(EventType::Transfer, 2, 1, 100_000_000, 1),
            Event::new(EventType::Mint, 0, 1, 1_000_000_000, 0),
            Event::new(EventType::Transfer, 1, 2, 50_000_000, 0),
        ];
        
        CanonicalEventOrdering::sort_events(&mut events, 100);
        
        assert_eq!(events[0].event_type, EventType::Mint);
        assert_eq!(events[1].event_type, EventType::Transfer);
        assert_eq!(events[2].event_type, EventType::Transfer);
        assert_eq!(events[1].source, 1);
        assert_eq!(events[2].source, 2);
    }
    
    #[test]
    fn test_ordering_validation() {
        let mut events = vec![
            Event::new(EventType::Transfer, 2, 1, 100_000_000, 1),
            Event::new(EventType::Mint, 0, 1, 1_000_000_000, 0),
        ];
        
        // Unsorted should be invalid
        let result = OrderingValidator::validate(&events, 100);
        assert!(!result.is_valid());
        
        // Sorted should be valid
        CanonicalEventOrdering::sort_events(&mut events, 100);
        let result = OrderingValidator::validate(&events, 100);
        assert!(result.is_valid());
    }
    
    #[test]
    fn test_duplicate_nonce_detection() {
        let events = vec![
            Event::new(EventType::Transfer, 1, 2, 100_000_000, 0),
            Event::new(EventType::Transfer, 1, 3, 50_000_000, 0),  // Same nonce!
        ];
        
        let result = OrderingValidator::validate(&events, 100);
        assert!(!result.is_valid());
    }
    
    #[test]
    fn test_key_comparison() {
        let event1 = Event::new(EventType::Transfer, 1, 2, 100_000_000, 5);
        let event2 = Event::new(EventType::Transfer, 1, 2, 50_000_000, 6);
        
        let key1 = CanonicalEventKey::from_event(&event1, 100);
        let key2 = CanonicalEventKey::from_event(&event2, 100);
        
        // Lower nonce comes first
        assert!(key1 < key2);
    }
}
