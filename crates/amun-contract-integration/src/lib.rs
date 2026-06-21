use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry, RegistryError,
};
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_bytecode::ConstitutionalProgram;
use amun_bytecode::OpCode;
use amun_gas_engine::GasEngine;

pub struct ContractDeployer;

impl ContractDeployer {
    pub fn deploy(
        registry: &mut ResourceRegistry,
        contract_id: ResourceId,
        code: Vec<OpCode>,
        owner: [u8; 32],
    ) -> Result<ResourceId, RegistryError> {
        let program = ConstitutionalProgram::new(1, 0, 0, code);
        if !program.verify() {
            return Err(RegistryError::IllegalTransformation {
                src: ResourceArchetype::Asset,
                tgt: ResourceArchetype::Asset,
            });
        }

        let meta = ResourceMetadata {
            resource_id: contract_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(contract_id),
            contract_id: [0u8; 32],
            owner,
        };
        registry.register_genesis(meta)?;
        Ok(contract_id)
    }
}

pub struct ContractExecutor;

impl ContractExecutor {
    pub fn call(
        registry: &mut ResourceRegistry,
        contract_id: ResourceId,
        _caller: [u8; 32],
        code: Vec<OpCode>,
        _input: Vec<u8>,
        block_height: u64,
        block_hash: [u8; 32],
        gas_limit: u64,
    ) -> Result<Vec<u8>, String> {
        // Verify contract exists
        registry.get(&contract_id)
            .ok_or("Contract not found")?;

        let code_len = code.len() as u64;

        // Create a ConstitutionalProgram for execution
        let program = ConstitutionalProgram::new(1, 0, 0, code);
        if !program.verify() {
            return Err("Invalid program".into());
        }

        // Use GasEngine to meter the execution (without actual WASM execution for now)
        let (result, _evidence) = GasEngine::execute_with_gas(
            gas_limit,
            contract_id,
            block_height,
            [0u8; 32],
            |gas_meter| {
                let _ctx = ExecutionContext {
                    contract_id,
                    caller: _caller,
                    block_height,
                    block_hash,
                    transaction_hash: [0u8; 32],
                    pre_state_root: registry.compute_state_root(),
                    authority: _caller,
                };
                // Simulate gas consumption based on code size
                gas_meter.charge(code_len * 10)?;
                Ok(())
            },
        );

        match result {
            amun_gas_engine::GasEngineResult::Success { .. } => Ok(vec![]),
            amun_gas_engine::GasEngineResult::OutOfGas { .. } => Err("Out of gas".into()),
        }
    }

    pub fn compute_contract_evidence_root(registry: &ResourceRegistry) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_CONTRACT_EVIDENCE_V1");
        hasher.update(&registry.compute_state_root());
        hasher.finalize().into()
    }
}
