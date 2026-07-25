use serde::{Deserialize, Serialize};

/// Message type constant for transaction propagation
pub const MESSAGE_TYPE_TRANSACTION: &str = "transaction";

/// A transaction message for peer-to-peer propagation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionMessage {
    /// The serialized transaction (using postcard)
    pub transaction_bytes: Vec<u8>,
    /// Hash of the transaction for deduplication
    pub tx_hash: [u8; 32],
    /// Sender's validator ID to prevent echo back
    pub sender_id: [u8; 32],
}
