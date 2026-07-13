use amun_canonical_codec::{CanonicalReader, CanonicalWriter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncMessage {
    /// Request snapshot manifest from peer
    RequestManifest { request_id: u64 },
    /// Send snapshot manifest to peer
    ManifestResponse {
        request_id: u64,
        manifest_data: Vec<u8>,
    },
    /// Request specific chunk by index
    RequestChunk { request_id: u64, chunk_index: u64 },
    /// Send chunk data to peer
    ChunkResponse {
        request_id: u64,
        chunk_index: u64,
        chunk_data: Vec<u8>,
    },
    /// Request WAL tail after snapshot checkpoint
    RequestWalTail { request_id: u64, from_sequence: u64 },
    /// Send WAL tail entries
    WalTailResponse {
        request_id: u64,
        entries: Vec<Vec<u8>>,
    },
    /// Verification complete - state is valid
    SyncComplete {
        final_root: [u8; 32],
        total_chunks: u64,
        total_frames: u64,
    },
    /// Sync failed with reason
    SyncFailed { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncRequest {
    pub request_id: u64,
    pub message_type: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncResponse {
    pub request_id: u64,
    pub message_type: u8,
    pub payload: Vec<u8>,
}

impl SyncRequest {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = CanonicalWriter::new();
        w.write_u64(self.request_id);
        w.write_u8(self.message_type);
        w.write_bytes(&self.payload);
        w.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut r = CanonicalReader::new(data);
        let request_id = r.read_u64()?;
        let message_type = r.read_u8()?;
        let payload = r.read_bytes()?;
        Some(Self {
            request_id,
            message_type,
            payload,
        })
    }
}

impl SyncResponse {
    pub fn encode(&self) -> Vec<u8> {
        let mut w = CanonicalWriter::new();
        w.write_u64(self.request_id);
        w.write_u8(self.message_type);
        w.write_bytes(&self.payload);
        w.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut r = CanonicalReader::new(data);
        let request_id = r.read_u64()?;
        let message_type = r.read_u8()?;
        let payload = r.read_bytes()?;
        Some(Self {
            request_id,
            message_type,
            payload,
        })
    }
}
