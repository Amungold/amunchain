use crate::envelope::Envelope;
use std::collections::VecDeque;

/// Abstract transport that stores messages in memory.
/// This is a mock for testing; a real implementation would use
/// TCP, QUIC, or libp2p.
pub struct MockTransport {
    /// Outbound queue: messages waiting to be delivered to their recipient.
    outbox: VecDeque<Envelope>,
    /// Inbound queue: messages received by this node.
    inbox: VecDeque<Envelope>,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            outbox: VecDeque::new(),
            inbox: VecDeque::new(),
        }
    }

    /// Queue an envelope for delivery.
    pub fn send(&mut self, envelope: Envelope) {
        self.outbox.push_back(envelope);
    }

    /// Take the next envelope from the outbox (for the network layer to deliver).
    pub fn next_outgoing(&mut self) -> Option<Envelope> {
        self.outbox.pop_front()
    }

    /// Deliver an envelope into this node's inbox.
    pub fn deliver(&mut self, envelope: Envelope) {
        self.inbox.push_back(envelope);
    }

    /// Take the next envelope from the inbox (for the consensus layer to process).
    pub fn next_incoming(&mut self) -> Option<Envelope> {
        self.inbox.pop_front()
    }

    /// Number of pending outgoing envelopes.
    pub fn outbox_len(&self) -> usize {
        self.outbox.len()
    }

    /// Number of pending incoming envelopes.
    pub fn inbox_len(&self) -> usize {
        self.inbox.len()
    }
}

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

// v0.2: Placeholder for time simulation. Does nothing in baseline mode.
impl MockTransport {
    pub fn tick(&mut self, _elapsed_ms: u64) {
        // In f47ec95 baseline, time does not pass.
    }
}
