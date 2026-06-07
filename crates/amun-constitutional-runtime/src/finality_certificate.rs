use amun_transition_proof::transition_proof::TransitionProof;
use serde::{Deserialize, Serialize};

use crate::block_validator::BlockValidationResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalFinalityCertificate {
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub evidence_root: [u8; 32],
    pub pccv_root: [u8; 32],
    pub qc_hash: [u8; 32],
    pub previous_certificate_hash: [u8; 32],
    pub certificate_hash: [u8; 32],
    pub transitions: Vec<TransitionProof>,
    pub pccv_results: Vec<String>,
    pub all_verified: bool,
}

impl ConstitutionalFinalityCertificate {
    pub fn issue(
        block_result: &BlockValidationResult,
        transitions: Vec<TransitionProof>,
        qc_hash: [u8; 32],
        block_height: u64,
        block_hash: [u8; 32],
    ) -> Self {
        let state_root = block_result.state_root;
        let proof_root = Self::compute_merkle_root(
            transitions.iter().map(|p| p.proof_hash).collect(),
        );
        let evidence_root = Self::compute_evidence_root(&transitions);
        let pccv_root = Self::compute_pccv_root(&transitions);
        let all_verified = block_result.block_valid
            && block_result.pccv_failed == 0
            && block_result.rejected == 0;
        let pccv_results: Vec<String> = transitions.iter().map(|_| "VERIFIED".to_string()).collect();

        let mut cert = Self {
            block_height, block_hash, state_root, proof_root, evidence_root,
            pccv_root, qc_hash, previous_certificate_hash: [0u8; 32],
            certificate_hash: [0u8; 32], transitions, pccv_results, all_verified,
        };
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    pub fn verify(&self) -> bool {
        self.certificate_hash == self.compute_hash() && self.all_verified
    }

    pub fn is_constitutionally_valid(&self) -> bool {
        self.all_verified && !self.transitions.is_empty() && self.proof_root != [0u8; 32]
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_FINALITY_CERTIFICATE_V2");
        hasher.update(&self.block_height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.state_root);
        hasher.update(&self.proof_root);
        hasher.update(&self.evidence_root);
        hasher.update(&self.pccv_root);
        hasher.update(&self.qc_hash);
        hasher.update(&self.previous_certificate_hash);
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn compute_merkle_root(hashes: Vec<[u8; 32]>) -> [u8; 32] {
        if hashes.is_empty() { return [0u8; 32]; }
        let mut level = hashes;
        while level.len() > 1 {
            let mut next = Vec::new();
            for chunk in level.chunks(2) {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"AMUN_PROOF_ROOT_V1");
                hasher.update(&chunk[0]);
                if chunk.len() == 2 { hasher.update(&chunk[1]); }
                else { hasher.update(&chunk[0]); }
                let hash = hasher.finalize();
                let mut h = [0u8; 32];
                h.copy_from_slice(hash.as_bytes());
                next.push(h);
            }
            level = next;
        }
        level[0]
    }

    pub fn compute_evidence_root(transitions: &[TransitionProof]) -> [u8; 32] {
        let ids: Vec<[u8; 32]> = transitions.iter()
            .flat_map(|p| p.evidence.iter().map(|e| e.evidence_id()))
            .collect();
        Self::compute_merkle_root(ids)
    }

    pub fn compute_pccv_root(transitions: &[TransitionProof]) -> [u8; 32] {
        let hashes: Vec<[u8; 32]> = transitions.iter().map(|p| {
            let mut hasher = blake3::Hasher::new();
            hasher.update(b"AMUN_PCCV_ROOT_V1");
            hasher.update(&p.proof_hash);
            let hash = hasher.finalize();
            let mut h = [0u8; 32];
            h.copy_from_slice(hash.as_bytes());
            h
        }).collect();
        Self::compute_merkle_root(hashes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::{ResourceId, ResourceRegistry};
    use amun_vm_kernel::execution_context::ExecutionContext;
    use amun_bytecode::program::ConstitutionalProgram;
    use amun_bytecode::opcodes::OpCode;
    use crate::block_validator::{ConstitutionalBlockValidator, BlockValidationResult};

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    #[test]
    fn n52_issue_finality_certificate() {
        let mut registry = ResourceRegistry::new(1000);
        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
        let programs: Vec<_> = (0..3).map(|i| {
            let ctx = ExecutionContext {
                contract_id: make_id(1), caller: [1u8; 32], block_height: 1,
                block_hash: [0u8; 32], transaction_hash: {
                    let mut h = [0u8; 32];
                    h[0..8].copy_from_slice(&(i as u64).to_le_bytes()); h
                },
                pre_state_root: registry.compute_state_root(),
                authority: [2u8; 32],
            };
            (program.clone(), ctx)
        }).collect();
        let block_result = ConstitutionalBlockValidator::validate_block(
            &programs, &mut registry, &[],
        ).unwrap();
        let transitions = vec![TransitionProof::new(
            [0xaa;32], make_id(1), 1, [0u8;32], [0u8;32], [0u8;32],
            vec![], vec![], vec![], vec![], 0,
        )];
        let cert = ConstitutionalFinalityCertificate::issue(
            &block_result, transitions, [0xaa; 32], 1, [0xbb; 32],
        );
        assert!(cert.verify());
        assert!(cert.is_constitutionally_valid());
    }

    #[test]
    fn n52_certificate_deterministic() {
        let br = BlockValidationResult {
            total_transactions: 3, committed: 3, rejected: 0,
            pccv_verified: 3, pccv_failed: 0, block_valid: true,
            state_root: [0x01; 32],
        };
        let c1 = ConstitutionalFinalityCertificate::issue(&br, vec![], [0xaa;32], 1, [0xbb;32]);
        let c2 = ConstitutionalFinalityCertificate::issue(&br, vec![], [0xaa;32], 1, [0xbb;32]);
        assert_eq!(c1.certificate_hash, c2.certificate_hash);
    }

    #[test]
    fn n52_certificate_detects_tampering() {
        let br = BlockValidationResult {
            total_transactions: 3, committed: 3, rejected: 0,
            pccv_verified: 3, pccv_failed: 0, block_valid: true,
            state_root: [0x01; 32],
        };
        let mut cert = ConstitutionalFinalityCertificate::issue(&br, vec![], [0xaa;32], 1, [0xbb;32]);
        cert.block_height = 999;
        assert!(!cert.verify());
    }
}
