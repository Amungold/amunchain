use amun_resource_core::{ResourceId, ResourceRegistry};
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;

use crate::consensus_types::{ConstitutionalBlock, ConstitutionalFinalityCertificate, ConstitutionalQC};

pub struct ConsensusIntegrator;

impl ConsensusIntegrator {
    pub fn execute_block(
        programs: &[(ConstitutionalProgram, ExecutionContext)],
        registry: &mut ResourceRegistry,
        block_height: u64,
        parent_hash: [u8; 32],
        proposer: ResourceId,
    ) -> Result<ConstitutionalBlock, String> {
        let mut hot = HotProofStore::new(10000);
        let mut archive = ProofArchive::new();
        let mut transitions = Vec::new();

        for (program, ctx) in programs {
            let result = ConstitutionalRuntime::execute(
                program, ctx, registry, &[], 100_000,
                &mut hot, &mut archive,
            ).map_err(|e| format!("Execution error: {}", e))?;

            match result {
                PipelineResult::Committed { transition_proof, .. }
                | PipelineResult::Rejected { transition_proof, .. } => {
                    transitions.push(transition_proof);
                }
            }
        }

        let state_root = registry.compute_state_root();
        let mut block = ConstitutionalBlock {
            block_height,
            block_hash: [0u8; 32],
            parent_hash,
            state_root,
            proof_root: [0u8; 32],
            transitions,
            proposer,
            timestamp: 0,
        };
        block.proof_root = block.compute_proof_root();

        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_BLOCK_V1");
        hasher.update(&block.block_height.to_le_bytes());
        hasher.update(&block.parent_hash);
        hasher.update(&block.state_root);
        hasher.update(&block.proof_root);
        let hash = hasher.finalize();
        let hash_bytes = hash.as_bytes();
        let mut bh = [0u8; 32];
        bh.copy_from_slice(hash_bytes);
        block.block_hash = bh;

        Ok(block)
    }

    pub fn form_consensus(
        block: &ConstitutionalBlock,
        quorum_size: usize,
        signatures: Vec<Vec<u8>>,
    ) -> Result<ConstitutionalFinalityCertificate, String> {
        if !block.verify_all_proofs() {
            return Err("Block proof verification failed".into());
        }
        let mut qc = ConstitutionalQC::for_block(block, quorum_size);
        for sig in signatures {
            qc.add_signature(sig);
        }
        if !qc.is_valid() {
            return Err(format!("Insufficient signatures: {}/{}", qc.signer_count, quorum_size));
        }
        Ok(ConstitutionalFinalityCertificate::issue(block, qc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::ResourceId;
    use amun_bytecode::opcodes::OpCode;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    #[test]
    fn w17_execute_block_with_proofs() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ConsensusIntegrator::execute_block(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        assert_eq!(block.transitions.len(), 1);
        assert!(block.verify_all_proofs());
        assert_ne!(block.proof_root, [0u8; 32]);
    }

    #[test]
    fn w17_form_consensus_with_quorum() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ConsensusIntegrator::execute_block(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        let sigs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 64]).collect();
        let cert = ConsensusIntegrator::form_consensus(&block, 5, sigs).unwrap();
        assert!(cert.verify());
        assert_eq!(cert.qc.signer_count, 5);
    }

    #[test]
    fn w17_reject_insufficient_quorum() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ConsensusIntegrator::execute_block(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        let sigs: Vec<Vec<u8>> = (0..2).map(|_| vec![0u8; 64]).collect();
        assert!(ConsensusIntegrator::form_consensus(&block, 5, sigs).is_err());
    }

    #[test]
    fn w17_finality_certificate_deterministic() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ConsensusIntegrator::execute_block(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        let sigs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 64]).collect();
        let cert1 = ConsensusIntegrator::form_consensus(&block, 5, sigs.clone()).unwrap();
        let cert2 = ConsensusIntegrator::form_consensus(&block, 5, sigs).unwrap();
        assert_eq!(cert1.certificate_hash, cert2.certificate_hash);
    }

    #[test]
    fn w17_block_links_proofs_to_state() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ConsensusIntegrator::execute_block(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        assert_eq!(block.state_root, registry.compute_state_root());
        assert_eq!(block.proof_root, block.compute_proof_root());
        assert!(!block.transitions.is_empty());
    }
}
