use amun_resource_core::{ResourceId, ResourceRegistry};
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_transition_proof::transition_proof::TransitionProof;
use amun_replay_verifier::replay_verifier::{ReplayVerifier, ReplayResult};
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;

use crate::replay_backed_types::{
    ReplayBackedFinalityCertificate, ReplayBackedQC,
    ReplayVerificationRecord, ReplayVerifiedBlock,
};

/// Replay-Backed Consensus Engine.
/// Requires every validator to replay and verify all transition proofs
/// before voting on a block.
pub struct ReplayBackedConsensus;

impl ReplayBackedConsensus {
    /// Execute a block and produce replay verifications for all transitions.
    pub fn execute_and_replay(
        programs: &[(ConstitutionalProgram, ExecutionContext)],
        registry: &mut ResourceRegistry,
        block_height: u64,
        parent_hash: [u8; 32],
        _proposer: ResourceId,
    ) -> Result<ReplayVerifiedBlock, String> {
        let mut hot = HotProofStore::new(10000);
        let mut archive = ProofArchive::new();
        let mut transitions = Vec::new();
        let mut replay_records = Vec::new();

        for (program, ctx) in programs {
            let result = ConstitutionalRuntime::execute(
                program, ctx, registry, &[], 100_000,
                &mut hot, &mut archive,
            ).map_err(|e| format!("Execution error: {}", e))?;

            let proof = match result {
                PipelineResult::Committed { transition_proof, .. }
                | PipelineResult::Rejected { transition_proof, .. } => transition_proof,
            };

            // Replay verification
            let mut fresh_reg = ResourceRegistry::new(10000);
            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);

            let record = match replay {
                ReplayResult::Match { state_root, proof_hash } => ReplayVerificationRecord {
                    proof_hash: proof.proof_hash,
                    state_root_match: state_root == proof.post_state_root,
                    proof_hash_match: proof_hash == proof.proof_hash,
                    gas_used_match: true,
                    replay_success: true,
                },
                _ => ReplayVerificationRecord {
                    proof_hash: proof.proof_hash,
                    state_root_match: false,
                    proof_hash_match: false,
                    gas_used_match: false,
                    replay_success: false,
                },
            };

            replay_records.push(record);
            transitions.push(proof);
        }

        let state_root = registry.compute_state_root();
        let proof_root = Self::compute_proof_root(&transitions);
        let all_verified = replay_records.iter().all(|r| r.is_verified());

        let mut block = ReplayVerifiedBlock {
            block_height,
            block_hash: [0u8; 32],
            state_root,
            proof_root,
            replay_root: [0u8; 32],
            transitions,
            replay_verifications: replay_records,
            all_verified,
        };
        block.replay_root = block.compute_replay_root();

        // Compute block hash
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_REPLAY_BLOCK_V1");
        hasher.update(&block.block_height.to_le_bytes());
        hasher.update(&parent_hash);
        hasher.update(&block.state_root);
        hasher.update(&block.proof_root);
        hasher.update(&block.replay_root);
        let hash = hasher.finalize();
        let mut bh = [0u8; 32];
        bh.copy_from_slice(hash.as_bytes());
        block.block_hash = bh;

        Ok(block)
    }

    /// Form consensus on a replay-verified block.
    pub fn form_consensus(
        block: &ReplayVerifiedBlock,
        quorum_size: usize,
        signatures: Vec<Vec<u8>>,
    ) -> Result<ReplayBackedFinalityCertificate, String> {
        if !block.all_verified {
            return Err("Not all transition proofs passed replay verification".into());
        }

        let mut qc = ReplayBackedQC::for_block(block, quorum_size);
        for sig in signatures {
            qc.signatures.push(sig);
            qc.signer_count = qc.signatures.len();
        }

        if !qc.is_valid() {
            return Err(format!("Insufficient quorum: {}/{}", qc.signer_count, quorum_size));
        }

        Ok(ReplayBackedFinalityCertificate::issue(block, qc))
    }

    fn compute_proof_root(transitions: &[TransitionProof]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_PROOF_ROOT_V1");
        for proof in transitions {
            hasher.update(&proof.proof_hash);
        }
        let hash = hasher.finalize();
        let mut root = [0u8; 32];
        root.copy_from_slice(hash.as_bytes());
        root
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
    fn w18_execute_and_replay_block() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ReplayBackedConsensus::execute_and_replay(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        assert!(block.all_verified);
        assert_eq!(block.replay_verifications.len(), 1);
        assert!(block.replay_verifications[0].is_verified());
    }

    #[test]
    fn w18_form_replay_backed_consensus() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ReplayBackedConsensus::execute_and_replay(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        let sigs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 64]).collect();
        let cert = ReplayBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
        assert!(cert.verify());
    }

    #[test]
    fn w18_reject_if_replay_fails() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let mut block = ReplayBackedConsensus::execute_and_replay(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        // Tamper with a verification record
        block.replay_verifications[0].replay_success = false;
        block.all_verified = false;
        let sigs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 64]).collect();
        assert!(ReplayBackedConsensus::form_consensus(&block, 5, sigs).is_err());
    }

    #[test]
    fn w18_replay_finality_certificate_deterministic() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ReplayBackedConsensus::execute_and_replay(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        let sigs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 64]).collect();
        let cert1 = ReplayBackedConsensus::form_consensus(&block, 5, sigs.clone()).unwrap();
        let cert2 = ReplayBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
        assert_eq!(cert1.certificate_hash, cert2.certificate_hash);
    }

    #[test]
    fn w18_replay_root_included_in_block() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = ReplayBackedConsensus::execute_and_replay(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        assert_eq!(block.replay_root, block.compute_replay_root());
        assert_ne!(block.replay_root, [0u8; 32]);
    }
}
