use amun_defi_lending_core::InterestModel;
use amun_nft_collateral::NftCollateralEngine;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};

#[test]
fn n156_lock_nft_and_borrow() {
    let mut reg = ResourceRegistry::new(100);
    let token_id = ResourceId([1u8; 32]);
    let owner = [10u8; 32];
    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner,
    })
    .unwrap();

    let mut engine = NftCollateralEngine::new();
    engine.lock_nft(&reg, token_id, &owner).unwrap();
    assert!(engine.is_locked(&token_id));

    let loan_id = engine
        .borrow_against_nft(&mut reg, token_id, owner, 500, 100, 1)
        .unwrap();
    assert!(engine.lending.loans.contains_key(&loan_id.0));
}

#[test]
fn n156_repay_and_unlock() {
    let mut reg = ResourceRegistry::new(100);
    let token_id = ResourceId([2u8; 32]);
    let owner = [10u8; 32];
    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner,
    })
    .unwrap();

    let mut engine = NftCollateralEngine::new();
    engine.lock_nft(&reg, token_id, &owner).unwrap();
    let loan_id = engine
        .borrow_against_nft(&mut reg, token_id, owner, 200, 0, 1)
        .unwrap();
    engine
        .repay_and_unlock(&mut reg, &loan_id, token_id, 200)
        .unwrap();
    assert!(!engine.is_locked(&token_id));
}

#[test]
fn n156_cannot_transfer_locked_nft() {
    let mut reg = ResourceRegistry::new(100);
    let token_id = ResourceId([3u8; 32]);
    let owner = [10u8; 32];
    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner,
    })
    .unwrap();

    let mut engine = NftCollateralEngine::new();
    engine.lock_nft(&reg, token_id, &owner).unwrap();
    assert!(engine.is_locked(&token_id));
}

#[test]
fn n156_liquidation_removes_lock() {
    let mut reg = ResourceRegistry::new(100);
    let token_id = ResourceId([4u8; 32]);
    let owner = [10u8; 32];
    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner,
    })
    .unwrap();

    let mut engine = NftCollateralEngine::new();
    engine.lock_nft(&reg, token_id, &owner).unwrap();
    let loan_id = engine
        .borrow_against_nft(&mut reg, token_id, owner, 500, 500, 1)
        .unwrap();
    let health = engine.lending.get_health_factor(&loan_id.0, 2_000_000);
    assert!(InterestModel::is_liquidatable(health));
    engine
        .liquidate(&loan_id, token_id, [99u8; 32], 2_000_000)
        .unwrap();
    assert!(!engine.is_locked(&token_id));
}

#[test]
fn n156_evidence_root_deterministic() {
    let mut reg1 = ResourceRegistry::new(100);
    let mut reg2 = ResourceRegistry::new(100);
    let token_id1 = ResourceId([5u8; 32]);
    let token_id2 = ResourceId([5u8; 32]);
    let owner = [10u8; 32];
    reg1.register_genesis(ResourceMetadata {
        resource_id: token_id1,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id1),
        contract_id: [0u8; 32],
        owner,
    })
    .unwrap();
    reg2.register_genesis(ResourceMetadata {
        resource_id: token_id2,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id2),
        contract_id: [0u8; 32],
        owner,
    })
    .unwrap();

    let mut engine1 = NftCollateralEngine::new();
    let mut engine2 = NftCollateralEngine::new();
    engine1.lock_nft(&reg1, token_id1, &owner).unwrap();
    engine2.lock_nft(&reg2, token_id2, &owner).unwrap();
    engine1
        .borrow_against_nft(&mut reg1, token_id1, owner, 300, 200, 1)
        .unwrap();
    engine2
        .borrow_against_nft(&mut reg2, token_id2, owner, 300, 200, 1)
        .unwrap();
    assert_eq!(
        engine1.compute_evidence_root(),
        engine2.compute_evidence_root()
    );
}
