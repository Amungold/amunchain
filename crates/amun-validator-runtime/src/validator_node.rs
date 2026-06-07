use amun_resource_core::ResourceId;
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_bytecode::program::ConstitutionalProgram;
use amun_bytecode::opcodes::OpCode;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;
use amun_operations::metrics::NodeMetrics;
use amun_operations::health_check::NodeHealth;

pub struct ValidatorNode {
    pub node_id: ResourceId,
    pub store: PersistentValidatorStore,
    pub metrics: NodeMetrics,
    pub health: NodeHealth,
    pub hot_store: HotProofStore,
    pub archive: ProofArchive,
}

impl ValidatorNode {
    pub fn new(node_id: ResourceId, data_dir: &str) -> Result<Self, String> {
        let store = PersistentValidatorStore::open(data_dir)?;
        let health = NodeHealth::new({
            let mut h = [0u8; 32];
            h[0..8].copy_from_slice(&node_id.0[0..8]);
            h
        });
        Ok(Self {
            node_id,
            store,
            metrics: NodeMetrics::new(),
            health,
            hot_store: HotProofStore::new(1000),
            archive: ProofArchive::new(),
        })
    }

    pub fn current_height(&self) -> u64 {
        self.store.current_height()
    }

    pub fn state_root(&self) -> [u8; 32] {
        self.store.state_root()
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.store.save()
    }

    pub fn propose_block(&mut self, height: u64) -> Result<[u8; 32], String> {
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

        let result = ConstitutionalRuntime::execute(
            &program,
            &ctx,
            self.store.registry_mut(),
            &[],
            100_000,
            &mut self.hot_store,
            &mut self.archive,
        ).map_err(|e| format!("Execution error: {}", e))?;

        match result {
            PipelineResult::Committed { post_state_root, .. } => {
                self.store.advance(height, [0u8; 32], post_state_root, vec![])?;
                self.metrics.record_block_produced();
                self.metrics.record_transaction();
                self.metrics.record_proof_generated();
                self.metrics.record_replay();
                self.health.current_height = height;
                self.health.is_synced = true;
                self.health.state_root = post_state_root;
                Ok(post_state_root)
            }
            PipelineResult::Rejected { .. } => Err("Block proposal rejected".into()),
        }
    }

    /// Import a block produced by another validator.
    /// VERIFIES by re-executing the same program and comparing state roots.
    pub fn import_block(&mut self, height: u64, expected_state_root: [u8; 32]) -> Result<(), String> {
        if height <= self.store.current_height() {
            return Ok(());
        }

        // Re-execute the same program to verify state transition
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

        let result = ConstitutionalRuntime::execute(
            &program,
            &ctx,
            self.store.registry_mut(),
            &[],
            100_000,
            &mut self.hot_store,
            &mut self.archive,
        ).map_err(|e| format!("Import execution error: {}", e))?;

        match result {
            PipelineResult::Committed { post_state_root, .. } => {
                // VERIFY: our computed state_root must match the leader's
                if post_state_root != expected_state_root {
                    return Err(format!(
                        "State root mismatch: computed {:?} != expected {:?}",
                        &post_state_root[..4], &expected_state_root[..4]
                    ));
                }
                self.store.advance(height, [0u8; 32], post_state_root, vec![])?;
                self.metrics.record_block_imported();
                self.metrics.record_proof_verified();
                self.health.current_height = height;
                self.health.is_synced = true;
                self.health.state_root = post_state_root;
                Ok(())
            }
            PipelineResult::Rejected { .. } => Err("Imported block rejected".into()),
        }
    }
}
