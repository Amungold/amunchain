use crate::chunk::SnapshotChunk;
use amun_snapshot_constitution::CanonicalSnapshot;
use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct ChunkManifest {
    pub snapshot_hash: [u8; 32],
    pub total_chunks: u64,
    pub total_entries: u64,
    pub chunk_hashes: Vec<[u8; 32]>,
    pub manifest_hash: [u8; 32],
}

impl ChunkManifest {
    pub fn new(snapshot: &CanonicalSnapshot, chunks: &[SnapshotChunk]) -> Self {
        let total = chunks.len() as u64;
        let chunk_hashes: Vec<[u8; 32]> = chunks.iter().map(|c| c.chunk_hash).collect();

        let mut h = Hasher::new();
        h.update(b"AMUN_MANIFEST_V1");
        h.update(&snapshot.snapshot_hash);
        h.update(&total.to_le_bytes());
        h.update(&snapshot.entry_count.to_le_bytes());
        for ch in &chunk_hashes {
            h.update(ch);
        }
        let mut manifest_hash = [0u8; 32];
        manifest_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self {
            snapshot_hash: snapshot.snapshot_hash,
            total_chunks: total,
            total_entries: snapshot.entry_count,
            chunk_hashes,
            manifest_hash,
        }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_MANIFEST_V1");
        h.update(&self.snapshot_hash);
        h.update(&self.total_chunks.to_le_bytes());
        h.update(&self.total_entries.to_le_bytes());
        for ch in &self.chunk_hashes {
            h.update(ch);
        }
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.manifest_hash
    }
}
