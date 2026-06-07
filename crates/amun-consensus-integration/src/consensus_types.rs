use amun_resource_core::ResourceId;
use amun_transition_proof::transition_proof::TransitionProof;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalBlock {
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub parent_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub transitions: Vec<TransitionProof>,
    pub proposer: ResourceId,
    pub timestamp: u64,
}

impl ConstitutionalBlock {
    pub fn compute_proof_root(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_PROOF_ROOT_V1");
        for proof in &self.transitions {
            hasher.update(&proof.proof_hash);
        }
        let hash = hasher.finalize();
        let mut root = [0u8; 32];
        root.copy_from_slice(hash.as_bytes());
        root
    }

    pub fn verify_all_proofs(&self) -> bool {
        if self.transitions.is_empty() {
            return self.proof_root == [0u8; 32];
        }
        for proof in &self.transitions {
            if !proof.verify_integrity() {
                return false;
            }
        }
        self.proof_root == self.compute_proof_root()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalQC {
    pub block_hash: [u8; 32],
    pub block_height: u64,
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub signatures: Vec<Vec<u8>>,
    pub signer_count: usize,
    pub quorum_threshold: usize,
}

impl ConstitutionalQC {
    pub fn is_valid(&self) -> bool {
        self.signer_count >= self.quorum_threshold
    }

    pub fn for_block(block: &ConstitutionalBlock, threshold: usize) -> Self {
        Self {
            block_hash: block.block_hash,
            block_height: block.block_height,
            state_root: block.state_root,
            proof_root: block.proof_root,
            signatures: Vec::new(),
            signer_count: 0,
            quorum_threshold: threshold,
        }
    }

    pub fn add_signature(&mut self, sig: Vec<u8>) {
        self.signatures.push(sig);
        self.signer_count = self.signatures.len();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalFinalityCertificate {
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub qc: ConstitutionalQC,
    pub certificate_hash: [u8; 32],
}

impl ConstitutionalFinalityCertificate {
    pub fn issue(block: &ConstitutionalBlock, qc: ConstitutionalQC) -> Self {
        let mut cert = Self {
            block_height: block.block_height,
            block_hash: block.block_hash,
            state_root: block.state_root,
            proof_root: block.proof_root,
            qc,
            certificate_hash: [0u8; 32],
        };
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_FINALITY_CERTIFICATE_V1");
        hasher.update(&self.block_height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.state_root);
        hasher.update(&self.proof_root);
        hasher.update(&self.qc.block_hash);
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }

    pub fn verify(&self) -> bool {
        self.certificate_hash == self.compute_hash() && self.qc.is_valid()
    }
}
