use crate::inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle};
use crate::CheckpointCertificate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveCheckpointProof {
    pub bundles: Vec<CheckpointBundle>,
    pub common_root: [u8; 32],
}

impl RecursiveCheckpointProof {
    pub fn verify(&self) -> Result<(), String> {
        if self.bundles.is_empty() {
            return Err("Empty chain".into());
        }
        let mut expected_height = self.bundles[0].checkpoint.start_height;
        for (i, bundle) in self.bundles.iter().enumerate() {
            bundle.verify()?;
            if bundle.inclusion_proof.root != self.common_root {
                return Err(format!("Bundle {} root mismatch", i));
            }
            if bundle.checkpoint.start_height != expected_height {
                return Err(format!("Height discontinuity at bundle {}", i));
            }
            expected_height = bundle.checkpoint.end_height + 1;
        }
        Ok(())
    }

    pub fn from_checkpoints(checkpoints: &[CheckpointCertificate]) -> Option<Self> {
        if checkpoints.is_empty() {
            return None;
        }
        let root = checkpoint_merkle_root(checkpoints);
        let mut bundles = Vec::new();
        for c in checkpoints {
            let proof = prove_checkpoint_inclusion(checkpoints, &c.checkpoint_hash_bytes())?;
            bundles.push(CheckpointBundle::new(c.clone(), proof));
        }
        Some(Self {
            bundles,
            common_root: root,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_certificate_network::distribution::LightClientProofBundle;
    use amun_constitutional_block::ConstitutionalBlock;
    use amun_constitutional_state::ConstitutionalStateRuntime;

    fn create_bundle(height: u64, parent_hash: &str) -> LightClientProofBundle {
        let mut rt = ConstitutionalStateRuntime::new();
        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
        let block = ConstitutionalBlock::new(
            height,
            parent_hash.into(),
            "t".into(),
            "p".into(),
            vec![],
            hex::encode(rt.state_root()),
            "g".into(),
            "e".into(),
            "ev".into(),
            merkle_root,
        );
        LightClientProofBundle::new(block, cert, proof)
    }

    fn create_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
        let mut bundles: Vec<LightClientProofBundle> = Vec::new();
        let parent = "0".repeat(64);
        for h in start..=end {
            let parent_hash = if h == start {
                &parent
            } else {
                &bundles.last().unwrap().block_header.block_hash
            };
            bundles.push(create_bundle(h, parent_hash));
        }
        CheckpointCertificate::create(start, end, &bundles).unwrap()
    }

    #[test]
    fn n13_single_checkpoint_chain() {
        assert!(
            RecursiveCheckpointProof::from_checkpoints(&[create_checkpoint(0, 0)])
                .unwrap()
                .verify()
                .is_ok()
        );
    }
    #[test]
    fn n13_two_checkpoint_chain() {
        let c1 = create_checkpoint(0, 1);
        let c2 = create_checkpoint(2, 3);
        assert!(RecursiveCheckpointProof::from_checkpoints(&[c1, c2])
            .unwrap()
            .verify()
            .is_ok());
    }
    #[test]
    fn n13_discontinuous_chain_fails() {
        let c1 = create_checkpoint(0, 0);
        let c2 = create_checkpoint(2, 2);
        assert!(RecursiveCheckpointProof::from_checkpoints(&[c1, c2])
            .unwrap()
            .verify()
            .is_err());
    }
    #[test]
    fn n13_wrong_root_fails() {
        let mut proof =
            RecursiveCheckpointProof::from_checkpoints(&[create_checkpoint(0, 0)]).unwrap();
        proof.common_root = [0xFF; 32];
        assert!(proof.verify().is_err());
    }
}
