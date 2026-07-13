use serde::{Deserialize, Serialize};

use amun_constitutional_runtime::certificate_chain::CertificateChain;
use amun_constitutional_runtime::finality_certificate::ConstitutionalFinalityCertificate;
use amun_constitutional_runtime::history_root::ConstitutionalHistoryRoot;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalCheckpoint {
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub certificate_hash: [u8; 32],
    pub history_root: [u8; 32],
    pub state_root: [u8; 32],
    pub proof_root: [u8; 32],
    pub evidence_root: [u8; 32],
    pub pccv_root: [u8; 32],
}

pub struct ConstitutionalLightClient {
    pub trusted_checkpoint: Option<ConstitutionalCheckpoint>,
    pub verified_heights: Vec<u64>,
}

impl ConstitutionalLightClient {
    pub fn new() -> Self {
        Self {
            trusted_checkpoint: None,
            verified_heights: Vec::new(),
        }
    }

    pub fn bootstrap(&mut self, checkpoint: ConstitutionalCheckpoint) {
        self.trusted_checkpoint = Some(checkpoint);
        self.verified_heights.clear();
    }

    pub fn verify_certificate(&self, cert: &ConstitutionalFinalityCertificate) -> bool {
        cert.verify() && cert.is_constitutionally_valid()
    }

    pub fn verify_chain_extension(&self, chain: &CertificateChain) -> bool {
        if let Some(ref checkpoint) = self.trusted_checkpoint {
            if chain.genesis().block_height != checkpoint.block_height + 1 {
                return false;
            }
            if !chain.verify_chain() {
                return false;
            }
            chain.certificates[0].previous_certificate_hash == checkpoint.certificate_hash
        } else {
            chain.verify_chain()
        }
    }

    pub fn verify_history_root(
        &self,
        root: &ConstitutionalHistoryRoot,
        chain: &CertificateChain,
    ) -> bool {
        root.verify_chain(chain)
    }

    pub fn advance(&mut self, chain: &CertificateChain) -> Result<(), String> {
        if !self.verify_chain_extension(chain) {
            return Err("Chain verification failed".into());
        }
        let tip = chain.tip();
        let history_root = ConstitutionalHistoryRoot::from_chain(chain);
        self.trusted_checkpoint = Some(ConstitutionalCheckpoint {
            block_height: tip.block_height,
            block_hash: tip.block_hash,
            certificate_hash: tip.certificate_hash,
            history_root: history_root.history_root,
            state_root: tip.state_root,
            proof_root: tip.proof_root,
            evidence_root: tip.evidence_root,
            pccv_root: tip.pccv_root,
        });
        self.verified_heights.push(tip.block_height);
        Ok(())
    }

    pub fn trusted_height(&self) -> Option<u64> {
        self.trusted_checkpoint.as_ref().map(|c| c.block_height)
    }

    pub fn verify_checkpoint(checkpoint: &ConstitutionalCheckpoint) -> bool {
        checkpoint.block_height > 0
            && checkpoint.certificate_hash != [0u8; 32]
            && checkpoint.history_root != [0u8; 32]
            && checkpoint.state_root != [0u8; 32]
    }
}

impl Default for ConstitutionalLightClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_constitutional_runtime::block_validator::BlockValidationResult;
    use amun_constitutional_runtime::certificate_chain::CertificateChain;
    use amun_constitutional_runtime::finality_certificate::ConstitutionalFinalityCertificate;
    use amun_constitutional_runtime::history_root::ConstitutionalHistoryRoot;
    use amun_resource_core::ResourceId;
    use amun_transition_proof::transition_proof::TransitionProof;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32];
        h[0] = seed;
        ResourceId(h)
    }

    fn make_cert(
        height: u64,
        state_root: [u8; 32],
        qc_hash: [u8; 32],
        prev_hash: [u8; 32],
    ) -> ConstitutionalFinalityCertificate {
        let block_result = BlockValidationResult {
            total_transactions: 1,
            committed: 1,
            rejected: 0,
            pccv_verified: 1,
            pccv_failed: 0,
            block_valid: true,
            state_root,
        };
        let transitions = vec![TransitionProof::new(
            [0xaa; 32],
            make_id(1),
            height,
            [0u8; 32],
            [0u8; 32],
            state_root,
            vec![],
            vec![],
            vec![],
            vec![],
            0,
        )];
        let mut cert = ConstitutionalFinalityCertificate::issue(
            &block_result,
            transitions,
            qc_hash,
            height,
            [0xbb; 32],
        );
        cert.previous_certificate_hash = prev_hash;
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    #[test]
    fn n55_bootstrap_client() {
        let cert = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let chain = CertificateChain::new(vec![cert.clone()]).unwrap();
        let history_root = ConstitutionalHistoryRoot::from_chain(&chain);
        let checkpoint = ConstitutionalCheckpoint {
            block_height: cert.block_height,
            block_hash: cert.block_hash,
            certificate_hash: cert.certificate_hash,
            history_root: history_root.history_root,
            state_root: cert.state_root,
            proof_root: cert.proof_root,
            evidence_root: cert.evidence_root,
            pccv_root: cert.pccv_root,
        };
        let mut client = ConstitutionalLightClient::new();
        client.bootstrap(checkpoint);
        assert_eq!(client.trusted_height(), Some(1));
    }

    #[test]
    fn n55_verify_valid_certificate() {
        let client = ConstitutionalLightClient::new();
        let cert = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        assert!(client.verify_certificate(&cert));
    }

    #[test]
    fn n55_reject_tampered_certificate() {
        let client = ConstitutionalLightClient::new();
        let mut cert = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        cert.block_height = 999;
        assert!(!client.verify_certificate(&cert));
    }

    #[test]
    fn n55_verify_chain_extension() {
        let cert1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let checkpoint = ConstitutionalCheckpoint {
            block_height: cert1.block_height,
            block_hash: cert1.block_hash,
            certificate_hash: cert1.certificate_hash,
            history_root: [0u8; 32],
            state_root: cert1.state_root,
            proof_root: cert1.proof_root,
            evidence_root: cert1.evidence_root,
            pccv_root: cert1.pccv_root,
        };
        let mut client = ConstitutionalLightClient::new();
        client.bootstrap(checkpoint);
        let cert2 = make_cert(2, [0x02; 32], [0xdd; 32], cert1.certificate_hash);
        let cert3 = make_cert(3, [0x03; 32], [0xee; 32], cert2.certificate_hash);
        let chain = CertificateChain::new(vec![cert2, cert3]).unwrap();
        assert!(client.verify_chain_extension(&chain));
    }

    #[test]
    fn n55_advance_client_state() {
        let cert1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let checkpoint = ConstitutionalCheckpoint {
            block_height: cert1.block_height,
            block_hash: cert1.block_hash,
            certificate_hash: cert1.certificate_hash,
            history_root: [0u8; 32],
            state_root: cert1.state_root,
            proof_root: cert1.proof_root,
            evidence_root: cert1.evidence_root,
            pccv_root: cert1.pccv_root,
        };
        let mut client = ConstitutionalLightClient::new();
        client.bootstrap(checkpoint);
        let cert2 = make_cert(2, [0x02; 32], [0xdd; 32], cert1.certificate_hash);
        let cert3 = make_cert(3, [0x03; 32], [0xee; 32], cert2.certificate_hash);
        let chain = CertificateChain::new(vec![cert2, cert3]).unwrap();
        assert!(client.advance(&chain).is_ok());
        assert_eq!(client.trusted_height(), Some(3));
    }
}
