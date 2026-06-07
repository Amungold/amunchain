use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};
use amun_vm_kernel::pending_buffer::OperationRecord;
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleProof {
    pub resource_id: ResourceId,
    pub state_root: [u8; 32],
    pub siblings: Vec<([u8; 32], bool)>,
}

impl MerkleProof {
    pub fn verify(&self, metadata: &ResourceMetadata) -> bool {
        let leaf_hash = ResourceRegistry::hash_resource(metadata);
        let mut current = leaf_hash;
        for (sibling, is_left) in &self.siblings {
            let mut hasher = blake3::Hasher::new();
            hasher.update(b"AMUN_MERKLE_NODE_V1");
            if *is_left {
                hasher.update(&current);
                hasher.update(sibling);
            } else {
                hasher.update(sibling);
                hasher.update(&current);
            }
            let hash = hasher.finalize();
            current.copy_from_slice(hash.as_bytes());
        }
        current == self.state_root
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageProof {
    pub resource_id: ResourceId,
    pub genesis_id: ResourceId,
    pub chain: Vec<(ResourceId, [u8; 32])>,
}

impl LineageProof {
    pub fn verify(&self) -> bool {
        if self.chain.is_empty() {
            return self.resource_id == self.genesis_id;
        }
        let mut expected_parent = self.resource_id;
        for (parent_id, _parent_hash) in &self.chain {
            if parent_id == &expected_parent {
                return false;
            }
            expected_parent = *parent_id;
        }
        expected_parent == self.genesis_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessBundle {
    pub consumed_proofs: Vec<MerkleProof>,
    pub lineage_proofs: Vec<LineageProof>,
    pub produced_metadata: Vec<ResourceMetadata>,
}

impl WitnessBundle {
    pub fn size_bytes(&self) -> usize {
        serde_json::to_vec(self).map(|v| v.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.consumed_proofs.is_empty() && self.lineage_proofs.is_empty() && self.produced_metadata.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnhancedTransitionProof {
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
    pub witness: WitnessBundle,
}
