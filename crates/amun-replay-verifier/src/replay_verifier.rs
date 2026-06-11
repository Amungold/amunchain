use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_invariant_engine::invariant_types::InvariantDeclaration;
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;
use amun_resource_core::{
    ResourceArchetype, ResourceLineage, ResourceMetadata, ResourceRegistry, ResourceState,
};
use amun_transition_proof::transition_proof::TransitionProof;
use amun_vm_kernel::execution_context::ExecutionContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayResult {
    Match {
        state_root: [u8; 32],
        proof_hash: [u8; 32],
    },
    Divergence {
        field: String,
        original: String,
        replay: String,
    },
    Error {
        reason: String,
    },
}

pub struct ReplayVerifier;

impl ReplayVerifier {
    pub fn replay(
        proof: &TransitionProof,
        program: &ConstitutionalProgram,
        registry: &mut ResourceRegistry,
        invariants: &[InvariantDeclaration],
    ) -> ReplayResult {
        // Load pre-state from proof's produced resources metadata
        for meta in &proof.produced_resources {
            if meta.lineage.parent_resource_ids.is_empty() {
                let _ = registry.register_genesis(meta.clone());
            }
        }
        // Also register consumed resources' parents to reconstruct pre-state
        for meta in &proof.produced_resources {
            if !meta.lineage.parent_resource_ids.is_empty() {
                let _ = registry.register_genesis(ResourceMetadata {
                    resource_id: meta.lineage.parent_resource_ids[0],
                    archetype: ResourceArchetype::Asset,
                    state: ResourceState::Active,
                    lineage: ResourceLineage::genesis(meta.lineage.parent_resource_ids[0]),
                    contract_id: [1u8; 32],
                    owner: [2u8; 32],
                });
            }
        }

        let ctx = ExecutionContext {
            contract_id: proof.contract_id,
            caller: [0u8; 32],
            block_height: proof.block_height,
            block_hash: proof.block_hash,
            transaction_hash: proof.transaction_hash,
            pre_state_root: proof.pre_state_root,
            authority: [0u8; 32],
        };

        let mut hot_store = HotProofStore::new(1000);
        let mut archive = ProofArchive::new();
        let result = ConstitutionalRuntime::execute(
            program,
            &ctx,
            registry,
            invariants,
            100_000,
            &mut hot_store,
            &mut archive,
        );

        match result {
            Ok(PipelineResult::Committed {
                post_state_root,
                transition_proof: replay_proof,
                ..
            }) => {
                if post_state_root != proof.post_state_root {
                    return ReplayResult::Divergence {
                        field: "post_state_root".into(),
                        original: hex::encode(proof.post_state_root),
                        replay: hex::encode(post_state_root),
                    };
                }
                if replay_proof.gas_used != proof.gas_used {
                    return ReplayResult::Divergence {
                        field: "gas_used".into(),
                        original: proof.gas_used.to_string(),
                        replay: replay_proof.gas_used.to_string(),
                    };
                }
                if replay_proof.proof_hash != proof.proof_hash {
                    return ReplayResult::Divergence {
                        field: "proof_hash".into(),
                        original: hex::encode(proof.proof_hash),
                        replay: hex::encode(replay_proof.proof_hash),
                    };
                }
                ReplayResult::Match {
                    state_root: post_state_root,
                    proof_hash: replay_proof.proof_hash,
                }
            }
            Ok(PipelineResult::Rejected { evidence, .. }) => ReplayResult::Error {
                reason: format!("Rejected: {} evidence", evidence.len()),
            },
            Err(e) => ReplayResult::Error { reason: e },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_bytecode::opcodes::OpCode;
    use amun_resource_core::ResourceId;
    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32];
        h[0] = seed;
        ResourceId(h)
    }

    #[test]
    fn w14_replay_equality() {
        let mut r1 = ResourceRegistry::new(1000);
        let mut r2 = ResourceRegistry::new(1000);
        let p = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Push(42), OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1),
            caller: [1u8; 32],
            block_height: 1,
            block_hash: [0u8; 32],
            transaction_hash: [0xaa; 32],
            pre_state_root: r1.compute_state_root(),
            authority: [2u8; 32],
        };
        let mut h = HotProofStore::new(100);
        let mut a = ProofArchive::new();
        let res =
            ConstitutionalRuntime::execute(&p, &ctx, &mut r1, &[], 10000, &mut h, &mut a).unwrap();
        let proof = match res {
            PipelineResult::Committed {
                transition_proof, ..
            } => transition_proof,
            _ => panic!(),
        };
        assert!(matches!(
            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
            ReplayResult::Match { .. }
        ));
    }

    #[test]
    fn w14_replay_detects_tampered_proof() {
        let mut r = ResourceRegistry::new(1000);
        let p = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(2),
            caller: [1u8; 32],
            block_height: 1,
            block_hash: [0u8; 32],
            transaction_hash: [0xbb; 32],
            pre_state_root: r.compute_state_root(),
            authority: [2u8; 32],
        };
        let mut h = HotProofStore::new(100);
        let mut a = ProofArchive::new();
        let res =
            ConstitutionalRuntime::execute(&p, &ctx, &mut r, &[], 10000, &mut h, &mut a).unwrap();
        let mut proof = match res {
            PipelineResult::Committed {
                transition_proof, ..
            } => transition_proof,
            _ => panic!(),
        };
        proof.gas_used = 99999;
        let mut r2 = ResourceRegistry::new(1000);
        assert!(!matches!(
            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
            ReplayResult::Match { .. }
        ));
    }

    #[test]
    fn w14_multi_replay_same_result() {
        let p = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(3),
            caller: [1u8; 32],
            block_height: 1,
            block_hash: [0u8; 32],
            transaction_hash: [0xcc; 32],
            pre_state_root: [0u8; 32],
            authority: [2u8; 32],
        };
        let mut h = HotProofStore::new(100);
        let mut a = ProofArchive::new();
        let mut r = ResourceRegistry::new(1000);
        let res =
            ConstitutionalRuntime::execute(&p, &ctx, &mut r, &[], 10000, &mut h, &mut a).unwrap();
        let proof = match res {
            PipelineResult::Committed {
                transition_proof, ..
            } => transition_proof,
            _ => panic!(),
        };
        for _ in 0..3 {
            let mut f = ResourceRegistry::new(1000);
            assert!(matches!(
                ReplayVerifier::replay(&proof, &p, &mut f, &[]),
                ReplayResult::Match { .. }
            ));
        }
    }
}
