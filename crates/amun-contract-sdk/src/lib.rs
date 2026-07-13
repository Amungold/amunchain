use amun_bytecode::OpCode;
use amun_contract_integration::ContractExecutor;
use amun_contract_registry::ContractRegistry;
use amun_resource_core::{RegistryError, ResourceArchetype, ResourceId, ResourceRegistry};

pub struct AmunContractSdk {
    pub registry: ResourceRegistry,
    pub contract_registry: ContractRegistry,
}

impl AmunContractSdk {
    pub fn new() -> Self {
        Self {
            registry: ResourceRegistry::new(1000),
            contract_registry: ContractRegistry::new(),
        }
    }

    pub fn deploy(
        &mut self,
        contract_id: [u8; 32],
        owner: [u8; 32],
        code: Vec<OpCode>,
        height: u64,
    ) -> Result<ResourceId, RegistryError> {
        let cid = ResourceId(contract_id);

        self.contract_registry
            .deploy(&mut self.registry, cid, owner, code, height)
            .map_err(|_| RegistryError::IllegalTransformation {
                src: ResourceArchetype::Asset,
                tgt: ResourceArchetype::Asset,
            })
    }

    pub fn call(
        &mut self,
        contract_id: [u8; 32],
        caller: [u8; 32],
        code: Vec<OpCode>,
        input: Vec<u8>,
        height: u64,
        gas_limit: u64,
    ) -> Result<Vec<u8>, String> {
        let cid = ResourceId(contract_id);

        ContractExecutor::call(
            &mut self.registry,
            cid,
            caller,
            code,
            input,
            height,
            [0u8; 32],
            gas_limit,
        )
    }

    pub fn compute_root(&self) -> [u8; 32] {
        self.contract_registry.compute_registry_root()
    }
}

impl Default for AmunContractSdk {
    fn default() -> Self {
        Self::new()
    }
}
