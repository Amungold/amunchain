use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

use amun_bytecode::OpCode;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
    ResourceState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractId(pub [u8; 32]);

#[derive(Debug, Clone)]
pub struct ContractRecord {
    pub contract_id: ResourceId,
    pub owner: [u8; 32],
    pub code_hash: [u8; 32],
    pub deployed_height: u64,
}

pub struct ContractRegistry {
    contracts: BTreeMap<ContractId, ContractRecord>,
}

impl Default for ContractRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractRegistry {
    pub fn new() -> Self {
        Self {
            contracts: BTreeMap::new(),
        }
    }

    pub fn deploy(
        &mut self,
        registry: &mut ResourceRegistry,
        contract_id: ResourceId,
        owner: [u8; 32],
        code: Vec<OpCode>,
        deployed_height: u64,
    ) -> Result<ResourceId, String> {
        let meta = ResourceMetadata {
            resource_id: contract_id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(contract_id),
            contract_id: [0u8; 32],
            owner,
        };

        registry
            .register_genesis(meta)
            .map_err(|e| format!("{:?}", e))?;

        let mut hasher = Sha256::new();

        for op in &code {
            hasher.update(format!("{:?}", op).as_bytes());
        }

        let code_hash: [u8; 32] = hasher.finalize().into();

        self.contracts.insert(
            ContractId(contract_id.0),
            ContractRecord {
                contract_id,
                owner,
                code_hash,
                deployed_height,
            },
        );

        Ok(contract_id)
    }

    pub fn get_contract(&self, id: &ResourceId) -> Option<&ContractRecord> {
        self.contracts.get(&ContractId(id.0))
    }

    pub fn compute_registry_root(&self) -> [u8; 32] {
        self.state_root()
    }

    pub fn state_root(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        for (id, record) in &self.contracts {
            hasher.update(id.0);
            hasher.update(record.owner);
            hasher.update(record.code_hash);
            hasher.update(record.deployed_height.to_le_bytes());
        }

        hasher.finalize().into()
    }

    pub fn len(&self) -> usize {
        self.contracts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }
}

impl ContractRegistry {
    pub fn update_contract(
        &mut self,
        contract_id: ResourceId,
        owner: [u8; 32],
        code_hash: [u8; 32],
        deployed_height: u64,
    ) {
        self.contracts.insert(
            ContractId(contract_id.0),
            ContractRecord {
                contract_id,
                owner,
                code_hash,
                deployed_height,
            },
        );
    }
}
