#![forbid(unsafe_code)]

//! Canonical Ordering Constitution
//! 
//! Defines deterministic ordering for all events in a block.
//! Without this, identical block contents can produce different state roots.

use crate::event::Event;
use crate::event::EventType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanonicalOrderKey {
    pub block_height: u64,
    pub sender_id: u64,
    pub nonce: u64,
    pub event_type_priority: u8,
}

impl CanonicalOrderKey {
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
        }
    }
}

pub struct CanonicalOrdering;

impl CanonicalOrdering {
    /// Sort events deterministically for a block
    pub fn sort_events(events: &mut [Event], block_height: u64) {
        events.sort_by(|a, b| {
            let key_a = CanonicalOrderKey::from_event(a, block_height);
            let key_b = CanonicalOrderKey::from_event(b, block_height);
            key_a.cmp(&key_b)
        });
    }
    
    /// Verify that events are in canonical order
    pub fn is_canonically_ordered(events: &[Event], block_height: u64) -> bool {
        for i in 1..events.len() {
            let key_prev = CanonicalOrderKey::from_event(&events[i-1], block_height);
            let key_curr = CanonicalOrderKey::from_event(&events[i], block_height);
            if key_prev > key_curr {
                return false;
            }
        }
        true
    }
    
    /// Get the canonical order specification
    pub fn specification() -> &'static str {
        r#"
        CANONICAL ORDERING CONSTITUTION v1.0
        
        Primary key: block_height (ascending)
        Secondary key: sender_id (ascending)
        Tertiary key: nonce (ascending)
        Quaternary key: event_type_priority (1=Mint, 2=Reward, 3=Transfer, 4=Delegate, 5=Undelegate, 6=Slash, 7=Burn)
        
        All validators MUST order events using this specification.
        Deviation from this order constitutes a consensus violation.
        "#
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::EventType;
    
    #[test]
    fn test_canonical_ordering() {
        let mut events = vec![
            Event::new(EventType::Transfer, 2, 1, 100, 1),
            Event::new(EventType::Mint, 0, 1, 1000, 0),
            Event::new(EventType::Transfer, 1, 2, 50, 0),
        ];
        
        CanonicalOrdering::sort_events(&mut events, 100);
        
        assert_eq!(events[0].event_type, EventType::Mint);
        assert_eq!(events[1].event_type, EventType::Transfer);
        assert_eq!(events[2].event_type, EventType::Transfer);
        
        assert!(CanonicalOrdering::is_canonically_ordered(&events, 100));
    }
}
