//! Constitutional State Transition - Deterministic Execution Proofs

use crate::canonical::{CanonicalEncoder, CanonicalDecoder, CanonicalSerialize, CanonicalDeserialize};
use crate::constitutional::ConstitutionalHashable;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

const MAX_TRANSITIONS_PER_WITNESS: usize = 10_000;

#[derive(Debug, Clone)]
pub struct VerifiedTransitionWitness {
    pub pre_state_hash: [u8; 32],
    pub post_state_hash: [u8; 32],
    pub transition_hash: [u8; 32],
    pub input_hash: [u8; 32],
    pub output_hash: [u8; 32],
    pub gas_used: u64,
}

impl VerifiedTransitionWitness {
    pub fn new(
        pre_state_hash: [u8; 32],
        post_state_hash: [u8; 32],
        transition_hash: [u8; 32],
        input_hash: [u8; 32],
        output_hash: [u8; 32],
        gas_used: u64,
    ) -> Self {
        Self { pre_state_hash, post_state_hash, transition_hash, input_hash, output_hash, gas_used }
    }

    pub fn verify_continuity(&self, expected_pre_hash: [u8; 32]) -> bool {
        self.pre_state_hash == expected_pre_hash
    }
}

impl CanonicalSerialize for VerifiedTransitionWitness {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_fixed_hash(&self.pre_state_hash);
        encoder.write_fixed_hash(&self.post_state_hash);
        encoder.write_fixed_hash(&self.transition_hash);
        encoder.write_fixed_hash(&self.input_hash);
        encoder.write_fixed_hash(&self.output_hash);
        encoder.write_u64(self.gas_used);
    }
}

impl CanonicalDeserialize for VerifiedTransitionWitness {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        Some(VerifiedTransitionWitness::new(
            decoder.read_fixed_hash()?,
            decoder.read_fixed_hash()?,
            decoder.read_fixed_hash()?,
            decoder.read_fixed_hash()?,
            decoder.read_fixed_hash()?,
            decoder.read_u64()?,
        ))
    }
}

impl ConstitutionalHashable for VerifiedTransitionWitness {
    const DOMAIN_TAG: &'static [u8] = b"AMUN_TRANSITION_WITNESS_V1";
}

#[derive(Debug, Clone)]
pub struct ExecutionWitness {
    pub transitions: Vec<VerifiedTransitionWitness>,
    pub start_state_hash: [u8; 32],
    pub end_state_hash: [u8; 32],
    pub total_gas: u64,
}

impl ExecutionWitness {
    pub fn new(transitions: Vec<VerifiedTransitionWitness>, start_state_hash: [u8; 32], end_state_hash: [u8; 32], total_gas: u64) -> Option<Self> {
        if transitions.len() > MAX_TRANSITIONS_PER_WITNESS { return None; }
        Some(Self { transitions, start_state_hash, end_state_hash, total_gas })
    }

    pub fn verify_chain(&self) -> bool {
        if self.transitions.is_empty() { return self.start_state_hash == self.end_state_hash; }
        if !self.transitions[0].verify_continuity(self.start_state_hash) { return false; }
        for i in 0..self.transitions.len() - 1 {
            if self.transitions[i].post_state_hash != self.transitions[i + 1].pre_state_hash { return false; }
        }
        self.transitions.last().map_or(false, |last| last.post_state_hash == self.end_state_hash)
    }

    pub fn compute_state_root(&self) -> [u8; 32] { self.end_state_hash }
}

impl CanonicalSerialize for ExecutionWitness {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_fixed_hash(&self.start_state_hash);
        encoder.write_fixed_hash(&self.end_state_hash);
        encoder.write_u64(self.total_gas);
        encoder.write_u64(self.transitions.len() as u64);
        for witness in &self.transitions { witness.encode(encoder); }
    }
}

impl CanonicalDeserialize for ExecutionWitness {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        let start_state_hash = decoder.read_fixed_hash()?;
        let end_state_hash = decoder.read_fixed_hash()?;
        let total_gas = decoder.read_u64()?;
        let len = decoder.read_u64()? as usize;
        if len > MAX_TRANSITIONS_PER_WITNESS { return None; }
        let mut transitions = Vec::with_capacity(len);
        for _ in 0..len { transitions.push(VerifiedTransitionWitness::decode(decoder)?); }
        Some(ExecutionWitness { transitions, start_state_hash, end_state_hash, total_gas })
    }
}

impl ConstitutionalHashable for ExecutionWitness {
    const DOMAIN_TAG: &'static [u8] = b"AMUN_EXECUTION_WITNESS_V1";
}

pub struct WitnessChainVerifier;

impl WitnessChainVerifier {
    pub fn verify(witness: &ExecutionWitness, expected_end_state: [u8; 32]) -> bool {
        witness.verify_chain() && witness.end_state_hash == expected_end_state
    }
}
