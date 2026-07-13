use super::chunk::SnapshotChunk;
use amun_canonical_codec::{CanonicalHasher, CanonicalReader, CanonicalWriter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkProof {
    pub chunk_index: u64,
    pub chunk_hash: [u8; 32],
    pub chunk_root: [u8; 32],
    pub proof_steps: Vec<ProofStep>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofStep {
    Left([u8; 32]),
    Right([u8; 32]),
}

impl ChunkProof {
    /// Generate a Merkle proof that a chunk is included in the chunk index.
    /// Uses positional tracking: for each pair at each level, we know whether
    /// our target is the left or right element.
    pub fn generate(
        chunk: &SnapshotChunk,
        all_chunks: &[SnapshotChunk],
        chunk_root: [u8; 32],
    ) -> Self {
        let mut hashes: Vec<[u8; 32]> = all_chunks.iter().map(|c| c.chunk_hash).collect();
        let mut target_idx = chunk.index as usize;
        let mut steps = Vec::new();

        while hashes.len() > 1 {
            let mut next_level = Vec::with_capacity((hashes.len() + 1) / 2);

            for pair_idx in 0..((hashes.len() + 1) / 2) {
                let left_idx = pair_idx * 2;
                let right_idx = left_idx + 1;

                let left = hashes[left_idx];
                let right = if right_idx < hashes.len() {
                    hashes[right_idx]
                } else {
                    hashes[left_idx]
                };

                // Determine if our target is in this pair
                if target_idx == left_idx {
                    if right_idx < hashes.len() {
                        steps.push(ProofStep::Right(right));
                    }
                } else if target_idx == right_idx {
                    steps.push(ProofStep::Left(left));
                }
                // else: target not in this pair, no step needed for this pair

                let mut h = CanonicalHasher::with_domain(b"AMUN_CHUNK_MERKLE_V1");
                h.update(&left);
                h.update(&right);
                next_level.push(h.finalize());
            }

            target_idx /= 2;
            hashes = next_level;
        }

        Self {
            chunk_index: chunk.index,
            chunk_hash: chunk.chunk_hash,
            chunk_root,
            proof_steps: steps,
        }
    }

    /// Verify the proof without needing all chunks.
    /// Rebuilds the Merkle root from the chunk hash and sibling steps.
    pub fn verify(&self) -> bool {
        let mut current = self.chunk_hash;
        for step in &self.proof_steps {
            let mut h = CanonicalHasher::with_domain(b"AMUN_CHUNK_MERKLE_V1");
            match step {
                ProofStep::Left(sibling) => {
                    h.update(sibling);
                    h.update(&current);
                }
                ProofStep::Right(sibling) => {
                    h.update(&current);
                    h.update(sibling);
                }
            }
            current = h.finalize();
        }
        current == self.chunk_root
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut w = CanonicalWriter::new();
        w.write_u64(self.chunk_index);
        w.write_hash(&self.chunk_hash);
        w.write_hash(&self.chunk_root);
        w.write_u64(self.proof_steps.len() as u64);
        for step in &self.proof_steps {
            match step {
                ProofStep::Left(h) => {
                    w.write_u8(0x00);
                    w.write_hash(h);
                }
                ProofStep::Right(h) => {
                    w.write_u8(0x01);
                    w.write_hash(h);
                }
            }
        }
        w.into_bytes()
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut r = CanonicalReader::new(data);
        let chunk_index = r.read_u64()?;
        let chunk_hash = r.read_hash()?;
        let chunk_root = r.read_hash()?;
        let step_count = r.read_u64()? as usize;
        let mut proof_steps = Vec::with_capacity(step_count);
        for _ in 0..step_count {
            let st = r.read_u8()?;
            let h = r.read_hash()?;
            proof_steps.push(if st == 0x00 {
                ProofStep::Left(h)
            } else {
                ProofStep::Right(h)
            });
        }
        if !r.is_finished() {
            return None;
        }
        Some(Self {
            chunk_index,
            chunk_hash,
            chunk_root,
            proof_steps,
        })
    }
}
