use amun_nft_bridge::BridgeLedger;
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

fn build_full_state() -> (
    ResourceRegistry,
    RoyaltyLedger,
    GovernanceLedger,
    BridgeLedger,
) {
    let mut reg = ResourceRegistry::new(1000);
    let mut royalty = RoyaltyLedger::new();
    let mut gov = GovernanceLedger::new();
    let mut bridge = BridgeLedger::new();

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

    for i in 1..11u8 {
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
    }

    bridge.lock(amun_nft_bridge::BridgeLock {
        source_chain: 1,
        token_id: unique_id(1),
        owner: unique_id(101),
        destination_chain: 2,
        destination_owner: unique_id(200),
        lock_height: 100,
    });

    (reg, royalty, gov, bridge)
}

#[test]
fn n150_state_roots_survive_rebuild() {
    let (reg1, royalty1, gov1, bridge1) = build_full_state();

    let state_root_before = reg1.compute_state_root();
    let royalty_root_before = royalty1.compute_accounting_root();
    let gov_root_before = gov1.compute_governance_root();
    let bridge_root_before = bridge1.compute_bridge_root();

    let (reg2, royalty2, gov2, bridge2) = build_full_state();

    assert_eq!(state_root_before, reg2.compute_state_root());
    assert_eq!(royalty_root_before, royalty2.compute_accounting_root());
    assert_eq!(gov_root_before, gov2.compute_governance_root());
    assert_eq!(bridge_root_before, bridge2.compute_bridge_root());
}
