use serde::{Deserialize, Serialize};

/// A message that can be sent between peers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub payload: Vec<u8>,
}
