use serde::{Serialize, Deserialize};
use amun_constitutional_commitments::smt::{SparseMerkleTree, MerkleProof};
use crate::CheckpointCertificate;

pub const CHECKPOINT_MERKLE_DOMAIN: &[u8] = b"AMUN_CHECKPOINT_MERKLE_DOMAIN";

pub fn checkpoint_merkle_root(checkpoints: &[CheckpointCertificate]) -> [u8; 32] {
    let mut tree = SparseMerkleTree::new(CHECKPOINT_MERKLE_DOMAIN);
    for (i, cert) in checkpoints.iter().enumerate() {
        let key = format!("{:020}", i);
        tree.insert(key.as_bytes(), &cert.checkpoint_hash_bytes());
    }
    tree.root()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointInclusionProof {
    pub checkpoint_hash: [u8; 32],
    pub index: u64,
    pub proof: MerkleProof,
    pub root: [u8; 32],
}

pub fn prove_checkpoint_inclusion(
    checkpoints: &[CheckpointCertificate],
    checkpoint_hash: &[u8; 32],
) -> Option<CheckpointInclusionProof> {
    let index = checkpoints.iter().position(|c| c.checkpoint_hash_bytes() == *checkpoint_hash)?;
    let mut tree = SparseMerkleTree::new(CHECKPOINT_MERKLE_DOMAIN);
    for (i, cert) in checkpoints.iter().enumerate() {
        let key = format!("{:020}", i);
        tree.insert(key.as_bytes(), &cert.checkpoint_hash_bytes());
    }
    let key = format!("{:020}", index);
    let proof = tree.prove(key.as_bytes());
    Some(CheckpointInclusionProof {
        checkpoint_hash: *checkpoint_hash,
        index: index as u64,
        proof,
        root: tree.root(),
    })
}

impl CheckpointInclusionProof {
    pub fn verify(&self) -> bool {
        if self.proof.leaf_value != Some(self.checkpoint_hash) {
            return false;
        }
        let tree = SparseMerkleTree::new(CHECKPOINT_MERKLE_DOMAIN);
        tree.verify(&self.root, &self.proof)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointBundle {
    pub checkpoint: CheckpointCertificate,
    pub inclusion_proof: CheckpointInclusionProof,
}

impl CheckpointBundle {
    pub fn new(checkpoint: CheckpointCertificate, inclusion_proof: CheckpointInclusionProof) -> Self {
        Self { checkpoint, inclusion_proof }
    }

    pub fn verify(&self) -> Result<(), String> {
        self.checkpoint.verify()?;
        if !self.inclusion_proof.verify() {
            return Err("Checkpoint inclusion proof verification failed".into());
        }
        Ok(())
    }
}

pub fn verify_checkpoint_sequence(
    bundles: &[CheckpointBundle],
    trusted_root: &[u8; 32],
) -> Result<(), String> {
    if bundles.is_empty() {
        return Err("Empty bundles".into());
    }
    for bundle in bundles {
        bundle.verify()?;
        if bundle.inclusion_proof.root != *trusted_root {
            return Err(format!(
                "Bundle root mismatch for checkpoint [{},{}]",
                bundle.checkpoint.start_height, bundle.checkpoint.end_height
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use amun_certificate_network::distribution::LightClientProofBundle;
    use super::*;
    use amun_constitutional_state::ConstitutionalStateRuntime;
    use amun_constitutional_block::ConstitutionalBlock;

    fn create_bundle(height: u64, parent_hash: &str) -> LightClientProofBundle {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
        let block = ConstitutionalBlock::new(
            height, parent_hash.into(), "t".into(), "p".into(), vec![],
            hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(), merkle_root,
        );
        LightClientProofBundle::new(block, cert, proof)
    }

    fn create_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
        let mut bundles: Vec<LightClientProofBundle> = Vec::new();
        let parent = "0".repeat(64);
        for h in start..=end {
            let parent_hash = if h == start { &parent } else { &bundles.last().unwrap().block_header.block_hash };
            bundles.push(create_bundle(h, parent_hash));
        }
        CheckpointCertificate::create(start, end, &bundles).unwrap()
    }

    #[test] fn n12b_single_checkpoint_merkle_root() { let c = create_checkpoint(0, 0); assert_ne!(checkpoint_merkle_root(&[c]), [0u8; 32]); }
    #[test] fn n12b_multiple_checkpoints_different_root() { let c1 = create_checkpoint(0, 0); let c2 = create_checkpoint(1, 1); assert_ne!(checkpoint_merkle_root(std::slice::from_ref(&c1)), checkpoint_merkle_root(&[c1, c2])); }
    #[test] fn n12b_inclusion_proof_valid() { let c1 = create_checkpoint(0, 0); let c2 = create_checkpoint(1, 1); let certs = vec![c1.clone(), c2.clone()]; let root = checkpoint_merkle_root(&certs); let proof = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap(); assert_eq!(proof.root, root); assert!(proof.verify()); }
    #[test] fn n12b_inclusion_proof_wrong_hash_fails() { let c1 = create_checkpoint(0, 0); let c2 = create_checkpoint(1, 1); let certs = vec![c1.clone(), c2.clone()]; let mut proof = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap(); proof.checkpoint_hash = c2.checkpoint_hash_bytes(); assert!(!proof.verify()); }
    #[test] fn n12b_root_mismatch_fails() { let c1 = create_checkpoint(0, 0); let certs = vec![c1.clone()]; let mut proof = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap(); proof.root = [0xFF; 32]; assert!(!proof.verify()); }
    #[test] fn n12c_bundle_valid() { let c = create_checkpoint(0, 0); let certs = vec![c.clone()]; let proof = prove_checkpoint_inclusion(&certs, &c.checkpoint_hash_bytes()).unwrap(); assert!(CheckpointBundle::new(c, proof).verify().is_ok()); }
    #[test] fn n12d_light_verify_sequence() { let c1 = create_checkpoint(0, 0); let c2 = create_checkpoint(1, 1); let certs = vec![c1.clone(), c2.clone()]; let root = checkpoint_merkle_root(&certs); let p1 = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap(); let p2 = prove_checkpoint_inclusion(&certs, &c2.checkpoint_hash_bytes()).unwrap(); let bundles = vec![CheckpointBundle::new(c1, p1), CheckpointBundle::new(c2, p2)]; assert!(verify_checkpoint_sequence(&bundles, &root).is_ok()); }
    #[test] fn n12d_light_verify_wrong_root() { let c1 = create_checkpoint(0, 0); let certs = vec![c1.clone()]; let p1 = prove_checkpoint_inclusion(&certs, &c1.checkpoint_hash_bytes()).unwrap(); assert!(verify_checkpoint_sequence(&[CheckpointBundle::new(c1, p1)], &[0x11; 32]).is_err()); }
}
