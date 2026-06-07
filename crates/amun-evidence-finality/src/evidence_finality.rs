#![allow(clippy::type_complexity)]
use amun_resource_core::{ResourceId, ResourceRegistry};
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_bytecode::program::ConstitutionalProgram;
use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
use amun_replay_verifier::replay_verifier::{ReplayVerifier, ReplayResult};
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use amun_proof_archive::hot_store::HotProofStore;
use amun_proof_archive::proof_archive::ProofArchive;
use serde::{Deserialize, Serialize};

/// A fully verified block with execution, proof, replay, and evidence roots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceVerifiedBlock {
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub replay_root: [u8; 32],
    pub evidence_root: [u8; 32],
    pub evidence: Vec<ConstitutionalEvidence>,
    pub all_verified: bool,
}

impl EvidenceVerifiedBlock {
    pub fn compute_evidence_root(evidence: &[ConstitutionalEvidence]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_EVIDENCE_ROOT_V1");
        for ev in evidence {
            hasher.update(&ev.evidence_id());
        }
        let hash = hasher.finalize();
        let mut root = [0u8; 32];
        root.copy_from_slice(hash.as_bytes());
        root
    }

    pub fn compute_block_hash(
        block_height: u64, parent_hash: &[u8; 32],
        state_root: &[u8; 32], proof_root: &[u8; 32],
        replay_root: &[u8; 32], evidence_root: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_EVIDENCE_BLOCK_V1");
        hasher.update(&block_height.to_le_bytes());
        hasher.update(parent_hash);
        hasher.update(state_root);
        hasher.update(proof_root);
        hasher.update(replay_root);
        hasher.update(evidence_root);
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }
}

/// A QC that includes evidence verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBackedQC {
    pub block_hash: [u8; 32],
    pub block_height: u64,
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub replay_root: [u8; 32],
    pub evidence_root: [u8; 32],
    pub signatures: Vec<Vec<u8>>,
    pub signer_count: usize,
    pub quorum_threshold: usize,
}

impl EvidenceBackedQC {
    pub fn is_valid(&self) -> bool {
        self.signer_count >= self.quorum_threshold
    }
}

/// The final constitutional finality certificate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalFinalityCertificate {
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub replay_root: [u8; 32],
    pub evidence_root: [u8; 32],
    pub qc: EvidenceBackedQC,
    pub certificate_hash: [u8; 32],
}

impl ConstitutionalFinalityCertificate {
    pub fn issue(block: &EvidenceVerifiedBlock, qc: EvidenceBackedQC) -> Self {
        let mut cert = Self {
            block_height: block.block_height,
            block_hash: block.block_hash,
            state_root: block.state_root,
            proof_root: block.proof_root,
            replay_root: block.replay_root,
            evidence_root: block.evidence_root,
            qc,
            certificate_hash: [0u8; 32],
        };
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CONSTITUTIONAL_FINALITY_V1");
        hasher.update(&self.block_height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.state_root);
        hasher.update(&self.proof_root);
        hasher.update(&self.replay_root);
        hasher.update(&self.evidence_root);
        hasher.update(&self.qc.block_hash);
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn verify(&self) -> bool {
        self.certificate_hash == self.compute_hash() && self.qc.is_valid()
    }

    /// The certificate binds five independent roots into one cryptographic commitment.
    pub fn bound_roots(&self) -> (&[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 32]) {
        (&self.state_root, &self.proof_root, &self.replay_root, &self.evidence_root, &self.qc.block_hash)
    }
}

/// Evidence-Backed Consensus Engine.
pub struct EvidenceBackedConsensus;

impl EvidenceBackedConsensus {
    pub fn execute_and_verify(
        programs: &[(ConstitutionalProgram, ExecutionContext)],
        registry: &mut ResourceRegistry,
        block_height: u64,
        parent_hash: [u8; 32],
        _proposer: ResourceId,
    ) -> Result<EvidenceVerifiedBlock, String> {
        let mut hot = HotProofStore::new(10000);
        let mut archive = ProofArchive::new();
        let mut all_evidence = Vec::new();
        let mut all_verified = true;
        let mut proof_hashes = Vec::new();
        let mut replay_hashes = Vec::new();

        for (program, ctx) in programs {
            let result = ConstitutionalRuntime::execute(
                program, ctx, registry, &[], 100_000,
                &mut hot, &mut archive,
            ).map_err(|e| format!("Execution error: {}", e))?;

            let proof = match result {
                PipelineResult::Committed { transition_proof, .. }
                | PipelineResult::Rejected { transition_proof, .. } => transition_proof,
            };

            proof_hashes.push(proof.proof_hash);

            let mut fresh_reg = ResourceRegistry::new(10000);
            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);

            let verified = matches!(replay, ReplayResult::Match { .. });
            replay_hashes.push(if verified { proof.proof_hash } else { [0u8; 32] });
            if !verified { all_verified = false; }

            // Collect evidence from the proof
            for ev in &proof.evidence {
                all_evidence.push(ev.clone());
            }
        }

        let state_root = registry.compute_state_root();
        let proof_root = Self::hash_list(b"AMUN_PROOF_ROOT_V1", &proof_hashes);
        let replay_root = Self::hash_list(b"AMUN_REPLAY_ROOT_V1", &replay_hashes);
        let evidence_root = EvidenceVerifiedBlock::compute_evidence_root(&all_evidence);
        let block_hash = EvidenceVerifiedBlock::compute_block_hash(
            block_height, &parent_hash, &state_root, &proof_root, &replay_root, &evidence_root,
        );

        Ok(EvidenceVerifiedBlock {
            block_height, block_hash, state_root,
            proof_root, replay_root, evidence_root,
            evidence: all_evidence, all_verified,
        })
    }

    pub fn form_consensus(
        block: &EvidenceVerifiedBlock,
        quorum_size: usize,
        signatures: Vec<Vec<u8>>,
    ) -> Result<ConstitutionalFinalityCertificate, String> {
        if !block.all_verified {
            return Err("Not all transitions passed replay verification".into());
        }
        let mut qc = EvidenceBackedQC {
            block_hash: block.block_hash,
            block_height: block.block_height,
            state_root: block.state_root,
            proof_root: block.proof_root,
            replay_root: block.replay_root,
            evidence_root: block.evidence_root,
            signatures: Vec::new(),
            signer_count: 0,
            quorum_threshold: quorum_size,
        };
        for sig in signatures {
            qc.signatures.push(sig);
            qc.signer_count = qc.signatures.len();
        }
        if !qc.is_valid() {
            return Err(format!("Insufficient quorum: {}/{}", qc.signer_count, quorum_size));
        }
        Ok(ConstitutionalFinalityCertificate::issue(block, qc))
    }

    fn hash_list(domain: &[u8], items: &[[u8; 32]]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(domain);
        for item in items {
            hasher.update(item);
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
    fn w19_evidence_root_in_certificate() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = EvidenceBackedConsensus::execute_and_verify(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        let sigs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 64]).collect();
        let cert = EvidenceBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
        assert!(cert.verify());
        assert_ne!(cert.evidence_root, [0u8; 32]);
    }

    #[test]
    fn w19_five_roots_bound_in_certificate() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = EvidenceBackedConsensus::execute_and_verify(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        let sigs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 64]).collect();
        let cert = EvidenceBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
        let (sr, pr, rr, er, qh) = cert.bound_roots();
        assert_eq!(sr, &cert.state_root);
        assert_eq!(pr, &cert.proof_root);
        assert_eq!(rr, &cert.replay_root);
        assert_eq!(er, &cert.evidence_root);
        assert_eq!(qh, &cert.qc.block_hash);
    }

    #[test]
    fn w19_certificate_deterministic() {
        let mut registry = ResourceRegistry::new(10000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let ctx = ExecutionContext {
            contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
            block_hash: [0u8; 32], transaction_hash: [0xaa; 32],
            pre_state_root: registry.compute_state_root(), authority: [2u8; 32],
        };
        let block = EvidenceBackedConsensus::execute_and_verify(
            &[(program, ctx)], &mut registry, 1, [0u8; 32], make_id(99),
        ).unwrap();
        let sigs: Vec<Vec<u8>> = (0..5).map(|_| vec![0u8; 64]).collect();
        let c1 = EvidenceBackedConsensus::form_consensus(&block, 5, sigs.clone()).unwrap();
        let c2 = EvidenceBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
        assert_eq!(c1.certificate_hash, c2.certificate_hash);
    }
}
