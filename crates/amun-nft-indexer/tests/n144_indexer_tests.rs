use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
use amun_nft_indexer::NftIndexer;
use amun_nft_marketplace::MarketplaceEvent;
use amun_resource_core::ResourceId;

#[test]
fn n144_index_and_query_nft() {
    let mut reg = ConstitutionalRegistry::new();
    let token = [1u8; 32];
    reg.register(NftConstitutionalRecord {
        token_id: token,
        owner: [10u8; 32],
        collection_id: Some([5u8; 32]),
        creator: [30u8; 32],
        mining_origin: Some("Validator".into()),
        royalty_policy: None,
        governance_right: None,
        bridge_lock: None,
    });

    let mut indexer = NftIndexer::new();
    indexer.index_registry(&reg);
    let nft = indexer.get_nft(&token).unwrap();
    assert_eq!(nft.owner, [10u8; 32]);
    assert_eq!(nft.mining_origin, Some("Validator".into()));
    assert!(!nft.bridge_locked);
}

#[test]
fn n144_query_by_owner() {
    let mut reg = ConstitutionalRegistry::new();
    let owner = [20u8; 32];
    for i in 0..3u8 {
        reg.register(NftConstitutionalRecord {
            token_id: [i; 32],
            owner,
            collection_id: None,
            creator: [30u8; 32],
            mining_origin: None,
            royalty_policy: None,
            governance_right: None,
            bridge_lock: None,
        });
    }
    let mut indexer = NftIndexer::new();
    indexer.index_registry(&reg);
    let owned = indexer.get_nfts_by_owner(&owner);
    assert_eq!(owned.len(), 3);
}

#[test]
fn n144_index_events_and_query() {
    let mut indexer = NftIndexer::new();
    let events = vec![
        MarketplaceEvent::ListingCreated {
            token_id: ResourceId([1u8; 32]),
            seller: [10u8; 32],
            price: 100,
        },
        MarketplaceEvent::SaleCompleted {
            token_id: ResourceId([1u8; 32]),
            seller: [10u8; 32],
            buyer: [20u8; 32],
            price: 100,
        },
    ];
    indexer.index_marketplace_events(&events, 42);
    let token_events = indexer.get_events_by_token(&[1u8; 32]);
    assert_eq!(token_events.len(), 2);
    assert_eq!(token_events[0].event_type, "ListingCreated");
    assert_eq!(token_events[1].event_type, "SaleCompleted");
}

#[test]
fn n144_deterministic_index_root() {
    let mut reg = ConstitutionalRegistry::new();
    reg.register(NftConstitutionalRecord {
        token_id: [7u8; 32],
        owner: [70u8; 32],
        collection_id: None,
        creator: [80u8; 32],
        mining_origin: Some("BugHunter".into()),
        royalty_policy: None,
        governance_right: None,
        bridge_lock: None,
    });

    let mut i1 = NftIndexer::new();
    let mut i2 = NftIndexer::new();
    i1.index_registry(&reg);
    i2.index_registry(&reg);
    assert_eq!(i1.compute_index_root(), i2.compute_index_root());
}

#[test]
fn n144_index_updates_after_registry_change() {
    let mut reg = ConstitutionalRegistry::new();
    let token = [9u8; 32];
    reg.register(NftConstitutionalRecord {
        token_id: token,
        owner: [10u8; 32],
        collection_id: None,
        creator: [30u8; 32],
        mining_origin: None,
        royalty_policy: None,
        governance_right: None,
        bridge_lock: None,
    });

    let mut indexer = NftIndexer::new();
    indexer.index_registry(&reg);
    let old_root = indexer.compute_index_root();

    let mut updated = reg.get(&token).unwrap().clone();
    updated.owner = [99u8; 32];
    reg.register(updated);

    indexer.index_registry(&reg);
    let new_root = indexer.compute_index_root();
    assert_ne!(old_root, new_root);
}
