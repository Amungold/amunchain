use amun_nft_marketplace::MarketplaceEngine;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};

#[test]
fn n134_list_and_buy_nft() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([1u8; 32]);
    let seller = [5u8; 32];
    let buyer = [6u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: seller,
    })
    .unwrap();

    let mut mp = MarketplaceEngine::new();
    mp.list_nft(&reg, token_id, &seller, 100, None).unwrap();
    let new_id = mp.buy_nft(&mut reg, &token_id, &buyer, 1, 1000).unwrap();

    assert!(matches!(
        reg.get(&token_id).unwrap().state,
        ResourceState::Consumed { .. }
    ));
    assert_eq!(reg.get(&new_id).unwrap().owner, buyer);
    assert_eq!(mp.event_log().len(), 2); // Listing + Sale
}

#[test]
fn n134_cancel_listing() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([1u8; 32]);
    let seller = [5u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: seller,
    })
    .unwrap();

    let mut mp = MarketplaceEngine::new();
    mp.list_nft(&reg, token_id, &seller, 100, None).unwrap();
    mp.cancel_listing(&token_id).unwrap();

    let result = mp.buy_nft(&mut reg, &token_id, &[9u8; 32], 1, 1000);
    assert!(result.is_err());
}

#[test]
fn n134_auction_flow() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([1u8; 32]);
    let seller = [5u8; 32];
    let bidder1 = [6u8; 32];
    let bidder2 = [7u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: seller,
    })
    .unwrap();

    let mut mp = MarketplaceEngine::new();
    mp.start_auction(&reg, token_id, &seller, 1000).unwrap();
    mp.place_bid(&token_id, &bidder1, 50, 500).unwrap();
    mp.place_bid(&token_id, &bidder2, 100, 600).unwrap();

    let new_id = mp.end_auction(&mut reg, &token_id, 1001, 1, 1001).unwrap();
    assert_eq!(reg.get(&new_id).unwrap().owner, bidder2);
}

// === N134.1 Hardening Tests ===

#[test]
fn n134_1_prevent_double_buy() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([1u8; 32]);
    let seller = [5u8; 32];
    let buyer1 = [6u8; 32];
    let buyer2 = [7u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: seller,
    })
    .unwrap();

    let mut mp = MarketplaceEngine::new();
    mp.list_nft(&reg, token_id, &seller, 100, None).unwrap();
    mp.buy_nft(&mut reg, &token_id, &buyer1, 1, 1000).unwrap();

    // Second buy must fail (listing no longer active + token consumed)
    let result = mp.buy_nft(&mut reg, &token_id, &buyer2, 2, 2000);
    assert!(result.is_err());
}

#[test]
fn n134_1_prevent_self_purchase() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([1u8; 32]);
    let seller = [5u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: seller,
    })
    .unwrap();

    let mut mp = MarketplaceEngine::new();
    mp.list_nft(&reg, token_id, &seller, 100, None).unwrap();

    let result = mp.buy_nft(&mut reg, &token_id, &seller, 1, 1000);
    assert!(result.is_err());
}

#[test]
fn n134_1_prevent_bid_below_highest() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([1u8; 32]);
    let seller = [5u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: seller,
    })
    .unwrap();

    let mut mp = MarketplaceEngine::new();
    mp.start_auction(&reg, token_id, &seller, 1000).unwrap();
    mp.place_bid(&token_id, &[6u8; 32], 100, 500).unwrap();

    let result = mp.place_bid(&token_id, &[7u8; 32], 50, 600);
    assert!(result.is_err());
}

#[test]
fn n134_1_marketplace_evidence_root() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([1u8; 32]);
    let seller = [5u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: seller,
    })
    .unwrap();

    let mut mp = MarketplaceEngine::new();
    mp.list_nft(&reg, token_id, &seller, 100, None).unwrap();

    let root = mp.compute_evidence_root();
    assert_ne!(root, [0u8; 32]);
}
