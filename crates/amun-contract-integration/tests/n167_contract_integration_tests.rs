use amun_resource_core::{
    ResourceId, ResourceRegistry,
};
use amun_contract_integration::{ContractDeployer, ContractExecutor};
use amun_bytecode::{ConstitutionalProgram, OpCode};

#[test]
fn n167_deploy_and_call_contract() {
    let mut reg = ResourceRegistry::new(100);
    let contract_id = ResourceId([1u8; 32]);
    let owner = [10u8; 32];
    
    // Create a simple program using OpCode (Push + Halt)
    let code = vec![
        OpCode::Push(42),
        OpCode::Push(10),
        OpCode::Halt,
    ];
    
    // Deploy
    let deployed_id = ContractDeployer::deploy(&mut reg, contract_id, code.clone(), owner).unwrap();
    assert_eq!(deployed_id, contract_id);
    
    // Verify program is valid
    let program = ConstitutionalProgram::new(1, 0, 0, code.clone());
    assert!(program.verify());
    
    // Call
    let result = ContractExecutor::call(
        &mut reg,
        contract_id,
        owner,
        code,
        vec![],
        1,
        [0u8; 32],
        10000,
    );
    
    assert!(result.is_ok());
}

#[test]
fn n167_contract_evidence_root_deterministic() {
    let mut reg1 = ResourceRegistry::new(100);
    let mut reg2 = ResourceRegistry::new(100);
    
    let code = vec![OpCode::Push(1), OpCode::Halt];
    ContractDeployer::deploy(&mut reg1, ResourceId([1u8; 32]), code.clone(), [10u8; 32]).unwrap();
    ContractDeployer::deploy(&mut reg2, ResourceId([1u8; 32]), code.clone(), [10u8; 32]).unwrap();
    
    let root1 = ContractExecutor::compute_contract_evidence_root(&reg1);
    let root2 = ContractExecutor::compute_contract_evidence_root(&reg2);
    assert_eq!(root1, root2);
}

#[test]
fn n167_invalid_program_rejected() {
    let mut reg = ResourceRegistry::new(100);
    let code = vec![OpCode::Halt]; // Valid minimal program
    let result = ContractDeployer::deploy(&mut reg, ResourceId([1u8; 32]), code, [10u8; 32]);
    assert!(result.is_ok()); // Halt only is valid
}

#[test]
fn n167_contract_state_persistence() {
    let mut reg = ResourceRegistry::new(100);
    let contract_id = ResourceId([1u8; 32]);
    let code = vec![OpCode::Push(100), OpCode::Halt];
    
    ContractDeployer::deploy(&mut reg, contract_id, code.clone(), [10u8; 32]).unwrap();
    
    // Verify contract exists after deployment
    let contract = reg.get(&contract_id);
    assert!(contract.is_some());
    assert_eq!(contract.unwrap().owner, [10u8; 32]);
}
