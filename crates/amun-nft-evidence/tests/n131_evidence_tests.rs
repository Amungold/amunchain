use amun_nft_core::{NftEvent, NftEvidence};
use amun_nft_evidence::{
    accumulate_nft_evidence_root, CekError, MintVerificationContext, NftEvidenceKernel,
};
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};

#[test]
fn n131_mint_produces_valid_evidence() {
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

    let token_id = ResourceId([2u8; 32]);
    let owner = [5u8; 32];
    let metadata_hash = [3u8; 32];
    let timestamp = 1000;

    let ctx = MintVerificationContext {
        registry: &reg,
        collection_id: &col_id,
        token_id: &token_id,
        owner: &owner,
        metadata_hash: &metadata_hash,
        actual_metadata_hash: &metadata_hash,
        timestamp,
        last_event_time: 0,
    };
    assert!(NftEvidenceKernel::verify_mint(ctx).is_ok());

    let event = NftEvent::Mint {
        collection_id: col_id,
        token_id,
        owner,
        metadata_hash,
    };
    let evidence = NftEvidenceKernel::generate_evidence(event, timestamp, 1).unwrap();

    assert_eq!(evidence.timestamp, timestamp);
    assert_eq!(evidence.block_height, 1);
    assert_ne!(evidence.evidence_hash, [0u8; 32]);
}

#[test]
fn n131_law1_prevents_unauthorized_transfer() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([2u8; 32]);
    let real_owner = [5u8; 32];
    let thief = [9u8; 32];

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: real_owner,
    })
    .unwrap();

    let result = NftEvidenceKernel::verify_transfer(&reg, &token_id, &thief, 2000, 1000);
    assert_eq!(result, Err(CekError::Law1InvalidOwnership));
}

#[test]
fn n131_law2_prevents_duplicate_mint() {
    let mut reg = ResourceRegistry::new(10);
    let token_id = ResourceId([2u8; 32]);

    reg.register_genesis(ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(token_id),
        contract_id: [0u8; 32],
        owner: [1u8; 32],
    })
    .unwrap();

    let result = NftEvidenceKernel::verify_non_duplicate(&reg, &token_id);
    assert_eq!(result, Err(CekError::Law2DuplicateToken));
}

#[test]
fn n131_law3_rejects_invalid_metadata_hash() {
    let result = NftEvidenceKernel::verify_metadata_hash(&[1u8; 32], &[2u8; 32]);
    assert_eq!(result, Err(CekError::Law3InvalidMetadataHash));
}

#[test]
fn n131_law4_replay_protection_rejects_old_timestamp() {
    let result = NftEvidenceKernel::verify_replay_protection(1000, 500);
    assert_eq!(result, Err(CekError::Law4ReplayDetected));
}

#[test]
fn n131_evidence_root_matches() {
    let event1 = NftEvent::Mint {
        collection_id: ResourceId([1u8; 32]),
        token_id: ResourceId([2u8; 32]),
        owner: [5u8; 32],
        metadata_hash: [3u8; 32],
    };
    let event2 = NftEvent::Transfer {
        token_id: ResourceId([2u8; 32]),
        from: [5u8; 32],
        to: [6u8; 32],
    };

    let ev1 = NftEvidence::new(event1, 1000, 1);
    let ev2 = NftEvidence::new(event2, 2000, 1);

    let root1 = accumulate_nft_evidence_root(&[ev1.clone(), ev2.clone()]);
    let root2 = accumulate_nft_evidence_root(&[ev1, ev2]);
    assert_eq!(root1, root2);
}

#[test]
fn n131_full_mint_flow_with_evidence() {
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

    let token_id = ResourceId([2u8; 32]);
    let owner = [5u8; 32];
    let metadata_hash = [3u8; 32];
    let timestamp = 1000;

    let ctx = MintVerificationContext {
        registry: &reg,
        collection_id: &col_id,
        token_id: &token_id,
        owner: &owner,
        metadata_hash: &metadata_hash,
        actual_metadata_hash: &metadata_hash,
        timestamp,
        last_event_time: 0,
    };
    assert!(NftEvidenceKernel::verify_mint(ctx).is_ok());

    let event = NftEvent::Mint {
        collection_id: col_id,
        token_id,
        owner,
        metadata_hash,
    };
    let evidence = NftEvidenceKernel::generate_evidence(event.clone(), timestamp, 42).unwrap();

    assert_eq!(evidence.block_height, 42);
    assert_ne!(evidence.evidence_hash, [0u8; 32]);

    let root = accumulate_nft_evidence_root(&[evidence]);
    assert_ne!(root, [0u8; 32]);
}
