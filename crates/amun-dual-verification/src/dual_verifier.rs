use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_invariant_engine::invariant_types::InvariantDeclaration;
use amun_pccv::pccv_verifier::{PCCVResult, PCCVVerifier};
use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;
use amun_resource_core::ResourceRegistry;
use amun_vm_kernel::execution_context::ExecutionContext;

pub struct DualVerifier;

impl DualVerifier {
    pub fn execute_and_verify(
        program: &ConstitutionalProgram,
        ctx: &ExecutionContext,
        registry: &mut ResourceRegistry,
        invariants: &[InvariantDeclaration],
    ) -> Result<bool, String> {
        let mut hot_store = HotProofStore::new(1000);
        let mut archive = ProofArchive::new();
        let pre_root = registry.compute_state_root();

        let result = ConstitutionalRuntime::execute(
            program,
            ctx,
            registry,
            invariants,
            100_000,
            &mut hot_store,
            &mut archive,
        )
        .map_err(|e| format!("Execution error: {}", e))?;

        match result {
            PipelineResult::Committed {
                post_state_root,
                transition_proof,
                pccv_verified,
            } => {
                // Build enhanced proof for PCCV
                let enhanced_proof = PCCVEngine::build_proof(
                    &amun_vm_kernel::pending_buffer::PendingBuffer::new(vec![]),
                    registry,
                    ctx.contract_id,
                    ctx.block_height,
                    ctx.block_hash,
                    ctx.transaction_hash,
                    pre_root,
                    post_state_root,
                    transition_proof.gas_used,
                );
                let pccv_result = PCCVVerifier::verify(&enhanced_proof, registry);
                Ok(pccv_verified && matches!(pccv_result, PCCVResult::Verified { .. }))
            }
            PipelineResult::Rejected { .. } => Ok(false),
        }
    }
}
