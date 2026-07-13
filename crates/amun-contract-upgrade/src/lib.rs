use amun_bytecode::{ConstitutionalProgram, OpCode};
use amun_contract_registry::ContractRegistry;
use amun_resource_core::{RegistryError, ResourceArchetype, ResourceId, ResourceRegistry};

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
        let (owner, old_code_hash) = {
            let contract = contract_registry
                .get_contract(&contract_id)
                .ok_or(RegistryError::NotFound(contract_id))?;

            (contract.owner, contract.code_hash)
        };

        let new_program = ConstitutionalProgram::new(1, 0, 0, new_code);

        if !new_program.verify() {
            return Err(RegistryError::IllegalTransformation {
                src: ResourceArchetype::Asset,
                tgt: ResourceArchetype::Asset,
            });
        }

        contract_registry.update_contract(
            contract_id,
            owner,
            new_program.program_hash,
            upgrade_height,
        );

        Ok(UpgradeRecord {
            contract_id,
            old_code_hash,
            new_code_hash: new_program.program_hash,
            upgrade_height,
        })
    }
}
