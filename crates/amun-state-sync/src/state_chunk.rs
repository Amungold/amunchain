use amun_resource_core::ResourceMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateChunk {
    pub chunk_id: u32,
    pub resources: Vec<ResourceMetadata>,
    pub chunk_hash: [u8; 32],
}

impl StateChunk {
    pub fn new(chunk_id: u32, resources: Vec<ResourceMetadata>) -> Self {
        let chunk_hash = Self::compute_chunk_hash(chunk_id, &resources);
        Self { chunk_id, resources, chunk_hash }
    }

    pub fn compute_chunk_hash(chunk_id: u32, resources: &[ResourceMetadata]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_STATE_CHUNK_V1");
        hasher.update(&chunk_id.to_le_bytes());
        for meta in resources {
            hasher.update(meta.resource_id.as_bytes());
            hasher.update(&[meta.archetype as u8]);
            hasher.update(&meta.lineage.version.to_le_bytes());
            hasher.update(&meta.contract_id);
            hasher.update(&meta.owner);
        }
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn verify(&self) -> bool {
        self.chunk_hash == Self::compute_chunk_hash(self.chunk_id, &self.resources)
    }

    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkMerkleProof {
    pub chunk_id: u32,
    pub chunk_hash: [u8; 32],
    pub chunk_root: [u8; 32],
    pub siblings: Vec<([u8; 32], bool)>,
}

impl ChunkMerkleProof {
    pub fn verify(&self) -> bool {
        let mut current = self.chunk_hash;
        for (sibling, sibling_is_left) in &self.siblings {
            let mut hasher = blake3::Hasher::new();
            hasher.update(b"AMUN_CHUNK_MERKLE_V1");
            if *sibling_is_left {
                hasher.update(sibling);
                hasher.update(&current);
            } else {
                hasher.update(&current);
                hasher.update(sibling);
            }
            let hash = hasher.finalize();
            current.copy_from_slice(hash.as_bytes());
        }
        current == self.chunk_root
    }
}

pub fn build_chunk_merkle_tree(chunks: &[StateChunk]) -> ([u8; 32], Vec<ChunkMerkleProof>) {
    if chunks.is_empty() {
        return ([0u8; 32], vec![]);
    }
    let leaf_hashes: Vec<[u8; 32]> = chunks.iter().map(|c| c.chunk_hash).collect();
    let chunk_root = compute_merkle_root(&leaf_hashes);
    let mut proofs = Vec::new();
    for (i, chunk) in chunks.iter().enumerate() {
        let siblings = compute_merkle_siblings(&leaf_hashes, i);
        proofs.push(ChunkMerkleProof {
            chunk_id: chunk.chunk_id,
            chunk_hash: chunk.chunk_hash,
            chunk_root,
            siblings,
        });
    }
    (chunk_root, proofs)
}

fn compute_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return [0u8; 32];
    }
    let mut level: Vec<[u8; 32]> = leaves.to_vec();
    while level.len() > 1 {
        let mut next = Vec::new();
        for pair in level.chunks(2) {
            let mut hasher = blake3::Hasher::new();
            hasher.update(b"AMUN_CHUNK_MERKLE_V1");
            hasher.update(&pair[0]);
            if pair.len() == 2 {
                hasher.update(&pair[1]);
            } else {
                hasher.update(&pair[0]);
            }
            let hash = hasher.finalize();
            let mut h = [0u8; 32];
            h.copy_from_slice(hash.as_bytes());
            next.push(h);
        }
        level = next;
    }
    level[0]
}

fn compute_merkle_siblings(leaves: &[[u8; 32]], target_idx: usize) -> Vec<([u8; 32], bool)> {
    let mut siblings = Vec::new();
    let mut idx = target_idx;
    let mut level: Vec<[u8; 32]> = leaves.to_vec();
    while level.len() > 1 {
        let current_is_left = idx.is_multiple_of(2);
        #[allow(clippy::manual_is_multiple_of)]
        let sibling_idx = if current_is_left { idx + 1 } else { idx - 1 };
        if sibling_idx < level.len() {
            let sibling_is_left = !current_is_left;
            siblings.push((level[sibling_idx], sibling_is_left));
        } else {
            // Odd leaf at this level: pair with itself (hash(X || X))
            siblings.push((level[idx], false));
        }
        let mut next = Vec::new();
        for pair in level.chunks(2) {
            let mut hasher = blake3::Hasher::new();
            hasher.update(b"AMUN_CHUNK_MERKLE_V1");
            hasher.update(&pair[0]);
            if pair.len() == 2 {
                hasher.update(&pair[1]);
            } else {
                hasher.update(&pair[0]);
            }
            let hash = hasher.finalize();
            let mut h = [0u8; 32];
            h.copy_from_slice(hash.as_bytes());
            next.push(h);
        }
        idx /= 2;
        level = next;
    }
    siblings
}
