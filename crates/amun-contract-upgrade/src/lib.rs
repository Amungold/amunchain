use amun_resource_core::{ResourceId, ResourceRegistry, RegistryError};
use amun_bytecode::{ConstitutionalProgram, OpCode};
use amun_contract_registry::{ContractRegistry, ContractRecord};

pub struct UpgradeRecord {
    pub contract_id: ResourceId,
    pub old_code_hash: [u8; 32],
    pub new_code_hash: [u8; 32],
    pub upgrade_height: u64,
}

pub struct ContractUpgrader;

impl ContractUpgrader {
    pub fn upgrade(
        _registry: &mut ResourceRegistry,
        contract_registry: &mut ContractRegistry,
        contract_id: ResourceId,
        new_code: Vec<OpCode>,
        upgrade_height: u64,
    ) -> Result<UpgradeRecord, RegistryError> {
        // Fetch the existing contract
        let contract = contract_registry.get_contract(&contract_id)
            .ok_or(RegistryError::NotFound(contract_id))?;

        // Verify the new code
        let new_program = ConstitutionalProgram::new(1, 0, 0, new_code.clone());
        if !new_program.verify() {
            return Err(RegistryError::IllegalTransformation {
                src: amun_resource_core::ResourceArchetype::Asset,
                tgt: amun_resource_core::ResourceArchetype::Asset,
            });
        }

        let old_code_hash = contract.code_hash;
        
        // Update the contract directly in the registry
        contract_registry.contracts.insert(contract_id, ContractRecord {
            contract_id,
            owner: contract.owner,
            code_hash: new_program.program_hash,
            deployed_height: upgrade_height,
        });

        Ok(UpgradeRecord {
            contract_id,
            old_code_hash,
            new_code_hash: new_program.program_hash,
            upgrade_height,
        })
    }
}
