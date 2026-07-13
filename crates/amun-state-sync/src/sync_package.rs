use serde::{Deserialize, Serialize};

use crate::snapshot_certificate::StateSnapshotCertificate;
use crate::state_chunk::{ChunkMerkleProof, StateChunk};

/// The complete sync package sent to a bootstrapping node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalSyncPackage {
    pub snapshot_certificate: StateSnapshotCertificate,
    pub chunks: Vec<StateChunk>,
    pub chunk_proofs: Vec<ChunkMerkleProof>,
    pub package_hash: [u8; 32],
}

impl ConstitutionalSyncPackage {
    pub fn new(
        snapshot_certificate: StateSnapshotCertificate,
        chunks: Vec<StateChunk>,
        chunk_proofs: Vec<ChunkMerkleProof>,
    ) -> Self {
        let mut pkg = Self {
            snapshot_certificate,
            chunks,
            chunk_proofs,
            package_hash: [0u8; 32],
        };
        pkg.package_hash = pkg.compute_hash();
        pkg
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_SYNC_PACKAGE_V1");
        hasher.update(&self.snapshot_certificate.certificate_hash);
        hasher.update(&self.snapshot_certificate.chunk_root);
        for chunk in &self.chunks {
            hasher.update(&chunk.chunk_hash);
        }
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn verify(&self) -> bool {
        self.package_hash == self.compute_hash()
    }

    pub fn total_chunks(&self) -> usize {
        self.chunks.len()
    }

    pub fn total_resources(&self) -> usize {
        self.chunks.iter().map(|c| c.resource_count()).sum()
    }
}
