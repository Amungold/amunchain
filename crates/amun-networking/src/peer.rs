use serde::{Deserialize, Serialize};

/// Represents a known peer in the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub address: String,
}

impl Peer {
    pub fn new(id: String, address: String) -> Self {
        Self { id, address }
    }
}
