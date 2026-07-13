use amun_chain_position::ChainPosition;
use blake3::Hasher;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayCertificate {
    pub genesis_root: [u8; 32],
    pub final_root: [u8; 32],
    pub start_position: ChainPosition,
    pub end_position: ChainPosition,
    pub event_count: u64,
    pub transaction_count: u64,
    pub seal_count: u64,
    pub transcript_chain_hash: [u8; 32],
    pub certificate_hash: [u8; 32],
    pub execution_version: u64,
}

impl ReplayCertificate {
    pub fn new(
        genesis_root: [u8; 32],
        final_root: [u8; 32],
        start_position: ChainPosition,
        end_position: ChainPosition,
        event_count: u64,
        transaction_count: u64,
        seal_count: u64,
        transcript_chain_hash: [u8; 32],
        execution_version: u64,
    ) -> Self {
        let mut h = Hasher::new();
        h.update(b"AMUN_REPLAY_CERT_V2");
        h.update(&genesis_root);
        h.update(&final_root);
        h.update(&start_position.hash());
        h.update(&end_position.hash());
        h.update(&event_count.to_le_bytes());
        h.update(&transaction_count.to_le_bytes());
        h.update(&seal_count.to_le_bytes());
        h.update(&transcript_chain_hash);
        h.update(&execution_version.to_le_bytes());
        let mut certificate_hash = [0u8; 32];
        certificate_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);

        Self {
            genesis_root,
            final_root,
            start_position,
            end_position,
            event_count,
            transaction_count,
            seal_count,
            transcript_chain_hash,
            certificate_hash,
            execution_version,
        }
    }

    pub fn verify(&self) -> bool {
        let mut h = Hasher::new();
        h.update(b"AMUN_REPLAY_CERT_V2");
        h.update(&self.genesis_root);
        h.update(&self.final_root);
        h.update(&self.start_position.hash());
        h.update(&self.end_position.hash());
        h.update(&self.event_count.to_le_bytes());
        h.update(&self.transaction_count.to_le_bytes());
        h.update(&self.seal_count.to_le_bytes());
        h.update(&self.transcript_chain_hash);
        h.update(&self.execution_version.to_le_bytes());
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
        computed == self.certificate_hash
    }
}
