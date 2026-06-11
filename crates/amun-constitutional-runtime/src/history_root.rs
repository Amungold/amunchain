use crate::certificate_chain::CertificateChain;
use serde::{Deserialize, Serialize};

/// A cryptographic commitment to the entire constitutional history.
/// Enables verification of the complete chain state from a single root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalHistoryRoot {
    pub history_root: [u8; 32],
    pub chain_length: usize,
    pub genesis_block: u64,
    pub latest_block: u64,
    pub total_transactions: usize,
    pub total_evidence: usize,
    pub all_constitutional: bool,
}

impl ConstitutionalHistoryRoot {
    /// Compute the history root from a certificate chain.
    pub fn from_chain(chain: &CertificateChain) -> Self {
        let total_transactions: usize =
            chain.certificates.iter().map(|c| c.transitions.len()).sum();

        let total_evidence: usize = chain
            .certificates
            .iter()
            .map(|c| {
                c.transitions
                    .iter()
                    .map(|t| t.evidence.len())
                    .sum::<usize>()
            })
            .sum();

        let genesis_block = chain.genesis().block_height;
        let latest_block = chain.tip().block_height;
        let all_constitutional = chain.is_fully_constitutional();

        let history_root = Self::compute_history_root(chain);

        Self {
            history_root,
            chain_length: chain.chain_length,
            genesis_block,
            latest_block,
            total_transactions,
            total_evidence,
            all_constitutional,
        }
    }

    /// Verify that a chain matches this history root.
    pub fn verify_chain(&self, chain: &CertificateChain) -> bool {
        if !chain.verify_chain() {
            return false;
        }
        if chain.chain_length != self.chain_length {
            return false;
        }
        if chain.genesis().block_height != self.genesis_block {
            return false;
        }
        if chain.tip().block_height != self.latest_block {
            return false;
        }
        Self::compute_history_root(chain) == self.history_root
    }

    fn compute_history_root(chain: &CertificateChain) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_HISTORY_ROOT_V1");
        hasher.update(&chain.chain_root);
        hasher.update(&chain.genesis().certificate_hash);
        hasher.update(&chain.tip().certificate_hash);
        hasher.update(&chain.chain_length.to_le_bytes());
        for cert in &chain.certificates {
            hasher.update(&cert.state_root);
            hasher.update(&cert.proof_root);
            hasher.update(&cert.evidence_root);
            hasher.update(&cert.pccv_root);
        }
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_validator::BlockValidationResult;
    use crate::certificate_chain::CertificateChain;
    use crate::finality_certificate::ConstitutionalFinalityCertificate;
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
    fn n54_compute_history_root() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);
        let c3 = make_cert(3, [0x03; 32], [0xee; 32], c2.certificate_hash);

        let chain = CertificateChain::new(vec![c1, c2, c3]).unwrap();
        let root = ConstitutionalHistoryRoot::from_chain(&chain);

        assert_eq!(root.chain_length, 3);
        assert_eq!(root.genesis_block, 1);
        assert_eq!(root.latest_block, 3);
        assert!(root.all_constitutional);
        assert_ne!(root.history_root, [0u8; 32]);
    }

    #[test]
    fn n54_history_root_deterministic() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);

        let chain1 = CertificateChain::new(vec![c1.clone(), c2.clone()]).unwrap();
        let chain2 = CertificateChain::new(vec![c1, c2]).unwrap();

        let root1 = ConstitutionalHistoryRoot::from_chain(&chain1);
        let root2 = ConstitutionalHistoryRoot::from_chain(&chain2);

        assert_eq!(root1.history_root, root2.history_root);
    }

    #[test]
    fn n54_verify_chain_against_root() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);

        let chain = CertificateChain::new(vec![c1, c2]).unwrap();
        let root = ConstitutionalHistoryRoot::from_chain(&chain);

        assert!(root.verify_chain(&chain));
    }

    #[test]
    fn n54_reject_tampered_chain() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);

        let chain = CertificateChain::new(vec![c1, c2.clone()]).unwrap();
        let root = ConstitutionalHistoryRoot::from_chain(&chain);

        // Tamper with the chain
        let mut tampered_chain = chain.clone();
        tampered_chain.certificates[0].block_height = 999;

        assert!(!root.verify_chain(&tampered_chain));
    }
}
