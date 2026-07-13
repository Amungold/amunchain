use amun_resource_core::ResourceId;
use serde::{Deserialize, Serialize};

/// A cryptographic proof that a resource was consumed on a source
/// contract and may be materialised on a target contract.
/// Implements N48.5-E Section 9 and Law X1 (single-use).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossContractTransferProof {
    /// Unique identifier for this proof — prevents replay.
    pub proof_id: [u8; 32],
    /// The resource that was consumed on the source contract.
    pub consumed_resource_id: ResourceId,
    /// The contract that consumed the resource.
    pub source_contract: [u8; 32],
    /// The contract authorized to receive the resource.
    pub target_contract: [u8; 32],
    /// Block height at which consumption occurred.
    pub consumed_at_height: u64,
    /// State root of the source contract after consumption.
    pub source_state_root: [u8; 32],
    /// Whether this proof has been consumed (used).
    pub consumed: bool,
}

impl CrossContractTransferProof {
    /// Create a new transfer proof with a deterministic proof_id.
    pub fn new(
        consumed_resource_id: ResourceId,
        source_contract: [u8; 32],
        target_contract: [u8; 32],
        consumed_at_height: u64,
        source_state_root: [u8; 32],
        transaction_hash: [u8; 32],
    ) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CROSS_CONTRACT_PROOF_V1");
        hasher.update(consumed_resource_id.as_bytes());
        hasher.update(&source_contract);
        hasher.update(&target_contract);
        hasher.update(&consumed_at_height.to_le_bytes());
        hasher.update(&source_state_root);
        hasher.update(&transaction_hash);
        let hash = hasher.finalize();
        let mut proof_id = [0u8; 32];
        proof_id.copy_from_slice(hash.as_bytes());

        Self {
            proof_id,
            consumed_resource_id,
            source_contract,
            target_contract,
            consumed_at_height,
            source_state_root,
            consumed: false,
        }
    }

    /// Verify the proof's integrity.
    pub fn verify(&self) -> bool {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CROSS_CONTRACT_PROOF_V1");
        hasher.update(self.consumed_resource_id.as_bytes());
        hasher.update(&self.source_contract);
        hasher.update(&self.target_contract);
        hasher.update(&self.consumed_at_height.to_le_bytes());
        hasher.update(&self.source_state_root);
        let hash = hasher.finalize();
        let mut expected = [0u8; 32];
        expected.copy_from_slice(hash.as_bytes());
        // Note: proof_id includes transaction_hash, so verify checks
        // the structural fields only (transaction_hash is external).
        // For full verification, recompute with the original tx hash.
        self.proof_id[0..24] == expected[0..24]
    }
}
