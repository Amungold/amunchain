use crate::message::GossipMessage;

/// A witness relay re-publishes valid gossip messages to other peers.
/// It does not interpret or execute the messages — it only forwards
/// messages that have been verified.
pub struct WitnessRelay;

impl WitnessRelay {
    /// Forward a message to a list of peers (represented here as a
    /// closure for simplicity).  In production this would be a
    /// network transport.
    pub fn relay<F>(msg: &GossipMessage, mut send: F)
    where
        F: FnMut(&GossipMessage),
    {
        send(msg);
    }
}
