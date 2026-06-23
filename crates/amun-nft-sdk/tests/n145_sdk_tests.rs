use amun_nft_core::NftMetadata;
use amun_nft_sdk::NftSdk;
use amun_resource_core::ResourceId;

#[test]
fn n145_mint_and_query_via_sdk() {
    let sdk = NftSdk::new();
    let col = [1u8; 32];
    let token = [2u8; 32];
    let owner = [10u8; 32];
    sdk.register_collection(col, [0u8; 32]).unwrap();
    sdk.mint_nft(
        col,
        token,
        owner,
        &NftMetadata {
            name: "Test".into(),
            description: "SDK mint".into(),
            image_uri: "ipfs://test".into(),
            attributes: vec![],
            constitutional_role: None,
        },
    )
    .unwrap();
    sdk.constitutional_registry.lock().unwrap().register(
        amun_nft_constitutional_registry::NftConstitutionalRecord {
            token_id: token,
            owner,
            collection_id: Some(col),
            creator: owner,
            mining_origin: Some("SDK".into()),
            royalty_policy: None,
            governance_right: None,
            bridge_lock: None,
        },
    );
    sdk.index_all();
    let info = sdk.get_nft_info(token).unwrap();
    assert_eq!(info.owner, owner);
    assert_eq!(info.mining_origin, Some("SDK".into()));
}

#[test]
fn n145_transfer_via_sdk() {
    let sdk = NftSdk::new();
    let token = [5u8; 32];
    let owner = [10u8; 32];
    let new_owner = [20u8; 32];
    sdk.registry
        .lock()
        .unwrap()
        .register_genesis(amun_resource_core::ResourceMetadata {
            resource_id: ResourceId(token),
            archetype: amun_resource_core::ResourceArchetype::NFTAsset,
            state: amun_resource_core::ResourceState::Active,
            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
            contract_id: [0u8; 32],
            owner,
        })
        .unwrap();
    let new_id = sdk.transfer_nft(token, new_owner).unwrap();
    let transferred = sdk.registry.lock().unwrap().get(&new_id).cloned().unwrap();
    assert_eq!(transferred.owner, new_owner);
}

#[test]
fn n145_list_and_buy_via_sdk() {
    let sdk = NftSdk::new();
    let token = [3u8; 32];
    let seller = [10u8; 32];
    let buyer = [20u8; 32];
    sdk.registry
        .lock()
        .unwrap()
        .register_genesis(amun_resource_core::ResourceMetadata {
            resource_id: ResourceId(token),
            archetype: amun_resource_core::ResourceArchetype::NFTAsset,
            state: amun_resource_core::ResourceState::Active,
            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
            contract_id: [0u8; 32],
            owner: seller,
        })
        .unwrap();
    sdk.list_nft(token, seller, 100).unwrap();
    let new_id = sdk.buy_nft(token, buyer, 1, 1000).unwrap();
    let nft = sdk.registry.lock().unwrap().get(&new_id).cloned().unwrap();
    assert_eq!(nft.owner, buyer);
}

#[test]
fn n145_auction_flow_via_sdk() {
    let sdk = NftSdk::new();
    let token = [7u8; 32];
    let seller = [10u8; 32];
    let bidder = [20u8; 32];
    sdk.registry
        .lock()
        .unwrap()
        .register_genesis(amun_resource_core::ResourceMetadata {
            resource_id: ResourceId(token),
            archetype: amun_resource_core::ResourceArchetype::NFTAsset,
            state: amun_resource_core::ResourceState::Active,
            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
            contract_id: [0u8; 32],
            owner: seller,
        })
        .unwrap();
    sdk.start_auction(token, seller, 1000).unwrap();
    sdk.place_bid(token, bidder, 50, 500).unwrap();
    let new_id = sdk.end_auction(token, 1001, 1, 1001).unwrap();
    let nft = sdk.registry.lock().unwrap().get(&new_id).cloned().unwrap();
    assert_eq!(nft.owner, bidder);
}

#[test]
fn n145_full_sdk_integration_flow() {
    let sdk = NftSdk::new();
    let col = [10u8; 32];
    let creator = [1u8; 32];
    let owner = [2u8; 32];
    let buyer = [3u8; 32];
    let token = [20u8; 32];

    // 1. Register collection and mint
    sdk.register_collection(col, creator).unwrap();
    sdk.mint_nft(
        col,
        token,
        owner,
        &NftMetadata {
            name: "Integration".into(),
            description: "Full test".into(),
            image_uri: "ipfs://full".into(),
            attributes: vec![],
            constitutional_role: None,
        },
    )
    .unwrap();

    // 2. List and buy
    sdk.list_nft(token, owner, 500).unwrap();
    let new_id = sdk.buy_nft(token, buyer, 1, 1000).unwrap();
    let nft = sdk.registry.lock().unwrap().get(&new_id).cloned().unwrap();
    assert_eq!(nft.owner, buyer);

    // 3. Index
    sdk.constitutional_registry.lock().unwrap().register(
        amun_nft_constitutional_registry::NftConstitutionalRecord {
            token_id: token,
            owner: buyer,
            collection_id: Some(col),
            creator,
            mining_origin: Some("FullIntegration".into()),
            royalty_policy: None,
            governance_right: None,
            bridge_lock: None,
        },
    );
    sdk.index_all();
    let info = sdk.get_nft_info(token).unwrap();
    assert_eq!(info.owner, buyer);
}
