use amun_bytecode::OpCode;
use amun_contract_registry::ContractRegistry;
use amun_contract_upgrade::ContractUpgrader;
use amun_resource_core::ResourceId;
use amun_resource_core::ResourceRegistry;

#[test]
fn n174_upgrade_contract_success() {
    let mut reg = ResourceRegistry::new(100);
    let mut cr = ContractRegistry::new();
    let owner = [10u8; 32];
    let contract_id = ResourceId([1u8; 32]);

    let old_code = vec![OpCode::Push(1), OpCode::Halt];
    let new_code = vec![OpCode::Push(1), OpCode::Push(2), OpCode::Halt];

    cr.deploy(&mut reg, contract_id, owner, old_code, 1)
        .unwrap();

    // Capture data before mutable borrow
    let old_hash = cr.get_contract(&contract_id).unwrap().code_hash;
    let old_height = cr.get_contract(&contract_id).unwrap().deployed_height;

    let record = ContractUpgrader::upgrade(&mut reg, &mut cr, contract_id, new_code, 2).unwrap();

    // Capture data after upgrade
    let new_hash = cr.get_contract(&contract_id).unwrap().code_hash;
    let new_height = cr.get_contract(&contract_id).unwrap().deployed_height;

    assert_eq!(record.contract_id, contract_id);
    assert_eq!(record.old_code_hash, old_hash);
    assert_eq!(record.new_code_hash, new_hash);
    assert_eq!(record.upgrade_height, 2);
    assert!(
        old_height != new_height,
        "Contract should be updated after upgrade"
    );
}
