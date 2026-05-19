use crate::event::ProtocolEvent;
use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct ProtocolTranscript {
    pub events: Vec<ProtocolEvent>,
    pub chain_hash: [u8; 32],
}

impl ProtocolTranscript {
    pub fn new() -> Self {
        Self { events: Vec::new(), chain_hash: [0u8; 32] }
    }

    pub fn append(&mut self, event: ProtocolEvent) {
        let mut h = Hasher::new();
        h.update(b"AMUN_TRANSCRIPT_V1");
        h.update(&self.chain_hash);
        h.update(&event.hash());
        let mut ch = [0u8; 32];
        ch.copy_from_slice(&h.finalize().as_bytes()[..32]);
        self.chain_hash = ch;
        self.events.push(event);
    }

    pub fn len(&self) -> usize { self.events.len() }

    /// Recomputes the full chain hash and verifies it matches.
    pub fn verify_continuity(&self) -> bool {
        let mut chain = [0u8; 32];
        for event in &self.events {
            let mut h = Hasher::new();
            h.update(b"AMUN_TRANSCRIPT_V1");
            h.update(&chain);
            h.update(&event.hash());
            let mut next = [0u8; 32];
            next.copy_from_slice(&h.finalize().as_bytes()[..32]);
            chain = next;
        }
        chain == self.chain_hash
    }

    /// Build a transcript from a slice of events (for recovery).
    pub fn from_events(events: &[ProtocolEvent]) -> Self {
        let mut transcript = Self::new();
        for event in events {
            transcript.append(event.clone());
        }
        transcript
    }
}
