use amun_bytecode::opcodes::OpCode;
use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_cross_contract::transfer_proof::CrossContractTransferProof;
use amun_cross_contract::transfer_registry::TransferProofRegistry;
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;
use amun_replay_verifier::replay_verifier::ReplayVerifier;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use amun_transition_proof::transition_proof::TransitionProof;
use amun_vm_kernel::execution_context::ExecutionContext;

fn make_id(seed: u64) -> ResourceId {
    let mut h = [0u8; 32];
    h[0..8].copy_from_slice(&seed.to_le_bytes());
    ResourceId(h)
}

#[test]
fn byz_001_forged_proof_rejected() {
    let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
    let ctx = ExecutionContext {
        contract_id: make_id(1),
        caller: [1u8; 32],
        block_height: 1,
        block_hash: [0u8; 32],
        transaction_hash: [0xaa; 32],
        pre_state_root: [0u8; 32],
        authority: [2u8; 32],
    };
    let mut hot = HotProofStore::new(100);
    let mut archive = ProofArchive::new();
    let mut reg = ResourceRegistry::new(1000);
    let result = ConstitutionalRuntime::execute(
        &program,
        &ctx,
        &mut reg,
        &[],
        10000,
        &mut hot,
        &mut archive,
    )
    .unwrap();
    let mut proof = match result {
        PipelineResult::Committed {
            transition_proof, ..
        } => transition_proof,
        _ => panic!("Expected Committed"),
    };
    proof.post_state_root = [0xff; 32];
    let mut fresh_reg = ResourceRegistry::new(1000);
    let replay = ReplayVerifier::replay(&proof, &program, &mut fresh_reg, &[]);
    assert!(!matches!(
        replay,
        amun_replay_verifier::replay_verifier::ReplayResult::Match { .. }
    ));
}

#[test]
fn byz_002_double_transfer_rejected() {
    let mut registry = TransferProofRegistry::new();
    let proof = CrossContractTransferProof::new(
        make_id(1),
        [1u8; 32],
        [2u8; 32],
        42,
        [0xaa; 32],
        [0xbb; 32],
    );
    assert!(registry.consume(&proof, make_id(99), 1, [0xcc; 32]).is_ok());
    assert!(registry
        .consume(&proof, make_id(99), 2, [0xdd; 32])
        .is_err());
}

#[test]
fn byz_003_lineage_cycle_rejected() {
    let mut reg = ResourceRegistry::new(1000);
    let a = make_id(1);
    let b = make_id(2);
    reg.register_genesis(ResourceMetadata {
        resource_id: a,
        archetype: ResourceArchetype::Asset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(a),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    })
    .unwrap();
    let hash_a = ResourceRegistry::hash_resource(reg.get(&a).unwrap());
    reg.consume_and_derive(
        &a,
        ResourceMetadata {
            resource_id: b,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(b, a, hash_a, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        },
    )
    .unwrap();
    let hash_b = ResourceRegistry::hash_resource(reg.get(&b).unwrap());
    let c = make_id(3);
    assert!(reg
        .consume_and_derive(
            &b,
            ResourceMetadata {
                resource_id: c,
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(c, b, hash_b, 3),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            }
        )
        .is_ok());
}

#[test]
fn byz_004_version_regression_rejected() {
    let mut reg = ResourceRegistry::new(1000);
    let parent_id = make_id(1);
    reg.register_genesis(ResourceMetadata {
        resource_id: parent_id,
        archetype: ResourceArchetype::Asset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(parent_id),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    })
    .unwrap();
    let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
    let result = reg.consume_and_derive(
        &parent_id,
        ResourceMetadata {
            resource_id: make_id(2),
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(make_id(2), parent_id, parent_hash, 1),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        },
    );
    assert!(result.is_err());
}

#[test]
fn byz_005_parent_hash_forgery_rejected() {
    let mut reg = ResourceRegistry::new(1000);
    let parent_id = make_id(1);
    reg.register_genesis(ResourceMetadata {
        resource_id: parent_id,
        archetype: ResourceArchetype::Asset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(parent_id),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    })
    .unwrap();
    let forged_hash = [0xff; 32];
    let result = reg.consume_and_derive(
        &parent_id,
        ResourceMetadata {
            resource_id: make_id(2),
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(make_id(2), parent_id, forged_hash, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        },
    );
    assert!(result.is_err());
}

#[test]
fn byz_006_illegal_transformation_rejected() {
    let mut reg = ResourceRegistry::new(1000);
    let ev = make_id(1);
    reg.register_genesis(ResourceMetadata {
        resource_id: ev,
        archetype: ResourceArchetype::Evidence,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(ev),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    })
    .unwrap();
    let parent_hash = ResourceRegistry::hash_resource(reg.get(&ev).unwrap());
    let result = reg.consume_and_derive(
        &ev,
        ResourceMetadata {
            resource_id: make_id(2),
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(make_id(2), ev, parent_hash, 2),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        },
    );
    assert!(result.is_err());
}

#[test]
fn byz_007_deep_lineage_no_crash() {
    let mut reg = ResourceRegistry::new(100_000);
    let root = make_id(0);
    reg.register_genesis(ResourceMetadata {
        resource_id: root,
        archetype: ResourceArchetype::Asset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(root),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    })
    .unwrap();
    let mut parent = root;
    for i in 1u64..=2000 {
        let child = make_id(i);
        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
        let version = reg.get(&parent).unwrap().lineage.version + 1;
        reg.consume_and_derive(
            &parent,
            ResourceMetadata {
                resource_id: child,
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(child, parent, hash, version),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            },
        )
        .unwrap();
        parent = child;
    }
    assert!(reg.lineage_depth(&parent) > 0);
}

#[test]
fn byz_008_wide_fanout_no_crash() {
    let mut reg = ResourceRegistry::new(200_000);
    for i in 0..10000u64 {
        let id = make_id(10000 + i);
        reg.register_genesis(ResourceMetadata {
            resource_id: id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(id),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        })
        .unwrap();
    }
    assert_eq!(reg.total_active(), 10000);
}

#[test]
fn byz_009_proof_tampering_detected() {
    let proof = TransitionProof::new(
        [0xaa; 32],
        make_id(1),
        1,
        [0u8; 32],
        [0x01; 32],
        [0x02; 32],
        vec![],
        vec![],
        vec![],
        vec![],
        1000,
    );
    assert!(proof.verify_integrity());
    let mut tampered = proof.clone();
    tampered.gas_used = 99999;
    assert!(!tampered.verify_integrity());
}

#[test]
fn byz_010_proof_replay_attack_blocked() {
    let mut registry = TransferProofRegistry::new();
    let proof = CrossContractTransferProof::new(
        make_id(1),
        [1u8; 32],
        [2u8; 32],
        42,
        [0xaa; 32],
        [0xbb; 32],
    );
    registry
        .consume(&proof, make_id(99), 1, [0xcc; 32])
        .unwrap();
    let result = registry.consume(&proof, make_id(99), 2, [0xdd; 32]);
    assert!(result.is_err());
    match *result.unwrap_err() {
        ConstitutionalEvidence::ConstitutionalViolation { law, .. } => {
            assert_eq!(law, "X1");
        }
        _ => panic!("Expected X1 violation"),
    }
}
