#![forbid(unsafe_code)]

use sha2::{Sha256, Digest};
use crate::state::ConstitutionalState;
use crate::receipt::ExecutionReceipts;

#[derive(Debug, Clone)]
pub struct ExecutionCertificate {
    pub block_height: u64,
    pub pre_state_hash: [u8; 32],
    pub post_state_hash: [u8; 32],
    pub receipts_accumulator: [u8; 32],
    pub trace_root: [u8; 32],
    pub validator_signature: Option<[u8; 64]>,
}

impl ExecutionCertificate {
    pub fn new(
        block_height: u64,
        pre_state: &ConstitutionalState,
        post_state: &ConstitutionalState,
        receipts: &ExecutionReceipts,
        trace_root: [u8; 32],
    ) -> Self {
        Self {
            block_height,
            pre_state_hash: pre_state.hash(),
            post_state_hash: post_state.hash(),
            receipts_accumulator: receipts.accumulator_hash,
            trace_root,
            validator_signature: None,
        }
    }
    
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + 32 + 32 + 32 + 32 + 64);
        bytes.extend_from_slice(&self.block_height.to_be_bytes());
        bytes.extend_from_slice(&self.pre_state_hash);
        bytes.extend_from_slice(&self.post_state_hash);
        bytes.extend_from_slice(&self.receipts_accumulator);
        bytes.extend_from_slice(&self.trace_root);
        bytes
    }
    
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.canonical_bytes());
        hasher.finalize().into()
    }
    
    pub fn sign(&mut self, signature: [u8; 64]) {
        self.validator_signature = Some(signature);
    }
    
    pub fn verify(&self) -> bool {
        self.validator_signature.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct ReplayCertificate {
    pub block_height: u64,
    pub transcript_hash: [u8; 32],
    pub final_state_hash: [u8; 32],
    pub receipts_accumulator: [u8; 32],
    pub execution_count: u64,
    pub execution_fingerprint: [u8; 32],
}

impl ReplayCertificate {
    pub fn new(
        block_height: u64,
        transcript_hash: [u8; 32],
        final_state: &ConstitutionalState,
        receipts: &ExecutionReceipts,
        execution_count: u64,
        execution_fingerprint: [u8; 32],
    ) -> Self {
        Self {
            block_height,
            transcript_hash,
            final_state_hash: final_state.hash(),
            receipts_accumulator: receipts.accumulator_hash,
            execution_count,
            execution_fingerprint,
        }
    }
    
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + 32 + 32 + 32 + 8 + 32);
        bytes.extend_from_slice(&self.block_height.to_be_bytes());
        bytes.extend_from_slice(&self.transcript_hash);
        bytes.extend_from_slice(&self.final_state_hash);
        bytes.extend_from_slice(&self.receipts_accumulator);
        bytes.extend_from_slice(&self.execution_count.to_be_bytes());
        bytes.extend_from_slice(&self.execution_fingerprint);
        bytes
    }
    
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.canonical_bytes());
        hasher.finalize().into()
    }
}

pub fn compute_execution_fingerprint() -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(env!("CARGO_PKG_VERSION").as_bytes());
    hasher.update(&crate::TRANSITION_VERSION.to_be_bytes());
    #[cfg(target_arch = "x86_64")]
    hasher.update(b"x86_64");
    #[cfg(target_arch = "aarch64")]
    hasher.update(b"aarch64");
    hasher.finalize().into()
}
