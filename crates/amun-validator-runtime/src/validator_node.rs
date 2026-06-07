use amun_resource_core::ResourceId;
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_bytecode::program::ConstitutionalProgram;
use amun_bytecode::opcodes::OpCode;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_replay_consensus::replay_backed_consensus::ReplayBackedConsensus;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_operations::metrics::NodeMetrics;
use amun_operations::health_check::NodeHealth;

pub struct ValidatorNode {
    pub node_id: ResourceId,
    pub store: PersistentValidatorStore,
    pub metrics: NodeMetrics,
    pub health: NodeHealth,
}

impl ValidatorNode {
    pub fn new(node_id: ResourceId, data_dir: &str) -> Result<Self, String> {
        let store = PersistentValidatorStore::open(data_dir)?;
        let health = NodeHealth::new({
            let mut h = [0u8; 32];
            h[0..8].copy_from_slice(&node_id.0[0..8]);
            h
        });
        Ok(Self { node_id, store, metrics: NodeMetrics::new(), health })
    }

    pub fn propose_block(&mut self, height: u64) -> Result<(), String> {
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: self.node_id,
            caller: [1u8; 32],
            block_height: height,
            block_hash: [0u8; 32],
            transaction_hash: {
                let mut h = [0u8; 32];
                h[0..8].copy_from_slice(&height.to_le_bytes());
                h
            },
            pre_state_root: self.store.state_root(),
            authority: [2u8; 32],
        };

        let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
        let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();

        let result = ConstitutionalRuntime::execute(
            &program, &ctx, self.store.registry_mut(), &[], 100_000,
            &mut hot, &mut archive,
        ).map_err(|e| format!("Execution error: {}", e))?;

        match result {
            PipelineResult::Committed { .. } => {
                self.store.advance(height, [0u8; 32], [0x10; 32], vec![])?;
                self.store.save()?;
                self.metrics.record_block_produced();
                self.metrics.record_transaction();
                self.metrics.record_proof_generated();
                self.metrics.record_replay();
                self.health.current_height = height;
                self.health.is_synced = true;
                Ok(())
            }
            _ => Err("Block proposal failed".into()),
        }
    }
}
