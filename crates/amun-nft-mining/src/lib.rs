use amun_nft_core::NftMetadata;
use amun_resource_core::{
    RegistryError, ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata,
    ResourceRegistry, ResourceState,
};

/// Types of contributions that earn NFT mining rewards.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContributionType {
    ValidatorOperation,
    StorageProvision,
    CodeContribution,
    BugDiscovery,
    GovernanceParticipation,
}

/// Evaluates a contribution and returns the reward token metadata if valid.
pub fn evaluate_contribution(
    _contributor: &[u8; 32],
    contribution: ContributionType,
) -> Option<NftMetadata> {
    match contribution {
        ContributionType::ValidatorOperation => Some(NftMetadata {
            name: "Validator NFT".into(),
            description: "Awarded for operating a validator node".into(),
            image_uri: "ipfs://validator-nft".into(),
            attributes: vec![],
            constitutional_role: Some("Validator".into()),
        }),
        ContributionType::StorageProvision => Some(NftMetadata {
            name: "Storage Provider NFT".into(),
            description: "Awarded for providing decentralized storage".into(),
            image_uri: "ipfs://storage-provider-nft".into(),
            attributes: vec![],
            constitutional_role: Some("StorageProvider".into()),
        }),
        ContributionType::CodeContribution => Some(NftMetadata {
            name: "Code Contributor NFT".into(),
            description: "Awarded for accepted code contributions".into(),
            image_uri: "ipfs://code-contributor-nft".into(),
            attributes: vec![],
            constitutional_role: Some("Contributor".into()),
        }),
        ContributionType::BugDiscovery => Some(NftMetadata {
            name: "Bug Hunter NFT".into(),
            description: "Awarded for discovering and reporting a critical bug".into(),
            image_uri: "ipfs://bug-hunter-nft".into(),
            attributes: vec![],
            constitutional_role: Some("BugHunter".into()),
        }),
        ContributionType::GovernanceParticipation => Some(NftMetadata {
            name: "Governance Participant NFT".into(),
            description: "Awarded for participating in constitutional governance".into(),
            image_uri: "ipfs://governance-participant-nft".into(),
            attributes: vec![],
            constitutional_role: Some("GovernanceParticipant".into()),
        }),
    }
}

/// Issues a MiningReward resource to a contributor.
pub fn issue_mining_reward(
    registry: &mut ResourceRegistry,
    reward_id: ResourceId,
    contributor: &[u8; 32],
    _collection_id: &ResourceId,
    _contribution: ContributionType,
) -> Result<ResourceId, RegistryError> {
    // Create genesis resource of type NFTMiningReward
    let meta = ResourceMetadata {
        resource_id: reward_id,
        archetype: ResourceArchetype::NFTMiningReward,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(reward_id),
        contract_id: [0u8; 32],
        owner: *contributor,
    };
    registry.register_genesis(meta)?;

    // Derive NFTAsset from the reward using the collection
    let parent_hash = registry.resource_hash(&reward_id)?;
    let parent_version = registry.get(&reward_id).unwrap().lineage.version;

    let nft_id = derive_nft_id(&reward_id);
    let child_meta = ResourceMetadata {
        resource_id: nft_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::single_ancestor(
            nft_id,
            reward_id,
            parent_hash,
            parent_version + 1,
        ),
        contract_id: [0u8; 32],
        owner: *contributor,
    };
    registry.consume_and_derive(&reward_id, child_meta)
}

fn derive_nft_id(reward_id: &ResourceId) -> ResourceId {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(reward_id.0);
    hasher.update(b"->nft");
    ResourceId(hasher.finalize().into())
}
