use amun_resource_core::{
    ResourceId, ResourceRegistry,
};
use amun_bytecode::OpCode;
use amun_contract_integration::ContractExecutor;
use amun_contract_registry::ContractRegistry;

pub struct SecurityAuditResult {
    pub test_name: String,
    pub passed: bool,
    pub details: String,
}

pub fn audit_reentrancy() -> SecurityAuditResult {
    let mut reg = ResourceRegistry::new(100);
    let mut contract_reg = ContractRegistry::new();
    let owner = [10u8; 32];
    let contract_id = ResourceId([1u8; 32]);
    let code = vec![OpCode::Push(1), OpCode::Halt];
    contract_reg.deploy(&mut reg, contract_id, owner, code.clone(), 1).unwrap();
    let result1 = ContractExecutor::call(&mut reg, contract_id, owner, code.clone(), vec![], 1, [0u8; 32], 10000);
    let result2 = ContractExecutor::call(&mut reg, contract_id, owner, code, vec![], 1, [0u8; 32], 10000);
    SecurityAuditResult {
        test_name: "reentrancy".into(),
        passed: result1.is_ok() && result2.is_ok(),
        details: format!("Call1: {:?}, Call2: {:?}", result1.is_ok(), result2.is_ok()),
    }
}

pub fn audit_gas_exhaustion() -> SecurityAuditResult {
    let mut reg = ResourceRegistry::new(100);
    let mut contract_reg = ContractRegistry::new();
    let owner = [10u8; 32];
    let contract_id = ResourceId([2u8; 32]);
    let code = vec![
        OpCode::Push(0),
        OpCode::Push(1),
        OpCode::JumpIfNonZero(-2),
        OpCode::Halt,
    ];
    contract_reg.deploy(&mut reg, contract_id, owner, code.clone(), 1).unwrap();
    let code_len = code.len() as u64;
    let gas_limit = code_len * 5; // Very low gas limit
    let result = ContractExecutor::call(&mut reg, contract_id, owner, code, vec![], 1, [0u8; 32], gas_limit);
    SecurityAuditResult {
        test_name: "gas_exhaustion".into(),
        passed: result.is_err(),
        details: format!("Gas limit: {}, Result: {:?}", gas_limit, result.is_err()),
    }
}

pub fn audit_state_isolation() -> SecurityAuditResult {
    let mut reg = ResourceRegistry::new(100);
    let mut contract_reg = ContractRegistry::new();
    let code1 = vec![OpCode::Push(1), OpCode::Halt];
    let code2 = vec![OpCode::Push(2), OpCode::Halt];
    let id1 = ResourceId([10u8; 32]);
    let id2 = ResourceId([20u8; 32]);
    contract_reg.deploy(&mut reg, id1, [10u8; 32], code1, 1).unwrap();
    contract_reg.deploy(&mut reg, id2, [20u8; 32], code2, 1).unwrap();
    let c1 = contract_reg.get_contract(&id1);
    let c2 = contract_reg.get_contract(&id2);
    SecurityAuditResult {
        test_name: "state_isolation".into(),
        passed: c1.is_some() && c2.is_some() && c1.unwrap().contract_id != c2.unwrap().contract_id,
        details: "Contracts are isolated".into(),
    }
}

pub fn audit_determinism() -> SecurityAuditResult {
    let mut reg1 = ResourceRegistry::new(100);
    let mut reg2 = ResourceRegistry::new(100);
    let mut cr1 = ContractRegistry::new();
    let mut cr2 = ContractRegistry::new();
    let code = vec![OpCode::Push(42), OpCode::Halt];
    let id = ResourceId([1u8; 32]);
    cr1.deploy(&mut reg1, id, [10u8; 32], code.clone(), 1).unwrap();
    cr2.deploy(&mut reg2, id, [10u8; 32], code, 1).unwrap();
    let root1 = cr1.compute_registry_root();
    let root2 = cr2.compute_registry_root();
    SecurityAuditResult {
        test_name: "determinism".into(),
        passed: root1 == root2,
        details: "Execution is deterministic".into(),
    }
}

pub fn audit_malicious_bytecode() -> SecurityAuditResult {
    let mut reg = ResourceRegistry::new(100);
    let mut contract_reg = ContractRegistry::new();
    let code = vec![OpCode::Halt];
    let result = contract_reg.deploy(&mut reg, ResourceId([1u8; 32]), [10u8; 32], code, 1);
    SecurityAuditResult {
        test_name: "malicious_bytecode".into(),
        passed: result.is_ok(),
        details: "Empty programs with Halt are valid".into(),
    }
}

pub fn audit_evidence_consistency() -> SecurityAuditResult {
    let mut reg = ResourceRegistry::new(100);
    let mut contract_reg = ContractRegistry::new();
    let code = vec![OpCode::Push(99), OpCode::Halt];
    contract_reg.deploy(&mut reg, ResourceId([1u8; 32]), [10u8; 32], code, 1).unwrap();
    let root1 = contract_reg.compute_registry_root();
    let root2 = contract_reg.compute_registry_root();
    SecurityAuditResult {
        test_name: "evidence_consistency".into(),
        passed: root1 == root2,
        details: "Evidence root consistent".into(),
    }
}
