use amun_nft_bridge::BridgeLedger;
use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
use amun_nft_governance::GovernanceLedger;
use amun_nft_royalty_accounting::RoyaltyLedger;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};
use sha2::{Digest, Sha256};

fn unique_id(seed: u8) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update([seed]);
    hasher.finalize().into()
}

type SnapshotTestState = (
    ResourceRegistry,
    RoyaltyLedger,
    GovernanceLedger,
    BridgeLedger,
    ConstitutionalRegistry,
    [u8; 32],
    [u8; 32],
    [u8; 32],
    [u8; 32],
    [u8; 32],
);

fn build_state() -> SnapshotTestState {
    let mut reg = ResourceRegistry::new(1000);
    let mut royalty = RoyaltyLedger::new();
    let mut gov = GovernanceLedger::new();
    let mut bridge = BridgeLedger::new();
    let mut const_reg = ConstitutionalRegistry::new();

    let col_id = ResourceId(unique_id(0));
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner: [0u8; 32],
    })
    .unwrap();

    for i in 1..6u8 {
        let token = unique_id(i);
        let owner = unique_id(i + 100);
        let parent_hash = reg.resource_hash(&col_id).unwrap();
        let version = reg.get(&col_id).unwrap().lineage.version + 1;
        reg.derive_from_collection(
            &col_id,
            ResourceMetadata {
                resource_id: ResourceId(token),
                archetype: ResourceArchetype::NFTAsset,
                state: ResourceState::Active,
                lineage: ResourceLineage::single_ancestor(
                    ResourceId(token),
                    col_id,
                    parent_hash,
                    version,
                ),
                contract_id: [0u8; 32],
                owner,
            },
        )
        .unwrap();

        royalty.settle(&amun_nft_royalty::RoyaltyRecord {
            token_id: token,
            creator: owner,
            payer: [0u8; 32],
            sale_price: 100,
            royalty_amount: 10,
            block_height: i as u64,
        });

        gov.set_rights(amun_nft_governance::GovernanceRight {
            token_id: token,
            owner,
            can_propose: true,
            can_veto: false,
            voting_power: 10,
        });

        const_reg.register(NftConstitutionalRecord {
            token_id: token,
            owner,
            collection_id: Some(col_id.0),
            creator: owner,
            mining_origin: Some("Test".into()),
            royalty_policy: None,
            governance_right: None,
            bridge_lock: None,
        });
    }

    bridge.lock(amun_nft_bridge::BridgeLock {
        source_chain: 1,
        token_id: unique_id(1),
        owner: unique_id(101),
        destination_chain: 2,
        destination_owner: unique_id(200),
        lock_height: 100,
    });

    let state_root = reg.compute_state_root();
    let royalty_root = royalty.compute_accounting_root();
    let gov_root = gov.compute_governance_root();
    let bridge_root = bridge.compute_bridge_root();
    let const_root = const_reg.compute_constitutional_root();

    (
        reg,
        royalty,
        gov,
        bridge,
        const_reg,
        state_root,
        royalty_root,
        gov_root,
        bridge_root,
        const_root,
    )
}

#[test]
fn n151_all_roots_deterministic_after_rebuild() {
    let (_, _, _, _, _, sr1, rr1, gr1, br1, cr1) = build_state();
    let (_, _, _, _, _, sr2, rr2, gr2, br2, cr2) = build_state();

    assert_eq!(sr1, sr2);
    assert_eq!(rr1, rr2);
    assert_eq!(gr1, gr2);
    assert_eq!(br1, br2);
    assert_eq!(cr1, cr2);
}

#[test]
fn n151_snapshot_roots_are_nonzero() {
    let (_, _, _, _, _, sr, rr, gr, br, cr) = build_state();

    assert_ne!(sr, [0u8; 32]);
    assert_ne!(rr, [0u8; 32]);
    assert_ne!(gr, [0u8; 32]);
    assert_ne!(br, [0u8; 32]);
    assert_ne!(cr, [0u8; 32]);
}

#[test]
fn n151_root_changes_after_mutation() {
    let (
        mut reg,
        mut royalty,
        mut gov,
        _bridge,
        mut const_reg,
        sr_before,
        rr_before,
        gr_before,
        _br_before,
        cr_before,
    ) = build_state();

    let new_token = unique_id(99);
    let new_owner = unique_id(199);
    let col_id = ResourceId(unique_id(0));
    let parent_hash = reg.resource_hash(&col_id).unwrap();
    let version = reg.get(&col_id).unwrap().lineage.version + 1;
    reg.derive_from_collection(
        &col_id,
        ResourceMetadata {
            resource_id: ResourceId(new_token),
            archetype: ResourceArchetype::NFTAsset,
            state: ResourceState::Active,
            lineage: ResourceLineage::single_ancestor(
                ResourceId(new_token),
                col_id,
                parent_hash,
                version,
            ),
            contract_id: [0u8; 32],
            owner: new_owner,
        },
    )
    .unwrap();

    royalty.settle(&amun_nft_royalty::RoyaltyRecord {
        token_id: new_token,
        creator: new_owner,
        payer: [0u8; 32],
        sale_price: 200,
        royalty_amount: 20,
        block_height: 99,
    });

    gov.set_rights(amun_nft_governance::GovernanceRight {
        token_id: new_token,
        owner: new_owner,
        can_propose: true,
        can_veto: true,
        voting_power: 99,
    });

    const_reg.register(NftConstitutionalRecord {
        token_id: new_token,
        owner: new_owner,
        collection_id: Some(col_id.0),
        creator: new_owner,
        mining_origin: Some("Mutation".into()),
        royalty_policy: None,
        governance_right: None,
        bridge_lock: None,
    });

    assert_ne!(sr_before, reg.compute_state_root());
    assert_ne!(rr_before, royalty.compute_accounting_root());
    assert_ne!(gr_before, gov.compute_governance_root());
    assert_ne!(cr_before, const_reg.compute_constitutional_root());
}
