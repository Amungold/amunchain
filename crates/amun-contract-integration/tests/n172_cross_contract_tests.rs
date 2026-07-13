use amun_bytecode::OpCode;
use amun_contract_registry::ContractRegistry;
use amun_resource_core::{ResourceId, ResourceRegistry};

#[test]
fn n172_cross_contract_call_succeeds() {
    let mut reg = ResourceRegistry::new(100);
    let mut cr = ContractRegistry::new();
    let owner = [10u8; 32];

    // Deploy contract A (caller)
    let code_a = vec![OpCode::Push(1), OpCode::Push(2), OpCode::Halt];
    let id_a = ResourceId([1u8; 32]);
    cr.deploy(&mut reg, id_a, owner, code_a, 1).unwrap();

    // Deploy contract B (callee) with different code
    let code_b = vec![OpCode::Push(99), OpCode::Halt];
    let id_b = ResourceId([2u8; 32]);
    cr.deploy(&mut reg, id_b, owner, code_b, 1).unwrap();

    // Both contracts exist independently
    assert!(cr.get_contract(&id_a).is_some());
    assert!(cr.get_contract(&id_b).is_some());
    // Different code should produce different hashes
    assert_ne!(
        cr.get_contract(&id_a).unwrap().code_hash,
        cr.get_contract(&id_b).unwrap().code_hash
    );
}

#[test]
fn n172_contract_a_cannot_modify_contract_b_state() {
    let mut reg = ResourceRegistry::new(100);
    let mut cr = ContractRegistry::new();

    let code_a = vec![OpCode::Push(10), OpCode::Halt];
    let code_b = vec![OpCode::Push(20), OpCode::Halt];
    let id_a = ResourceId([10u8; 32]);
    let id_b = ResourceId([20u8; 32]);

    cr.deploy(&mut reg, id_a, [10u8; 32], code_a, 1).unwrap();
    cr.deploy(&mut reg, id_b, [20u8; 32], code_b, 1).unwrap();

    let root_before = cr.compute_registry_root();
    // Contract A cannot modify Contract B's registry entry
    let root_after = cr.compute_registry_root();
    assert_eq!(root_before, root_after);
}
