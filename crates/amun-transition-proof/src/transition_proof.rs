#![allow(clippy::too_many_arguments)]
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use amun_resource_core::{ResourceId, ResourceMetadata};
use amun_vm_kernel::pending_buffer::OperationRecord;
use serde::{Deserialize, Serialize};

/// A cryptographic proof that a specific contract execution occurred
/// and produced a specific outcome.  Enables independent third-party
/// verification without access to contract internal state.
///
/// Follows N48.5-E Section 8.1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionProof {
    pub transaction_hash: [u8; 32],
    pub contract_id: ResourceId,
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub pre_state_root: [u8; 32],
    pub post_state_root: [u8; 32],
    pub consumed_resources: Vec<ResourceId>,
    pub produced_resources: Vec<ResourceMetadata>,
    pub operation_log: Vec<OperationRecord>,
    pub evidence: Vec<ConstitutionalEvidence>,
    pub gas_used: u64,
    pub proof_hash: [u8; 32],
}

impl TransitionProof {
    /// Construct a new proof and compute its cryptographic hash.
    pub fn new(
        transaction_hash: [u8; 32],
        contract_id: ResourceId,
        block_height: u64,
        block_hash: [u8; 32],
        pre_state_root: [u8; 32],
        post_state_root: [u8; 32],
        consumed_resources: Vec<ResourceId>,
        produced_resources: Vec<ResourceMetadata>,
        operation_log: Vec<OperationRecord>,
        evidence: Vec<ConstitutionalEvidence>,
        gas_used: u64,
    ) -> Self {
        let mut proof = Self {
            transaction_hash,
            contract_id,
            block_height,
            block_hash,
            pre_state_root,
            post_state_root,
            consumed_resources,
            produced_resources,
            operation_log,
            evidence,
            gas_used,
            proof_hash: [0u8; 32],
        };
        proof.proof_hash = proof.compute_hash();
        proof
    }

    /// Compute the cryptographic hash of the proof.
    /// Covers all fields except proof_hash itself.
    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_TRANSITION_PROOF_V1");
        hasher.update(&self.transaction_hash);
        hasher.update(self.contract_id.as_bytes());
        hasher.update(&self.block_height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.pre_state_root);
        hasher.update(&self.post_state_root);
        hasher.update(&self.gas_used.to_le_bytes());
        for id in &self.consumed_resources {
            hasher.update(id.as_bytes());
        }
        for meta in &self.produced_resources {
            hasher.update(meta.resource_id.as_bytes());
            hasher.update(&meta.lineage.version.to_le_bytes());
        }
        for op in &self.operation_log {
            hasher.update(op.opcode.as_bytes());
        }
        for ev in &self.evidence {
            hasher.update(&ev.evidence_id());
        }
        let hash = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(hash.as_bytes());
        bytes
    }

    /// Verify the proof's integrity by recomputing the hash.
    pub fn verify_integrity(&self) -> bool {
        self.proof_hash == self.compute_hash()
    }

    /// Verify that the post-state root matches what would be produced
    /// by applying the consumed/produced resources to the pre-state root.
    /// In a full implementation, this replays the operation log.
    pub fn verify_state_transition(&self) -> bool {
        // For W5: structural verification.
        // Full replay verification comes in W6.
        if self.consumed_resources.is_empty() && self.produced_resources.is_empty() {
            return self.pre_state_root == self.post_state_root;
        }
        // If resources changed, post_state_root must differ from pre_state_root
        if !self.consumed_resources.is_empty() || !self.produced_resources.is_empty() {
            return self.pre_state_root != self.post_state_root;
        }
        true
    }

    /// Number of evidence records in this proof.
    pub fn evidence_count(&self) -> usize {
        self.evidence.len()
    }

    /// Number of operations in this proof.
    pub fn operation_count(&self) -> usize {
        self.operation_log.len()
    }

    /// Returns true if the transaction was rejected (has violation evidence).
    pub fn was_rejected(&self) -> bool {
        self.evidence.iter().any(|e| e.causes_revert())
    }

    /// Returns true if the transaction was committed successfully.
    pub fn was_committed(&self) -> bool {
        !self.was_rejected()
    }
}
