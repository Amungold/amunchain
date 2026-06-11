use crate::inclusion::{verify_checkpoint_sequence, CheckpointBundle};
use amun_constitutional_state::ConstitutionalStateRuntime;
use amun_stateless_sync::StatelessNode;

pub struct BootstrapSession {
    trusted_checkpoint_root: [u8; 32],
    node: StatelessNode,
}

impl BootstrapSession {
    pub fn new(trusted_checkpoint_root: [u8; 32]) -> Self {
        Self {
            trusted_checkpoint_root,
            node: StatelessNode::new(),
        }
    }

    pub fn ingest_bundles(&mut self, bundles: &[CheckpointBundle]) -> Result<(), String> {
        verify_checkpoint_sequence(bundles, &self.trusted_checkpoint_root)
    }

    pub fn verify_complete_chain(&self, expected_state_root: &str) -> Result<(), String> {
        self.node.verify_chain()?;
        let rt = ConstitutionalStateRuntime::new();
        let current = hex::encode(rt.state_root());
        if current != expected_state_root {
            return Err(format!(
                "State root mismatch: expected {} got {}",
                expected_state_root, current
            ));
        }
        Ok(())
    }

    pub fn trusted_root(&self) -> [u8; 32] {
        self.trusted_checkpoint_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion};
    use crate::CheckpointCertificate;
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
    fn n14_accepts_valid_bundles() {
        let c = create_checkpoint(0, 0);
        let certs = vec![c.clone()];
        let root = checkpoint_merkle_root(&certs);
        let proof = prove_checkpoint_inclusion(&certs, &c.checkpoint_hash_bytes()).unwrap();
        let bundle = CheckpointBundle::new(c, proof);

        let mut session = BootstrapSession::new(root);
        assert!(session.ingest_bundles(&[bundle]).is_ok());
    }

    #[test]
    fn n14_rejects_wrong_root() {
        let c = create_checkpoint(0, 0);
        let certs = vec![c.clone()];
        let proof = prove_checkpoint_inclusion(&certs, &c.checkpoint_hash_bytes()).unwrap();
        let bundle = CheckpointBundle::new(c, proof);

        let mut session = BootstrapSession::new([0x11; 32]);
        assert!(session.ingest_bundles(&[bundle]).is_err());
    }

    #[test]
    fn n14_rejects_empty_bundles() {
        let root = checkpoint_merkle_root(&[]);
        let mut session = BootstrapSession::new(root);
        assert!(session.ingest_bundles(&[]).is_err());
    }

    #[test]
    fn n14_trusted_root_stored() {
        let root = [0xAB; 32];
        let session = BootstrapSession::new(root);
        assert_eq!(session.trusted_root(), root);
    }
}
