use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificateChain {
    pub certificates: Vec<crate::finality_certificate::ConstitutionalFinalityCertificate>,
    pub chain_root: [u8; 32],
    pub chain_length: usize,
}

impl CertificateChain {
    pub fn new(
        certificates: Vec<crate::finality_certificate::ConstitutionalFinalityCertificate>,
    ) -> Result<Self, String> {
        if certificates.is_empty() {
            return Err("Certificate chain cannot be empty".into());
        }
        if !certificates[0].verify() {
            return Err("First certificate is invalid".into());
        }
        for i in 1..certificates.len() {
            if !certificates[i].verify() {
                return Err(format!("Certificate at index {} is invalid", i));
            }
            let expected_prev = certificates[i - 1].certificate_hash;
            let actual_prev = certificates[i].previous_certificate_hash;
            if actual_prev != expected_prev {
                return Err(format!(
                    "Chain broken at index {}: expected previous hash {}, got {}",
                    i,
                    hex::encode(expected_prev),
                    hex::encode(actual_prev),
                ));
            }
        }
        let chain_root = Self::compute_chain_root(&certificates);
        let len = certificates.len();
        Ok(Self {
            certificates,
            chain_root,
            chain_length: len,
        })
    }

    pub fn verify_chain(&self) -> bool {
        if self.certificates.is_empty() {
            return false;
        }
        for cert in &self.certificates {
            if !cert.verify() {
                return false;
            }
        }
        for i in 1..self.certificates.len() {
            if self.certificates[i].previous_certificate_hash
                != self.certificates[i - 1].certificate_hash
            {
                return false;
            }
        }
        Self::compute_chain_root(&self.certificates) == self.chain_root
    }

    pub fn is_fully_constitutional(&self) -> bool {
        self.verify_chain()
            && self
                .certificates
                .iter()
                .all(|c| c.is_constitutionally_valid())
    }

    pub fn tip(&self) -> &crate::finality_certificate::ConstitutionalFinalityCertificate {
        &self.certificates[self.certificates.len() - 1]
    }

    pub fn genesis(&self) -> &crate::finality_certificate::ConstitutionalFinalityCertificate {
        &self.certificates[0]
    }

    fn compute_chain_root(
        certs: &[crate::finality_certificate::ConstitutionalFinalityCertificate],
    ) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CERTIFICATE_CHAIN_V1");
        for cert in certs {
            hasher.update(&cert.certificate_hash);
            hasher.update(&cert.block_hash);
            hasher.update(&cert.state_root);
            hasher.update(&cert.previous_certificate_hash);
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
    fn n53_create_certificate_chain() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);
        let c3 = make_cert(3, [0x03; 32], [0xee; 32], c2.certificate_hash);
        let chain = CertificateChain::new(vec![c1, c2, c3]).unwrap();
        assert_eq!(chain.chain_length, 3);
        assert!(chain.verify_chain());
    }

    #[test]
    fn n53_chain_deterministic_root() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);
        let r1 = CertificateChain::new(vec![c1.clone(), c2.clone()])
            .unwrap()
            .chain_root;
        let r2 = CertificateChain::new(vec![c1, c2]).unwrap().chain_root;
        assert_eq!(r1, r2);
    }

    #[test]
    fn n53_chain_rejects_invalid_certificate() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let mut c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);
        c2.block_height = 999;
        assert!(CertificateChain::new(vec![c1, c2]).is_err());
    }

    #[test]
    fn n53_chain_tampering_detected() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);
        let mut chain = CertificateChain::new(vec![c1, c2]).unwrap();
        chain.certificates[0].block_height = 999;
        assert!(!chain.verify_chain());
    }

    #[test]
    fn n53a_broken_chain_detected() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);
        let c3 = make_cert(3, [0x03; 32], [0xee; 32], c1.certificate_hash);
        assert!(CertificateChain::new(vec![c1, c2, c3]).is_err());
    }

    #[test]
    fn n53a_replaced_certificate_detected() {
        let c1 = make_cert(1, [0x01; 32], [0xcc; 32], [0u8; 32]);
        let c2 = make_cert(2, [0x02; 32], [0xdd; 32], c1.certificate_hash);
        let c3 = make_cert(3, [0x03; 32], [0xee; 32], c2.certificate_hash);
        let mut chain = CertificateChain::new(vec![c1, c2, c3]).unwrap();
        let fake = make_cert(
            2,
            [0xff; 32],
            [0xdd; 32],
            chain.certificates[0].certificate_hash,
        );
        chain.certificates[1] = fake;
        assert!(!chain.verify_chain());
    }
}
