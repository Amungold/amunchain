use blake3::Hasher;

pub const CHUNK_SIZE: usize = 4096;

#[derive(Debug, Clone)]
pub struct SnapshotChunk {
    pub chunk_index: u64,
    pub total_chunks: u64,
    pub entries: Vec<([u8; 32], Vec<u8>)>,
    pub chunk_hash: [u8; 32],
    pub previous_chunk_hash: [u8; 32],
}

impl SnapshotChunk {
    pub fn new(
        chunk_index: u64,
        total_chunks: u64,
        entries: Vec<([u8; 32], Vec<u8>)>,
        prev_hash: [u8; 32],
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_CHUNK_V1");
        h.update(&chunk_index.to_le_bytes());
        h.update(&total_chunks.to_le_bytes());
        h.update(&prev_hash);
        for (key, value) in &entries {
            h.update(key);
            h.update(&(value.len() as u32).to_le_bytes());
            h.update(value);
        }
        let mut chunk_hash = [0u8; 32];
        chunk_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self { chunk_index, total_chunks, entries, chunk_hash, previous_chunk_hash: prev_hash }
    }

    pub fn verify(&self, prev_hash: [u8; 32]) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_CHUNK_V1");
        h.update(&self.chunk_index.to_le_bytes());
        h.update(&self.total_chunks.to_le_bytes());
        h.update(&prev_hash);
        for (key, value) in &self.entries {
            h.update(key);
            h.update(&(value.len() as u32).to_le_bytes());
            h.update(value);
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.chunk_hash && self.previous_chunk_hash == prev_hash
    }
}
