use amun_resource_core::{
    ResourceId, ResourceRegistry,
};
use amun_bytecode::OpCode;
use amun_contract_registry::ContractRegistry;
use amun_contract_integration::ContractExecutor;
use rand::Rng;

pub struct FuzzResult {
    pub iterations: u64,
    pub successful_deploys: u64,
    pub failed_deploys: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub gas_exhaustions: u64,
    pub evidence_mismatches: u64,
}

impl FuzzResult {
    pub fn new() -> Self {
        Self {
            iterations: 0,
            successful_deploys: 0,
            failed_deploys: 0,
            successful_calls: 0,
            failed_calls: 0,
            gas_exhaustions: 0,
            evidence_mismatches: 0,
        }
    }

    pub fn passed(&self) -> bool {
        self.evidence_mismatches == 0
    }
}

fn random_opcode(rng: &mut impl Rng) -> OpCode {
    match rng.gen_range(0..6) {
        0 => OpCode::Push(rng.gen::<u64>()),
        1 => OpCode::Pop,
        2 => OpCode::Dup(rng.gen::<u32>()),
        3 => OpCode::Swap(rng.gen::<u32>()),
        4 => OpCode::Return,
        _ => OpCode::Halt,
    }
}

fn random_program(rng: &mut impl Rng) -> Vec<OpCode> {
    let len = rng.gen_range(1..10);
    let mut code = Vec::with_capacity(len);
    for _ in 0..len - 1 {
        code.push(random_opcode(rng));
    }
    code.push(OpCode::Halt);
    code
}

pub fn fuzz_contract_deploy(iterations: u64) -> FuzzResult {
    let mut result = FuzzResult::new();
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(100);
        let mut cr = ContractRegistry::new();
        let code = random_program(&mut rng);
        let cid = ResourceId(rng.gen::<[u8; 32]>());
        let owner = rng.gen::<[u8; 32]>();

        match cr.deploy(&mut reg, cid, owner, code.clone(), 1) {
            Ok(_) => result.successful_deploys += 1,
            Err(_) => result.failed_deploys += 1,
        }

        // Verify evidence consistency
        let root1 = cr.compute_registry_root();
        let root2 = cr.compute_registry_root();
        if root1 != root2 {
            result.evidence_mismatches += 1;
        }

        result.iterations += 1;
    }
    result
}

pub fn fuzz_contract_call(iterations: u64) -> FuzzResult {
    let mut result = FuzzResult::new();
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(100);
        let mut cr = ContractRegistry::new();
        let code = vec![OpCode::Push(42), OpCode::Halt];
        let cid = ResourceId(rng.gen::<[u8; 32]>());
        let owner = rng.gen::<[u8; 32]>();

        if cr.deploy(&mut reg, cid, owner, code.clone(), 1).is_ok() {
            let gas_limit = rng.gen_range(1..1000);
            match ContractExecutor::call(
                &mut reg, cid, owner, code, vec![], 1, [0u8; 32], gas_limit,
            ) {
                Ok(_) => result.successful_calls += 1,
                Err(e) => {
                    result.failed_calls += 1;
                    if e.contains("Out of gas") || e.contains("gas") {
                        result.gas_exhaustions += 1;
                    }
                }
            }
        }

        let root1 = cr.compute_registry_root();
        let root2 = cr.compute_registry_root();
        if root1 != root2 {
            result.evidence_mismatches += 1;
        }

        result.iterations += 1;
    }
    result
}

pub fn fuzz_gas_limits(iterations: u64) -> FuzzResult {
    let mut result = FuzzResult::new();
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let mut reg = ResourceRegistry::new(100);
        let mut cr = ContractRegistry::new();
        let code = vec![
            OpCode::Push(0),
            OpCode::Push(1),
            OpCode::JumpIfNonZero(-2),
            OpCode::Halt,
        ];
        let cid = ResourceId(rng.gen::<[u8; 32]>());
        let owner = rng.gen::<[u8; 32]>();

        if cr.deploy(&mut reg, cid, owner, code.clone(), 1).is_ok() {
            let gas_limit = rng.gen_range(1..500);
            match ContractExecutor::call(
                &mut reg, cid, owner, code, vec![], 1, [0u8; 32], gas_limit,
            ) {
                Ok(_) => result.successful_calls += 1,
                Err(e) => {
                    result.failed_calls += 1;
                    if e.contains("Out of gas") || e.contains("gas") {
                        result.gas_exhaustions += 1;
                    }
                }
            }
        }

        let root1 = cr.compute_registry_root();
        let root2 = cr.compute_registry_root();
        if root1 != root2 {
            result.evidence_mismatches += 1;
        }

        result.iterations += 1;
    }
    result
}
