use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use amun_nft_marketplace::MarketplaceEngine;
use amun_nft_bridge::{BridgeLock, BridgeLedger};
use amun_nft_evidence::NftEvidenceKernel;
use amun_nft_constitutional_registry::{NftConstitutionalRecord, ConstitutionalRegistry};

#[test]
fn n149_double_mint_rejected() {
    let mut reg = ResourceRegistry::new(10);
    let col_id = ResourceId([1u8; 32]);
    let token_id = ResourceId([2u8; 32]);
    let owner = [5u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    }).unwrap();

    let parent_hash = reg.resource_hash(&col_id).unwrap();
    let version = reg.get(&col_id).unwrap().lineage.version + 1;
    reg.derive_from_collection(&col_id, ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
        contract_id: [0u8; 32],
        owner,
    }).unwrap();

    let result = reg.derive_from_collection(&col_id, ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
        contract_id: [0u8; 32],
        owner,
    });
    assert!(result.is_err());
}

#[test]
fn n149_double_spend_prevented() {
    let mut reg = ResourceRegistry::new(10);
    let mut mp = MarketplaceEngine::new();
    let token_id = ResourceId([3u8; 32]);
    let seller = [10u8; 32];
    let buyer1 = [20u8; 32];
    let buyer2 = [30u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: seller,
    }).unwrap();

    mp.list_nft(&reg, token_id, &seller, 100, None).unwrap();
    mp.buy_nft(&mut reg, &token_id, &buyer1, 1, 1000).unwrap();

    let result = mp.buy_nft(&mut reg, &token_id, &buyer2, 2, 2000);
    assert!(result.is_err());
}

#[test]
fn n149_invalid_evidence_rejected() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([4u8; 32]);

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: [10u8; 32],
    }).unwrap();

    let result = NftEvidenceKernel::verify_metadata_hash(&[1u8; 32], &[2u8; 32]);
    assert!(result.is_err());
}

#[test]
fn n149_unauthorized_transfer_rejected() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([5u8; 32]);
    let owner = [10u8; 32];
    let thief = [99u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner,
    }).unwrap();

    let result = NftEvidenceKernel::verify_ownership(&reg, &token_id, &thief);
    assert!(result.is_err());
}

#[test]
fn n149_bridge_locked_sale_rejected() {
    let mut reg = ResourceRegistry::new(10);
    let mut bridge = BridgeLedger::new();
    let mut constitutional = ConstitutionalRegistry::new();
    let token_id = ResourceId([6u8; 32]);
    let owner = [10u8; 32];
    let _buyer = [20u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner,
    }).unwrap();

    let lock = BridgeLock {
        source_chain: 1,
        token_id: token_id.0,
        owner,
        destination_chain: 2,
        destination_owner: [30u8; 32],
        lock_height: 100,
    };
    let lock_id = bridge.lock(lock);

    constitutional.register(NftConstitutionalRecord {
        token_id: token_id.0,
        owner,
        collection_id: None,
        creator: owner,
        mining_origin: None,
        royalty_policy: None,
        governance_right: None,
        bridge_lock: Some(bridge.locks.get(&lock_id).unwrap().clone()),
    });

    let can_sell = amun_nft_constitutional_enforcement::EnforcementEngine::can_be_sold(
        &constitutional, &bridge, &token_id.0
    );
    assert!(!can_sell);
}
