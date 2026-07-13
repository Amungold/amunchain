use amun_nft_mining::{evaluate_contribution, issue_mining_reward, ContributionType};
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};

#[test]
fn n133_validator_contribution_evaluates() {
    let contributor = [5u8; 32];
    let metadata = evaluate_contribution(&contributor, ContributionType::ValidatorOperation);
    assert!(metadata.is_some());
    let m = metadata.unwrap();
    assert!(m.constitutional_role == Some("Validator".into()));
}

#[test]
fn n133_mining_reward_creates_nft() {
    let mut reg = ResourceRegistry::new(10);
    let col_id = ResourceId([1u8; 32]);

    // Register a collection
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [10u8; 32],
    })
    .unwrap();

    let contributor = [5u8; 32];
    let reward_id = ResourceId([2u8; 32]);

    let nft_id = issue_mining_reward(
        &mut reg,
        reward_id,
        &contributor,
        &col_id,
        ContributionType::ValidatorOperation,
    )
    .unwrap();

    // Verify the reward was consumed and NFT is active
    let reward = reg.get(&reward_id).unwrap();
    assert!(matches!(reward.state, ResourceState::Consumed { .. }));

    let nft = reg.get(&nft_id).unwrap();
    assert_eq!(nft.archetype, ResourceArchetype::NFTAsset);
    assert_eq!(nft.owner, contributor);
}

#[test]
fn n133_multiple_contributions_get_different_nfts() {
    let mut reg = ResourceRegistry::new(10);
    let col_id = ResourceId([1u8; 32]);

    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [10u8; 32],
    })
    .unwrap();

    let contributor1 = [5u8; 32];
    let contributor2 = [6u8; 32];

    let id1 = issue_mining_reward(
        &mut reg,
        ResourceId([2u8; 32]),
        &contributor1,
        &col_id,
        ContributionType::BugDiscovery,
    )
    .unwrap();

    let id2 = issue_mining_reward(
        &mut reg,
        ResourceId([3u8; 32]),
        &contributor2,
        &col_id,
        ContributionType::CodeContribution,
    )
    .unwrap();

    assert_ne!(id1, id2);
    assert_eq!(reg.get(&id1).unwrap().owner, contributor1);
    assert_eq!(reg.get(&id2).unwrap().owner, contributor2);
}
