#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

/// A certificate authenticating a state snapshot at a specific height.
/// Signed by the constitutional authority and verifiable against the
/// history root chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSnapshotCertificate {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub history_root: [u8; 32],
    pub chunk_root: [u8; 32],
    pub chunk_count: u32,
    pub total_resources: u64,
    pub certificate_hash: [u8; 32],
    pub issued_at: u64,
    pub verifier: String,
}

impl StateSnapshotCertificate {
    pub fn new(
        height: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
        history_root: [u8; 32],
        chunk_root: [u8; 32],
        chunk_count: u32,
        total_resources: u64,
        issued_at: u64,
        verifier: String,
    ) -> Self {
        let mut cert = Self {
            height,
            block_hash,
            state_root,
            history_root,
            chunk_root,
            chunk_count,
            total_resources,
            certificate_hash: [0u8; 32],
            issued_at,
            verifier,
        };
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_SNAPSHOT_CERT_V1");
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.state_root);
        hasher.update(&self.history_root);
        hasher.update(&self.chunk_root);
        hasher.update(&self.chunk_count.to_le_bytes());
        hasher.update(&self.total_resources.to_le_bytes());
        hasher.update(&self.issued_at.to_le_bytes());
        hasher.update(self.verifier.as_bytes());
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn verify(&self) -> bool {
        self.certificate_hash == self.compute_hash()
    }
}
