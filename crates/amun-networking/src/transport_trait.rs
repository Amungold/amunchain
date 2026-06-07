use crate::envelope::Envelope;

/// Abstract transport layer for network communication.
///
/// Implementations:
/// - MockTransport: in-memory testing (existing)
/// - TcpTransport: real TCP network (new)
pub trait Transport: Send {
    /// Queue an envelope for sending.
    fn send(&mut self, envelope: Envelope);

    /// Take the next envelope from the outbox for delivery.
    fn next_outgoing(&mut self) -> Option<Envelope>;

    /// Receive an envelope from the network.
    fn deliver(&mut self, envelope: Envelope);

    /// Take the next envelope from the inbox for processing.
    fn next_incoming(&mut self) -> Option<Envelope>;

    /// Advance transport state. For mock: advances time. For TCP: reads/writes sockets.
    fn tick(&mut self, elapsed_ms: u64);
}
