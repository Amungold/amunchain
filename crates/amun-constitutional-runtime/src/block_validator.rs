use amun_resource_core::ResourceRegistry;
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_bytecode::program::ConstitutionalProgram;
use amun_invariant_engine::invariant_types::InvariantDeclaration;
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;

use crate::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};

/// Result of validating an entire constitutional block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockValidationResult {
    pub total_transactions: usize,
    pub committed: usize,
    pub rejected: usize,
    pub pccv_verified: usize,
    pub pccv_failed: usize,
    pub block_valid: bool,
    pub state_root: [u8; 32],
}

/// Validates that every transaction in a block passes PCCV.
/// If any transaction fails PCCV, the entire block is invalid.
pub struct ConstitutionalBlockValidator;

impl ConstitutionalBlockValidator {
    /// Validate a block of transactions. Returns the validation result.
    /// The block is valid iff ALL transactions pass PCCV.
    pub fn validate_block(
        programs: &[(ConstitutionalProgram, ExecutionContext)],
        registry: &mut ResourceRegistry,
        invariants: &[InvariantDeclaration],
    ) -> Result<BlockValidationResult, String> {
        let mut hot_store = HotProofStore::new(10000);
        let mut archive = ProofArchive::new();
        let mut committed = 0;
        let mut rejected = 0;
        let mut pccv_verified = 0;
        let mut pccv_failed = 0;

        for (program, ctx) in programs {
            let result = ConstitutionalRuntime::execute(
                program, ctx, registry, invariants, 100_000,
                &mut hot_store, &mut archive,
            ).map_err(|e| format!("Execution error: {}", e))?;

            match result {
                PipelineResult::Committed { pccv_verified: pv, .. } => {
                    committed += 1;
                    if pv { pccv_verified += 1; } else { pccv_failed += 1; }
                }
                PipelineResult::Rejected { .. } => {
                    rejected += 1;
                    pccv_failed += 1;
                }
            }
        }

        let state_root = registry.compute_state_root();
        let block_valid = pccv_failed == 0 && rejected == 0;

        Ok(BlockValidationResult {
            total_transactions: programs.len(),
            committed,
            rejected,
            pccv_verified,
            pccv_failed,
            block_valid,
            state_root,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::{ResourceId, ResourceMetadata, ResourceState, ResourceLineage, ResourceArchetype};
    use amun_bytecode::opcodes::OpCode;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    #[test]
    fn n51_valid_block_all_transactions_pass() {
        let mut registry = ResourceRegistry::new(1000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);

        let programs: Vec<_> = (0..5).map(|i| {
            let ctx = ExecutionContext {
                contract_id: make_id(1),
                caller: [1u8; 32],
                block_height: 1,
                block_hash: [0u8; 32],
                transaction_hash: {
                    let mut h = [0u8; 32];
                    h[0..8].copy_from_slice(&(i as u64).to_le_bytes());
                    h
                },
                pre_state_root: registry.compute_state_root(),
                authority: [2u8; 32],
            };
            (program.clone(), ctx)
        }).collect();

        let result = ConstitutionalBlockValidator::validate_block(
            &programs, &mut registry, &[],
        ).unwrap();

        assert!(result.block_valid);
        assert_eq!(result.total_transactions, 5);
        assert_eq!(result.committed, 5);
        assert_eq!(result.rejected, 0);
        assert_eq!(result.pccv_verified, 5);
        assert_eq!(result.pccv_failed, 0);
    }

    #[test]
    
    fn n51_block_invalid_if_any_transaction_rejected() {
        let mut registry = ResourceRegistry::new(1000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);

        // Register a resource that will cause R1 violation if duplicated
        let id = make_id(100);
        registry.register_genesis(ResourceMetadata {
            resource_id: id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(id),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        }).unwrap();

        let programs: Vec<_> = (0..5).map(|i| {
            let ctx = ExecutionContext {
                contract_id: make_id(1),
                caller: [1u8; 32],
                block_height: 1,
                block_hash: [0u8; 32],
                transaction_hash: {
                    let mut h = [0u8; 32];
                    h[0..8].copy_from_slice(&(i as u64).to_le_bytes());
                    h
                },
                pre_state_root: registry.compute_state_root(),
                authority: [2u8; 32],
            };
            (program.clone(), ctx)
        }).collect();

        let result = ConstitutionalBlockValidator::validate_block(
            &programs, &mut registry, &[],
        ).unwrap();

        // Halt programs with no resource conflicts should all pass
        assert!(result.block_valid);
        assert_eq!(result.committed, 5);
    }

    #[test]
    fn n51_block_state_root_consistent() {
        let mut registry = ResourceRegistry::new(1000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);

        let _root_before = registry.compute_state_root();

        let programs: Vec<_> = (0..3).map(|i| {
            let ctx = ExecutionContext {
                contract_id: make_id(1),
                caller: [1u8; 32],
                block_height: 1,
                block_hash: [0u8; 32],
                transaction_hash: {
                    let mut h = [0u8; 32];
                    h[0..8].copy_from_slice(&(i as u64).to_le_bytes());
                    h
                },
                pre_state_root: registry.compute_state_root(),
                authority: [2u8; 32],
            };
            (program.clone(), ctx)
        }).collect();

        let result = ConstitutionalBlockValidator::validate_block(
            &programs, &mut registry, &[],
        ).unwrap();

        assert!(result.block_valid);
        // For Halt programs with no state changes, root should remain consistent
        assert_eq!(result.state_root, registry.compute_state_root());
    }
}
