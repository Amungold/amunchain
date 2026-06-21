use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry,
};
use amun_contract_registry::ContractRegistry;
use amun_bytecode::OpCode;

#[test]
fn n169_contract_interacts_with_nft() {
    let mut reg = ResourceRegistry::new(100);
    let mut contract_reg = ContractRegistry::new();
    let owner = [10u8; 32];

    let contract_id = ResourceId([1u8; 32]);
    let code = vec![
        OpCode::Push(42),
        OpCode::Halt,
    ];
    contract_reg.deploy(&mut reg, contract_id, owner, code.clone(), 1).unwrap();

    let col_id = ResourceId([2u8; 32]);
    reg.register_genesis(ResourceMetadata {
        resource_id: col_id,
        archetype: ResourceArchetype::NFTCollection,
        state: ResourceState::Active,
        lineage: ResourceLineage::genesis(col_id),
        contract_id: [0u8; 32],
        owner,
    }).unwrap();

    let token_id = ResourceId([3u8; 32]);
    let parent_hash = reg.resource_hash(&col_id).unwrap();
    let version = reg.get(&col_id).unwrap().lineage.version + 1;
    reg.derive_from_collection(&col_id, ResourceMetadata {
        resource_id: token_id,
        archetype: ResourceArchetype::NFTAsset,
        state: ResourceState::Active,
        lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
        contract_id: contract_id.0,
        owner,
    }).unwrap();

    let nft = reg.get(&token_id).unwrap();
    assert_eq!(nft.contract_id, contract_id.0);
    assert_eq!(nft.archetype, ResourceArchetype::NFTAsset);
}

#[test]
fn n169_multiple_contracts_independent() {
    let mut reg = ResourceRegistry::new(100);
    let mut contract_reg = ContractRegistry::new();

    let code1 = vec![OpCode::Push(1), OpCode::Halt];
    let code2 = vec![OpCode::Push(2), OpCode::Halt];

    let id1 = contract_reg.deploy(&mut reg, ResourceId([10u8; 32]), [10u8; 32], code1, 1).unwrap();
    let id2 = contract_reg.deploy(&mut reg, ResourceId([20u8; 32]), [20u8; 32], code2, 1).unwrap();

    assert_ne!(id1, id2);
    assert!(contract_reg.get_contract(&id1).is_some());
    assert!(contract_reg.get_contract(&id2).is_some());
}

#[test]
fn n169_contract_registry_root_deterministic() {
    let mut reg1 = ResourceRegistry::new(100);
    let mut reg2 = ResourceRegistry::new(100);
    let mut cr1 = ContractRegistry::new();
    let mut cr2 = ContractRegistry::new();

    let code = vec![OpCode::Push(99), OpCode::Halt];
    cr1.deploy(&mut reg1, ResourceId([1u8; 32]), [10u8; 32], code.clone(), 1).unwrap();
    cr2.deploy(&mut reg2, ResourceId([1u8; 32]), [10u8; 32], code, 1).unwrap();

    assert_eq!(cr1.compute_registry_root(), cr2.compute_registry_root());
}
