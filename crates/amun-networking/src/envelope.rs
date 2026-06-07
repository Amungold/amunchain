use serde::{Deserialize, Serialize};

/// A network envelope that wraps any consensus message with routing metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    /// Sender's peer id.
    pub sender: String,
    /// Intended recipient (empty string = broadcast).
    pub recipient: String,
    /// Monotonically increasing sequence number for ordering.
    pub sequence: u64,
    /// Timestamp when the envelope was created (wall-clock).
    pub timestamp: u64,
    /// Type identifier so the receiver knows how to decode the payload.
    pub message_type: String,
    /// Serialized message body.
    pub payload: Vec<u8>,
}
