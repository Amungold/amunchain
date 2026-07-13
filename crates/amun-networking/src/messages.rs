/// All network messages are constitutional artifacts.
/// Every message carries constitutional identity for verification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    /// Request constitutional identity from peer
    IdentityRequest,
    /// Respond with constitutional identity
    IdentityResponse,
    /// Request snapshot manifest
    ManifestRequest,
    /// Respond with snapshot manifest
    ManifestResponse,
    /// Request specific chunk
    ChunkRequest { chunk_index: u64 },
    /// Respond with chunk data
    ChunkResponse {
        chunk_index: u64,
        chunk_hash: [u8; 32],
    },
    /// Request WAL tail after checkpoint
    WalTailRequest { from_sequence: u64 },
    /// Respond with WAL entries
    WalTailResponse { entry_count: u64 },
    /// Request lineage proof
    LineageRequest,
    /// Respond with lineage proof
    LineageResponse,
    /// Sync complete verification
    SyncVerification { final_root: [u8; 32] },
    /// Reject peer due to constitutional incompatibility
    ConstitutionalRejection { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalMessage {
    pub message_type: MessageType,
    pub sender_identity_hash: [u8; 32],
    pub sender_civilization_id: [u8; 32],
    pub payload_hash: [u8; 32],
    pub message_hash: [u8; 32],
}
