use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceArchetype, ResourceState,
    ResourceLineage, ResourceRegistry, RegistryError,
};
use amun_bytecode::{ConstitutionalProgram, OpCode};
use std::collections::BTreeMap;

pub struct ContractRecord {
    pub contract_id: ResourceId,
    pub owner: [u8; 32],
    pub code_hash: [u8; 32],
    pub deployed_height: u64,
}

pub struct ContractRegistry {
    pub contracts: BTreeMap<ResourceId, ContractRecord>,
}

impl ContractRegistry {
    pub fn new() -> Self {
        Self { contracts: BTreeMap::new() }
    }

    pub fn deploy(
        &mut self,
        registry: &mut ResourceRegistry,
        contract_id: ResourceId,
        owner: [u8; 32],
        code: Vec<OpCode>,
        deployed_height: u64,
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

        self.contracts.insert(contract_id, ContractRecord {
            contract_id,
            owner,
            code_hash: program.program_hash,
            deployed_height,
        });

        Ok(contract_id)
    }

    pub fn get_contract(&self, contract_id: &ResourceId) -> Option<&ContractRecord> {
        self.contracts.get(contract_id)
    }

    pub fn compute_registry_root(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"AMUN_CONTRACT_REGISTRY_V1");
        for (id, record) in &self.contracts {
            hasher.update(&id.0);
            hasher.update(&record.owner);
            hasher.update(&record.code_hash);
            hasher.update(&record.deployed_height.to_le_bytes());
        }
        hasher.finalize().into()
    }
}
