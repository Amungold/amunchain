//! Replay Certificate - Constitutional Proof of Execution

use crate::canonical::{CanonicalEncoder, CanonicalSerialize};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayCertificate {
    pub transcript_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub validator_root: [u8; 32],
    pub protocol_version: u32,
    pub serialization_version: u32,
    pub replay_rounds: u64,
}

impl ReplayCertificate {
    pub fn new(
        transcript_hash: [u8; 32],
        state_root: [u8; 32],
        validator_root: [u8; 32],
        protocol_version: u32,
        serialization_version: u32,
        replay_rounds: u64,
    ) -> Self {
        Self {
            transcript_hash,
            state_root,
            validator_root,
            protocol_version,
            serialization_version,
            replay_rounds,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut encoder = CanonicalEncoder::new();
        encoder.write_bytes(b"AMUN_REPLAY_CERT_V1");
        encoder.write_fixed_hash(&self.transcript_hash);
        encoder.write_fixed_hash(&self.state_root);
        encoder.write_fixed_hash(&self.validator_root);
        encoder.write_u32(self.protocol_version);
        encoder.write_u32(self.serialization_version);
        encoder.write_u64(self.replay_rounds);
        blake3::hash(&encoder.into_bytes()).into()
    }

    pub fn verify(&self, transcript_hash: [u8; 32], state_root: [u8; 32], validator_root: [u8; 32]) -> bool {
        transcript_hash == self.transcript_hash && state_root == self.state_root && validator_root == self.validator_root
    }
}

impl CanonicalSerialize for ReplayCertificate {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_fixed_hash(&self.transcript_hash);
        encoder.write_fixed_hash(&self.state_root);
        encoder.write_fixed_hash(&self.validator_root);
        encoder.write_u32(self.protocol_version);
        encoder.write_u32(self.serialization_version);
        encoder.write_u64(self.replay_rounds);
    }
}
