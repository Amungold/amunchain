#![allow(clippy::too_many_arguments)]
use amun_resource_core::{ResourceId, ResourceMetadata, ResourceRegistry};
use amun_vm_kernel::pending_buffer::PendingBuffer;
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use amun_evidence_engine::evidence_engine::EvidenceEngine;

use crate::transition_proof::TransitionProof;
use crate::enhanced_proof::{EnhancedTransitionProof, WitnessBundle, MerkleProof, LineageProof};

pub struct ProofBuilder;

impl ProofBuilder {
    pub fn build(
        buffer: &PendingBuffer,
        contract_id: ResourceId,
        block_height: u64,
        block_hash: [u8; 32],
        transaction_hash: [u8; 32],
        pre_state_root: [u8; 32],
        post_state_root: [u8; 32],
        gas_used: u64,
    ) -> TransitionProof {
        let consumed_resources: Vec<ResourceId> = buffer.consumed_handles().iter().map(|(_, rid, _)| *rid).collect();
        let produced_resources: Vec<ResourceMetadata> = buffer.produced_resources().iter().map(|m| (*m).clone()).collect();
        let operation_log = buffer.operation_log();
        let evidence: Vec<ConstitutionalEvidence> = buffer.all_evidence().iter()
            .map(|ev| EvidenceEngine::convert(ev, contract_id, block_height, transaction_hash)).collect();
        TransitionProof::new(transaction_hash, contract_id, block_height, block_hash,
            pre_state_root, post_state_root, consumed_resources, produced_resources, operation_log, evidence, gas_used)
    }

    pub fn build_enhanced(
        buffer: &PendingBuffer,
        registry: &ResourceRegistry,
        contract_id: ResourceId,
        block_height: u64,
        block_hash: [u8; 32],
        transaction_hash: [u8; 32],
        pre_state_root: [u8; 32],
        post_state_root: [u8; 32],
        gas_used: u64,
    ) -> EnhancedTransitionProof {
        let consumed_resources: Vec<ResourceId> = buffer.consumed_handles().iter().map(|(_, rid, _)| *rid).collect();
        let produced_resources: Vec<ResourceMetadata> = buffer.produced_resources().iter().map(|m| (*m).clone()).collect();
        let operation_log = buffer.operation_log();
        let evidence: Vec<ConstitutionalEvidence> = buffer.all_evidence().iter()
            .map(|ev| EvidenceEngine::convert(ev, contract_id, block_height, transaction_hash)).collect();

        let mut consumed_proofs = Vec::new();
        let mut lineage_proofs = Vec::new();
        for rid in &consumed_resources {
            if registry.get(rid).is_some() {
                let mp = Self::build_merkle_proof(registry, rid, pre_state_root);
                consumed_proofs.push(mp);
                let lp = Self::build_lineage_proof(registry, rid);
                lineage_proofs.push(lp);
            }
        }

        let witness = WitnessBundle {
            consumed_proofs,
            lineage_proofs,
            produced_metadata: produced_resources.clone(),
        };

        let mut proof = EnhancedTransitionProof {
            transaction_hash, contract_id, block_height, block_hash,
            pre_state_root, post_state_root,
            consumed_resources, produced_resources,
            operation_log, evidence, gas_used,
            proof_hash: [0u8; 32],
            witness,
        };
        proof.proof_hash = Self::compute_enhanced_hash(&proof);
        proof
    }

    fn build_merkle_proof(registry: &ResourceRegistry, rid: &ResourceId, pre_state_root: [u8; 32]) -> MerkleProof {
        let siblings = Self::compute_siblings(registry, rid);
        MerkleProof { resource_id: *rid, state_root: pre_state_root, siblings }
    }

    fn compute_siblings(registry: &ResourceRegistry, rid: &ResourceId) -> Vec<([u8; 32], bool)> {
        let mut siblings = Vec::new();
        let ids = registry.active_ids();
        if ids.is_empty() { return siblings; }
        let leaves: Vec<([u8; 32], ResourceId)> = ids.iter()
            .filter_map(|id| registry.get(id).map(|m| (ResourceRegistry::hash_resource(m), *id)))
            .collect();
        let pos = leaves.iter().position(|(_, id)| id == rid);
        if pos.is_none() { return siblings; }
        let mut idx = pos.unwrap();
        let mut level: Vec<[u8; 32]> = leaves.iter().map(|(h, _)| *h).collect();
        while level.len() > 1 {
            let is_left = idx % 2 == 0;
            let sib = if is_left { idx + 1 } else { idx - 1 };
            if sib < level.len() { siblings.push((level[sib], !is_left)); }
            let mut next = Vec::new();
            for chunk in level.chunks(2) {
                let mut h = blake3::Hasher::new();
                h.update(b"AMUN_MERKLE_NODE_V1");
                h.update(&chunk[0]);
                h.update(if chunk.len() == 2 { &chunk[1] } else { &chunk[0] });
                let hash = h.finalize();
                let mut arr = [0u8; 32];
                arr.copy_from_slice(hash.as_bytes());
                next.push(arr);
            }
            idx /= 2;
            level = next;
        }
        siblings
    }

    fn build_lineage_proof(registry: &ResourceRegistry, rid: &ResourceId) -> LineageProof {
        let mut chain = Vec::new();
        let mut cur = *rid;
        let mut gen = *rid;
        while let Some(m) = registry.get(&cur) {
            if m.lineage.parent_resource_ids.is_empty() { gen = cur; break; }
            let p = m.lineage.parent_resource_ids[0];
            let h = if m.lineage.parent_hashes.len() == 1 { m.lineage.parent_hashes[0] } else { [0u8; 32] };
            chain.push((p, h));
            cur = p;
        }
        LineageProof { resource_id: *rid, genesis_id: gen, chain }
    }

    fn compute_enhanced_hash(proof: &EnhancedTransitionProof) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(b"AMUN_ENHANCED_PROOF_V1");
        h.update(&proof.transaction_hash); h.update(proof.contract_id.as_bytes());
        h.update(&proof.block_height.to_le_bytes()); h.update(&proof.block_hash);
        h.update(&proof.pre_state_root); h.update(&proof.post_state_root);
        h.update(&proof.gas_used.to_le_bytes());
        for id in &proof.consumed_resources { h.update(id.as_bytes()); }
        for m in &proof.produced_resources { h.update(m.resource_id.as_bytes()); h.update(&m.lineage.version.to_le_bytes()); }
        for ev in &proof.evidence { h.update(&ev.evidence_id()); }
        let wb = serde_json::to_vec(&proof.witness).unwrap_or_default(); h.update(&wb);
        let hash = h.finalize(); let mut arr = [0u8; 32]; arr.copy_from_slice(hash.as_bytes()); arr
    }
}
