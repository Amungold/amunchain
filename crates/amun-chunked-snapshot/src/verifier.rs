use crate::chunk::SnapshotChunk;
use crate::manifest::ChunkManifest;

/// Verify a single chunk against the manifest and previous chunk hash.
pub fn verify_chunk(
    chunk: &SnapshotChunk,
    manifest: &ChunkManifest,
    prev_hash: [u8; 32],
) -> Result<(), &'static str> {
    if chunk.chunk_index >= manifest.total_chunks {
        return Err("chunk index out of range");
    }

    if !chunk.verify(prev_hash) {
        return Err("chunk hash verification failed");
    }

    if chunk.chunk_hash != manifest.chunk_hashes[chunk.chunk_index as usize] {
        return Err("chunk hash does not match manifest");
    }

    if chunk.total_chunks != manifest.total_chunks {
        return Err("total chunks mismatch");
    }

    Ok(())
}
