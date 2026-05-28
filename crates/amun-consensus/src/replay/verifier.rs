//! Replay Verifier - Constitutionally Verifies Execution

use super::certificate::ReplayCertificate;
use crate::constitutional::ExecutionWitness;

pub struct ReplayVerifier;

impl ReplayVerifier {
    pub fn verify_transcript(
        certificate: &ReplayCertificate,
        transcript_hash: [u8; 32],
        state_root: [u8; 32],
        validator_root: [u8; 32],
    ) -> bool {
        certificate.verify(transcript_hash, state_root, validator_root)
    }

    pub fn verify_witness(witness: &ExecutionWitness, certificate: &ReplayCertificate) -> bool {
        witness.verify_chain() && witness.compute_state_root() == certificate.state_root
    }
}
