pub mod enhanced_proof;
pub mod pccv_verifier;
pub mod witness_builder;
pub mod transition_proof_engine;

pub use enhanced_proof::*;
pub use pccv_verifier::*;
pub use witness_builder::*;
pub use transition_proof_engine::*;

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::{
        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata,
        ResourceRegistry, ResourceState,
    };
    use amun_vm_kernel::pending_buffer::OperationRecord;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    #[test]
    fn n49_structural_verify_empty() {
        let proof = EnhancedTransitionProof {
            transaction_hash: [0xaa; 32], contract_id: make_id(1),
            block_height: 1, block_hash: [0u8; 32],
            pre_state_root: [0x01; 32], post_state_root: [0x02; 32],
            consumed_resources: vec![], produced_resources: vec![],
            operation_log: vec![], evidence: vec![],
            gas_used: 10, proof_hash: [0u8; 32],
            witness: WitnessBundle {
                consumed_proofs: vec![], lineage_proofs: vec![], produced_metadata: vec![],
            },
        };
        let mut p = proof.clone();
        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
        let reg = ResourceRegistry::new(1000);
        let result = PCCVVerifier::verify(&p, &reg);
        assert!(matches!(result, PCCVResult::Verified { .. }));
    }

    #[test]
    fn n49_detect_r1_duplicate_id() {
        let mut reg = ResourceRegistry::new(1000);
        let id = make_id(1);
        reg.register_genesis(ResourceMetadata {
            resource_id: id, archetype: ResourceArchetype::Asset,
            state: ResourceState::Active, lineage: ResourceLineage::genesis(id),
            contract_id: [1u8; 32], owner: [2u8; 32],
        }).unwrap();
        let proof = EnhancedTransitionProof {
            transaction_hash: [0xbb; 32], contract_id: make_id(99),
            block_height: 1, block_hash: [0u8; 32],
            pre_state_root: reg.compute_state_root(),
            post_state_root: reg.compute_state_root(),
            consumed_resources: vec![],
            produced_resources: vec![
                ResourceMetadata {
                    resource_id: id, archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active, lineage: ResourceLineage::genesis(id),
                    contract_id: [1u8; 32], owner: [2u8; 32],
                },
                ResourceMetadata {
                    resource_id: id, archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active, lineage: ResourceLineage::genesis(id),
                    contract_id: [1u8; 32], owner: [2u8; 32],
                },
            ],
            operation_log: vec![],
            evidence: vec![],
            gas_used: 10, proof_hash: [0u8; 32],
            witness: WitnessBundle {
                consumed_proofs: vec![], lineage_proofs: vec![], produced_metadata: vec![],
            },
        };
        let mut p = proof;
        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
        let result = PCCVVerifier::verify(&p, &reg);
        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R1")));
    }

    #[test]
    fn n49_detect_r6_version_regression() {
        let mut reg = ResourceRegistry::new(1000);
        let parent = make_id(1);
        reg.register_genesis(ResourceMetadata {
            resource_id: parent, archetype: ResourceArchetype::Asset,
            state: ResourceState::Active, lineage: ResourceLineage::genesis(parent),
            contract_id: [1u8; 32], owner: [2u8; 32],
        }).unwrap();
        let proof = EnhancedTransitionProof {
            transaction_hash: [0xcc; 32], contract_id: make_id(99),
            block_height: 1, block_hash: [0u8; 32],
            pre_state_root: reg.compute_state_root(),
            post_state_root: reg.compute_state_root(),
            consumed_resources: vec![parent],
            produced_resources: vec![
                ResourceMetadata {
                    resource_id: make_id(2), archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::single_ancestor(make_id(2), parent, [0u8; 32], 1),
                    contract_id: [1u8; 32], owner: [2u8; 32],
                },
            ],
            operation_log: vec![],
            evidence: vec![],
            gas_used: 10, proof_hash: [0u8; 32],
            witness: WitnessBundle {
                consumed_proofs: vec![],
                lineage_proofs: vec![],
                produced_metadata: vec![ResourceMetadata {
                    resource_id: parent, archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active, lineage: ResourceLineage::genesis(parent),
                    contract_id: [1u8; 32], owner: [2u8; 32],
                }],
            },
        };
        let mut p = proof;
        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
        let result = PCCVVerifier::verify(&p, &reg);
        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R6")));
    }

    #[test]
    fn n49_detect_t1_illegal_transformation() {
        let mut reg = ResourceRegistry::new(1000);
        let ev = make_id(1);
        reg.register_genesis(ResourceMetadata {
            resource_id: ev, archetype: ResourceArchetype::Evidence,
            state: ResourceState::Active, lineage: ResourceLineage::genesis(ev),
            contract_id: [1u8; 32], owner: [2u8; 32],
        }).unwrap();
        let proof = EnhancedTransitionProof {
            transaction_hash: [0xdd; 32], contract_id: make_id(99),
            block_height: 1, block_hash: [0u8; 32],
            pre_state_root: reg.compute_state_root(),
            post_state_root: reg.compute_state_root(),
            consumed_resources: vec![ev],
            produced_resources: vec![
                ResourceMetadata {
                    resource_id: make_id(2), archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::transformation(make_id(2), ev, [0u8; 32], 2),
                    contract_id: [1u8; 32], owner: [2u8; 32],
                },
            ],
            operation_log: vec![],
            evidence: vec![],
            gas_used: 10, proof_hash: [0u8; 32],
            witness: WitnessBundle {
                consumed_proofs: vec![],
                lineage_proofs: vec![],
                produced_metadata: vec![ResourceMetadata {
                    resource_id: ev, archetype: ResourceArchetype::Evidence,
                    state: ResourceState::Active, lineage: ResourceLineage::genesis(ev),
                    contract_id: [1u8; 32], owner: [2u8; 32],
                }],
            },
        };
        let mut p = proof;
        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
        let result = PCCVVerifier::verify(&p, &reg);
        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("T1")));
    }

    #[test]
    fn n49_full_semantic_verification_passes() {
        let mut reg = ResourceRegistry::new(1000);
        let parent = make_id(1);
        let child = make_id(2);
        reg.register_genesis(ResourceMetadata {
            resource_id: parent, archetype: ResourceArchetype::Asset,
            state: ResourceState::Active, lineage: ResourceLineage::genesis(parent),
            contract_id: [1u8; 32], owner: [2u8; 32],
        }).unwrap();
        let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
        let proof = EnhancedTransitionProof {
            transaction_hash: [0xee; 32], contract_id: make_id(99),
            block_height: 1, block_hash: [0u8; 32],
            pre_state_root: reg.compute_state_root(),
            post_state_root: reg.compute_state_root(),
            consumed_resources: vec![parent],
            produced_resources: vec![
                ResourceMetadata {
                    resource_id: child, archetype: ResourceArchetype::ConstitutionalAsset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::transformation(child, parent, parent_hash, 2),
                    contract_id: [1u8; 32], owner: [2u8; 32],
                },
            ],
            operation_log: vec![OperationRecord {
                opcode: "OP_TRANSFORM".into(), inputs: vec![0], outputs: vec![1],
            }],
            evidence: vec![],
            gas_used: 15, proof_hash: [0u8; 32],
            witness: WitnessBundle {
                consumed_proofs: vec![],
                lineage_proofs: vec![],
                produced_metadata: vec![ResourceMetadata {
                    resource_id: parent, archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active, lineage: ResourceLineage::genesis(parent),
                    contract_id: [1u8; 32], owner: [2u8; 32],
                }],
            },
        };
        let mut p = proof;
        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
        let result = PCCVVerifier::verify(&p, &reg);
        assert!(matches!(result, PCCVResult::Verified { .. }));
    }
}
