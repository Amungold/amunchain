// Chunking System - Constitutional chunk boundaries at 16MB
// Each chunk is individually hashed and committed in a Merkle tree.

use super::snapshot::{SerializedNode, MAX_CHUNK_SIZE};
use amun_canonical_codec::CanonicalHasher;
use amun_canonical_codec::PROTOCOL_DOMAIN_CHUNK;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotChunk {
    pub index: u64,
    pub node_count: u64,
    pub total_size: u64,
    pub chunk_hash: [u8; 32],
    pub nodes: Vec<SerializedNode>,
}

impl SnapshotChunk {
    pub fn new(index: u64, nodes: Vec<SerializedNode>) -> Self {
        let node_count = nodes.len() as u64;
        let total_size = nodes
            .iter()
            .map(|n| n.data.len() as u64 + 8 + 1 + 32 + 8)
            .sum();
        let chunk_hash = Self::compute_hash(index, &nodes);
        Self {
            index,
            node_count,
            total_size,
            chunk_hash,
            nodes,
        }
    }

    fn compute_hash(index: u64, nodes: &[SerializedNode]) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(PROTOCOL_DOMAIN_CHUNK);
        h.update_u64(index);
        for node in nodes {
            let encoded = node.encode();
            h.update_u64(encoded.len() as u64);
            h.update(&encoded);
        }
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        Self::compute_hash(self.index, &self.nodes) == self.chunk_hash
    }
}

// ============================================================
// Chunk Builder - Splits nodes into 16MB chunks
// ============================================================
pub struct ChunkBuilder {
    chunks: Vec<SnapshotChunk>,
    current_nodes: Vec<SerializedNode>,
    current_size: u64,
    next_index: u64,
}

impl ChunkBuilder {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            current_nodes: Vec::new(),
            current_size: 0,
            next_index: 0,
        }
    }

    pub fn add_node(&mut self, node: SerializedNode) {
        let node_size = node.data.len() as u64 + 8 + 1 + 32 + 8;
        if self.current_size + node_size > MAX_CHUNK_SIZE && !self.current_nodes.is_empty() {
            self.finish_chunk();
        }
        self.current_size += node_size;
        self.current_nodes.push(node);
    }

    fn finish_chunk(&mut self) {
        let nodes = std::mem::take(&mut self.current_nodes);
        let chunk = SnapshotChunk::new(self.next_index, nodes);
        self.chunks.push(chunk);
        self.next_index += 1;
        self.current_size = 0;
    }

    pub fn build(mut self) -> ChunkIndex {
        if !self.current_nodes.is_empty() {
            self.finish_chunk();
        }
        let chunk_count = self.chunks.len() as u64;
        let total_nodes: u64 = self.chunks.iter().map(|c| c.node_count).sum();
        let total_size: u64 = self.chunks.iter().map(|c| c.total_size).sum();
        let chunk_root = ChunkIndex::compute_merkle_root(&self.chunks);
        ChunkIndex {
            chunks: self.chunks,
            chunk_count,
            total_nodes,
            total_size,
            chunk_root,
        }
    }
}

// ============================================================
// Chunk Index - Merkle tree over chunk hashes
// ============================================================
#[derive(Debug, Clone)]
pub struct ChunkIndex {
    pub chunks: Vec<SnapshotChunk>,
    pub chunk_count: u64,
    pub total_nodes: u64,
    pub total_size: u64,
    pub chunk_root: [u8; 32],
}

impl ChunkIndex {
    fn compute_merkle_root(chunks: &[SnapshotChunk]) -> [u8; 32] {
        if chunks.is_empty() {
            return [0u8; 32];
        }
        let mut hashes: Vec<[u8; 32]> = chunks.iter().map(|c| c.chunk_hash).collect();
        while hashes.len() > 1 {
            let mut next = Vec::with_capacity((hashes.len() + 1) / 2);
            for pair in hashes.chunks(2) {
                let left = pair[0];
                let right = if pair.len() > 1 { pair[1] } else { pair[0] };
                let mut h = CanonicalHasher::with_domain(b"AMUN_CHUNK_MERKLE_V1");
                h.update(&left);
                h.update(&right);
                next.push(h.finalize());
            }
            hashes = next;
        }
        hashes[0]
    }

    pub fn get_chunk(&self, index: u64) -> Option<&SnapshotChunk> {
        self.chunks.iter().find(|c| c.index == index)
    }

    pub fn verify(&self) -> bool {
        for chunk in &self.chunks {
            if !chunk.verify() {
                return false;
            }
        }
        Self::compute_merkle_root(&self.chunks) == self.chunk_root
    }
}
