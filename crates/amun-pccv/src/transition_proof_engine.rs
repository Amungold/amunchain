#![allow(clippy::too_many_arguments)]
use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceRegistry,
};
use amun_vm_kernel::pending_buffer::PendingBuffer;

use crate::enhanced_proof::{EnhancedTransitionProof};
use crate::witness_builder::WitnessBuilder;
use crate::pccv_verifier::{PCCVVerifier, PCCVResult};

pub struct TransitionProofEngine;

impl TransitionProofEngine {
    pub fn build_proof(
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
        let consumed_resources: Vec<ResourceId> = buffer
            .consumed_handles()
            .iter()
            .map(|(_, rid, _)| *rid)
            .collect();

        let produced_resources: Vec<ResourceMetadata> = buffer
            .produced_resources()
            .iter()
            .map(|m| (*m).clone())
            .collect();

        let operation_log = buffer.operation_log();

        let evidence: Vec<amun_evidence_engine::evidence_types::ConstitutionalEvidence> = buffer
            .all_evidence()
            .iter()
            .map(|ev| {
                amun_evidence_engine::evidence_engine::EvidenceEngine::convert(
                    ev, contract_id, block_height, transaction_hash,
                )
            })
            .collect();

        let witness = WitnessBuilder::build(registry, &consumed_resources, &produced_resources);

        let mut proof = EnhancedTransitionProof {
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
            witness,
        };

        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
        proof
    }

    pub fn prove_and_verify(
        buffer: &PendingBuffer,
        registry: &ResourceRegistry,
        contract_id: ResourceId,
        block_height: u64,
        block_hash: [u8; 32],
        transaction_hash: [u8; 32],
        pre_state_root: [u8; 32],
        post_state_root: [u8; 32],
        gas_used: u64,
    ) -> (EnhancedTransitionProof, PCCVResult) {
        let proof = Self::build_proof(
            buffer, registry, contract_id, block_height, block_hash,
            transaction_hash, pre_state_root, post_state_root, gas_used,
        );
        let result = PCCVVerifier::verify(&proof, registry);
        (proof, result)
    }
}

#[cfg(test)]
mod tests {
    use amun_resource_core::{
        ResourceId, ResourceMetadata, ResourceRegistry,
        ResourceState, ResourceLineage, ResourceArchetype,
    };
    use crate::enhanced_proof::EnhancedTransitionProof;
    use crate::enhanced_proof::WitnessBundle;
    use crate::TransitionProofEngine;
    use crate::PCCVVerifier;
    use crate::PCCVResult;
    use amun_vm_kernel::pending_buffer::PendingBuffer;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    #[test]
    fn n49b_build_and_verify_simple_transition() {
        let mut reg = ResourceRegistry::new(1000);
        let parent = make_id(1);
        let child_id = make_id(2);
        let parent_meta = ResourceMetadata {
            resource_id: parent,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(parent),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        reg.register_genesis(parent_meta.clone()).unwrap();
        let pre_root = reg.compute_state_root();
        let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
        let pre_state = vec![parent_meta.clone()];
        let mut buffer = PendingBuffer::new(pre_state);
        let child_meta = ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::ConstitutionalAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::transformation(child_id, parent, parent_hash, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        buffer.register_production(child_meta.clone());
        buffer.register_consumption(0, ResourceState::Consumed {
            derived_children: vec![child_id],
        }).unwrap();
        buffer.record_operation("OP_TRANSFORM", vec![0], vec![1]);
        let proof = TransitionProofEngine::build_proof(
            &buffer, &reg, make_id(99), 1, [0u8; 32],
            [0xaa; 32], pre_root, pre_root, 15,
        );
        let result = PCCVVerifier::verify(&proof, &reg);
        assert!(matches!(result, PCCVResult::Verified { .. }));
    }

    #[test]
    fn n49b_detect_illegal_transition_in_proof() {
        let mut reg = ResourceRegistry::new(1000);
        let ev = make_id(1);
        let child_id = make_id(2);
        let ev_meta = ResourceMetadata {
            resource_id: ev,
            archetype: ResourceArchetype::Evidence,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(ev),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        reg.register_genesis(ev_meta.clone()).unwrap();
        let pre_root = reg.compute_state_root();
        let parent_hash = ResourceRegistry::hash_resource(reg.get(&ev).unwrap());
        let pre_state = vec![ev_meta.clone()];
        let mut buffer = PendingBuffer::new(pre_state);
        buffer.register_production(ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::transformation(child_id, ev, parent_hash, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        });
        buffer.register_consumption(0, ResourceState::Consumed {
            derived_children: vec![child_id],
        }).unwrap();
        buffer.record_operation("OP_TRANSFORM", vec![0], vec![1]);
        let consumed_resources = vec![ev];
        let produced_resources = vec![ResourceMetadata {
            resource_id: child_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::transformation(child_id, ev, parent_hash, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        }];
        let witness = WitnessBundle {
            consumed_proofs: vec![],
            lineage_proofs: vec![],
            produced_metadata: vec![ev_meta],
        };
        let mut proof = EnhancedTransitionProof {
            transaction_hash: [0xbb; 32],
            contract_id: make_id(99),
            block_height: 1,
            block_hash: [0u8; 32],
            pre_state_root: pre_root,
            post_state_root: pre_root,
            consumed_resources,
            produced_resources,
            operation_log: buffer.operation_log(),
            evidence: vec![],
            gas_used: 15,
            proof_hash: [0u8; 32],
            witness,
        };
        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
        let result = PCCVVerifier::verify(&proof, &reg);
        assert!(matches!(result, PCCVResult::Failed { ref reason } if reason.contains("T1")));
    }

    #[test]
    fn n49b_proof_hash_deterministic() {
        let mut reg = ResourceRegistry::new(1000);
        let parent = make_id(1);
        reg.register_genesis(ResourceMetadata {
            resource_id: parent,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(parent),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        }).unwrap();
        let pre_root = reg.compute_state_root();
        let buffer = PendingBuffer::new(vec![]);
        let proof1 = TransitionProofEngine::build_proof(
            &buffer, &reg, make_id(99), 1, [0u8; 32],
            [0xcc; 32], pre_root, pre_root, 0,
        );
        let proof2 = TransitionProofEngine::build_proof(
            &buffer, &reg, make_id(99), 1, [0u8; 32],
            [0xcc; 32], pre_root, pre_root, 0,
        );
        assert_eq!(proof1.proof_hash, proof2.proof_hash);
    }
}
