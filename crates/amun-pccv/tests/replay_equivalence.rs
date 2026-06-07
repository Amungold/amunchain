use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata,
    ResourceRegistry, ResourceState,
};
use amun_vm_kernel::pending_buffer::PendingBuffer;
use amun_pccv::transition_proof_engine::TransitionProofEngine;
use amun_pccv::pccv_verifier::PCCVResult;

fn make_id(seed: u8) -> ResourceId {
    let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
}

#[test]
fn n49c_replay_produces_identical_proof() {
    let mut reg1 = ResourceRegistry::new(1000);
    let mut reg2 = ResourceRegistry::new(1000);

    let parent_meta = ResourceMetadata {
        resource_id: make_id(1),
        archetype: ResourceArchetype::Asset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(make_id(1)),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    };
    let child_id = make_id(2);

    reg1.register_genesis(parent_meta.clone()).unwrap();
    reg2.register_genesis(parent_meta.clone()).unwrap();

    let pre_root1 = reg1.compute_state_root();
    let pre_root2 = reg2.compute_state_root();
    assert_eq!(pre_root1, pre_root2);

    let parent_hash = ResourceRegistry::hash_resource(reg1.get(&make_id(1)).unwrap());
    let child_meta = ResourceMetadata {
        resource_id: child_id,
        archetype: ResourceArchetype::ConstitutionalAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::transformation(child_id, make_id(1), parent_hash, 2),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    };

    let pre_state = vec![parent_meta.clone()];
    let mut buffer1 = PendingBuffer::new(pre_state.clone());
    let mut buffer2 = PendingBuffer::new(pre_state);

    buffer1.register_production(child_meta.clone());
    buffer1.register_consumption(0, ResourceState::Consumed {
        derived_children: vec![child_id],
    }).unwrap();
    buffer1.record_operation("OP_TRANSFORM", vec![0], vec![1]);

    buffer2.register_production(child_meta);
    buffer2.register_consumption(0, ResourceState::Consumed {
        derived_children: vec![child_id],
    }).unwrap();
    buffer2.record_operation("OP_TRANSFORM", vec![0], vec![1]);

    let proof1 = TransitionProofEngine::build_proof(
        &buffer1, &reg1, make_id(99), 1, [0u8; 32],
        [0xaa; 32], pre_root1, pre_root1, 15,
    );
    let proof2 = TransitionProofEngine::build_proof(
        &buffer2, &reg2, make_id(99), 1, [0u8; 32],
        [0xaa; 32], pre_root2, pre_root2, 15,
    );

    assert_eq!(proof1.proof_hash, proof2.proof_hash);
    assert_eq!(proof1.pre_state_root, proof2.pre_state_root);
    assert_eq!(proof1.post_state_root, proof2.post_state_root);

    let result1 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof1, &reg1);
    let result2 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof2, &reg2);
    assert!(matches!(result1, PCCVResult::Verified { .. }));
    assert!(matches!(result2, PCCVResult::Verified { .. }));
}

#[test]
fn n49c_replay_consistent_across_iterations() {
    let mut reg = ResourceRegistry::new(1000);
    let parent_meta = ResourceMetadata {
        resource_id: make_id(1),
        archetype: ResourceArchetype::Asset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(make_id(1)),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    };
    reg.register_genesis(parent_meta.clone()).unwrap();
    let pre_root = reg.compute_state_root();
    let parent_hash = ResourceRegistry::hash_resource(reg.get(&make_id(1)).unwrap());
    let child_id = make_id(2);

    let pre_state = vec![parent_meta];
    let mut buffer = PendingBuffer::new(pre_state);
    buffer.register_production(ResourceMetadata {
        resource_id: child_id,
        archetype: ResourceArchetype::ConstitutionalAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::transformation(child_id, make_id(1), parent_hash, 2),
        contract_id: [1u8; 32],
        owner: [2u8; 32],
    });
    buffer.register_consumption(0, ResourceState::Consumed {
        derived_children: vec![child_id],
    }).unwrap();
    buffer.record_operation("OP_TRANSFORM", vec![0], vec![1]);

    let proofs: Vec<_> = (0..5).map(|_| {
        TransitionProofEngine::build_proof(
            &buffer, &reg, make_id(99), 1, [0u8; 32],
            [0xaa; 32], pre_root, pre_root, 15,
        )
    }).collect();

    for i in 1..proofs.len() {
        assert_eq!(proofs[0].proof_hash, proofs[i].proof_hash);
    }
}
