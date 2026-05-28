//! # Constitutional Execution Receipts
//!
//! This crate defines the **immutable evidence** produced by every
//! state transition. It is the bridge between execution and truth.
//!
//! ## Constitutional Invariants
//! - Every receipt is immutable after creation.
//! - If `state_changed` is true, `pre_root != post_root`.
//! - If `state_changed` is false, `pre_root == post_root`.
//! - Receipts are hash‑linked into an append‑only chain.
//! - Transcripts verify both individual receipts and chain continuity.

use amun_kernel::canonical::{CanonicalEncode, CanonicalEncoder};
use amun_kernel::hashing::domain_tags;

/// The outcome of executing a transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStatus {
    /// Execution succeeded and state was mutated.
    Success,
    /// Execution reverted but fees were deducted.
    Reverted,
    /// Transaction was rejected before execution.
    Rejected,
}

impl CanonicalEncode for ExecutionStatus {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        let tag: u8 = match self {
            ExecutionStatus::Success => 0,
            ExecutionStatus::Reverted => 1,
            ExecutionStatus::Rejected => 2,
        };
        tag.encode_canonical(out);
    }
}

/// An immutable receipt produced after a single state transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReceipt {
    pub tx_hash: [u8; 32],
    pub pre_state_root: [u8; 32],
    pub post_state_root: [u8; 32],
    pub execution_result_hash: [u8; 32],
    pub status: ExecutionStatus,
    pub state_changed: bool,
    /// Links to the previous receipt's hash, forming an append‑only chain.
    pub previous_receipt_hash: [u8; 32],
}

impl CanonicalEncode for ExecutionReceipt {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.tx_hash);
        out.extend_from_slice(&self.pre_state_root);
        out.extend_from_slice(&self.post_state_root);
        out.extend_from_slice(&self.execution_result_hash);
        self.status.encode_canonical(out);
        (self.state_changed as u8).encode_canonical(out);
        out.extend_from_slice(&self.previous_receipt_hash);
    }
}

impl ExecutionReceipt {
    /// Create a new receipt. This is the only way to produce one.
    pub fn new(
        tx_hash: [u8; 32],
        pre_state_root: [u8; 32],
        post_state_root: [u8; 32],
        execution_result_hash: [u8; 32],
        status: ExecutionStatus,
        state_changed: bool,
        previous_receipt_hash: [u8; 32],
    ) -> Self {
        Self {
            tx_hash,
            pre_state_root,
            post_state_root,
            execution_result_hash,
            status,
            state_changed,
            previous_receipt_hash,
        }
    }

    /// Compute the canonical hash of this receipt.
    pub fn receipt_hash(&self) -> [u8; 32] {
        CanonicalEncoder::hash_value(self, domain_tags::EXECUTION_RECEIPT)
    }

    /// Verify a critical constitutional invariant:
    /// - If roots are equal, `state_changed` must be false.
    /// - If roots differ, `state_changed` must be true.
    pub fn verify_consistency(&self) -> Result<(), &'static str> {
        if self.pre_state_root == self.post_state_root && self.state_changed {
            return Err("Constitutional anomaly: state unchanged but execution claims mutation");
        }
        if self.pre_state_root != self.post_state_root && !self.state_changed {
            return Err("Constitutional anomaly: state mutated but execution claims no mutation");
        }
        Ok(())
    }
}

/// A sequence of execution receipts, forming a deterministic transcript.
#[derive(Debug, Clone)]
pub struct ExecutionTranscript {
    pub receipts: Vec<ExecutionReceipt>,
}

impl CanonicalEncode for ExecutionTranscript {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        (self.receipts.len() as u64).encode_canonical(out);
        for receipt in &self.receipts {
            receipt.encode_canonical(out);
        }
    }
}

impl Default for ExecutionTranscript {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionTranscript {
    pub fn new() -> Self {
        Self {
            receipts: Vec::new(),
        }
    }

    /// Append a receipt to the transcript.
    pub fn add_receipt(&mut self, receipt: ExecutionReceipt) {
        self.receipts.push(receipt);
    }

    /// Compute the canonical hash of the entire transcript.
    pub fn transcript_hash(&self) -> [u8; 32] {
        CanonicalEncoder::hash_value(self, domain_tags::EXECUTION_TRANSCRIPT)
    }

    /// Verify the transcript against the block's declared initial root.
    pub fn verify_against_initial_root(
        &self,
        declared_pre_root: &[u8; 32],
    ) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(first) = self.receipts.first() {
            if &first.pre_state_root != declared_pre_root {
                errors.push("First receipt pre_root does not match block pre_root".to_string());
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Verify the entire transcript for constitutional consistency.
    /// Checks each receipt individually AND checks chain continuity.
    pub fn verify_transcript(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Per-receipt checks
        for receipt in &self.receipts {
            if let Err(e) = receipt.verify_consistency() {
                errors.push(format!("Receipt {}: {}", hex::encode(receipt.tx_hash), e));
            }
        }

        // Chain continuity check
        for w in self.receipts.windows(2) {
            if w[0].post_state_root != w[1].pre_state_root {
                errors.push(format!(
                    "Transcript continuity broken: receipt {} post_root != receipt {} pre_root",
                    hex::encode(w[0].tx_hash),
                    hex::encode(w[1].tx_hash),
                ));
            }
        }

        // Receipt chain linking check (previous_receipt_hash)
        for i in 1..self.receipts.len() {
            let prev_hash = self.receipts[i - 1].receipt_hash();
            if self.receipts[i].previous_receipt_hash != prev_hash {
                errors.push(format!(
                    "Receipt chain broken at index {}: expected previous_hash {}",
                    i,
                    hex::encode(prev_hash)
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
