use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use amun_nft_explorer::ExplorerEngine;

#[test]
fn n135_query_collections() {
    let mut reg = ResourceRegistry::new(10);
    let col_id = ResourceId([1u8; 32]);
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [10u8; 32],
    }).unwrap();

    let collections = ExplorerEngine::get_collections(&reg);
    assert_eq!(collections.len(), 1);
    assert_eq!(collections[0].collection_id, col_id);
}

#[test]
fn n135_query_nft_by_id() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([2u8; 32]);
    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: [5u8; 32],
    }).unwrap();

    let nft = ExplorerEngine::get_nft(&reg, &token_id).unwrap();
    assert_eq!(nft.owner, [5u8; 32]);
}

#[test]
fn n135_query_owner_nfts() {
    let mut reg = ResourceRegistry::new(10);
    let owner = [7u8; 32];
    let id1 = ResourceId([1u8; 32]);
    let id2 = ResourceId([2u8; 32]);
    reg.register_genesis(ResourceMetadata {
        resource_id: id1,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(id1),
        contract_id: [0u8; 32],
        owner,
    }).unwrap();
    reg.register_genesis(ResourceMetadata {
        resource_id: id2,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(id2),
        contract_id: [0u8; 32],
        owner,
    }).unwrap();

    let owner_data = ExplorerEngine::get_owner_nfts(&reg, &owner);
    assert_eq!(owner_data.nft_count, 2);
}
