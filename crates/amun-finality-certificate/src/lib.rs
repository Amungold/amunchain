#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::cast_possible_truncation)]
use blake3::Hasher;
use std::collections::HashMap;

/// A constitutional finality certificate that binds consensus quorum
/// with the complete evidence chain (replay, audit, evidence root).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalFinalityCertificate {
    pub finality_id: [u8; 32],
    pub height: u64,
    pub block_hash: [u8; 32],
    pub evidence_root: [u8; 32],
    pub replay_certificate_head: [u8; 32],
    pub audit_record_head: [u8; 32],
    pub quorum_certificate_hash: [u8; 32],
    pub previous_finality: [u8; 32],
    pub timestamp: u64,
}

impl ConstitutionalFinalityCertificate {
    #[allow(clippy::too_many_arguments)]
    pub fn issue(
        height: u64,
        block_hash: [u8; 32],
        evidence_root: [u8; 32],
        replay_head: [u8; 32],
        audit_head: [u8; 32],
        qc_hash: [u8; 32],
        previous_finality: [u8; 32],
        timestamp: u64,
    ) -> Self {
        let mut cert = Self {
            finality_id: [0u8; 32],
            height,
            block_hash,
            evidence_root,
            replay_certificate_head: replay_head,
            audit_record_head: audit_head,
            quorum_certificate_hash: qc_hash,
            previous_finality,
            timestamp,
        };
        cert.finality_id = cert.compute_id();
        cert
    }

    fn compute_id(&self) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_CONSTITUTIONAL_FINALITY_V1");
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&self.evidence_root);
        hasher.update(&self.replay_certificate_head);
        hasher.update(&self.audit_record_head);
        hasher.update(&self.quorum_certificate_hash);
        hasher.update(&self.previous_finality);
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn verify(&self) -> bool {
        self.finality_id == self.compute_id()
            && self.block_hash != [0u8; 32]
            && self.evidence_root != [0u8; 32]
    }
}

/// A verifiable chain of constitutional finality certificates.
#[derive(Debug, Clone, Default)]
pub struct FinalityChain {
    certificates: HashMap<[u8; 32], ConstitutionalFinalityCertificate>,
    latest_id: Option<[u8; 32]>,
}

impl FinalityChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, cert: ConstitutionalFinalityCertificate) -> Result<(), &'static str> {
        if !cert.verify() {
            return Err("Certificate verification failed");
        }
        if let Some(ref latest) = self.latest_id {
            if cert.previous_finality != *latest {
                return Err("Finality chain broken");
            }
        }
        self.latest_id = Some(cert.finality_id);
        self.certificates.insert(cert.finality_id, cert);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.certificates.is_empty()
    }
    pub fn len(&self) -> usize {
        self.certificates.len()
    }
    pub fn latest(&self) -> Option<&ConstitutionalFinalityCertificate> {
        self.latest_id.and_then(|id| self.certificates.get(&id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issue_cert(h: u64) -> ConstitutionalFinalityCertificate {
        ConstitutionalFinalityCertificate::issue(
            h,
            [h as u8; 32],
            [h as u8; 32],
            [h as u8; 32],
            [h as u8; 32],
            [h as u8; 32],
            [0u8; 32],
            h * 1000,
        )
    }

    #[test]
    fn n41_single_finality_certificate() {
        let cert = issue_cert(1);
        assert!(cert.verify());
        assert_ne!(cert.finality_id, [0u8; 32]);
    }

    #[test]
    fn n41_hash_deterministic() {
        let c1 = issue_cert(1);
        let c2 = issue_cert(1);
        assert_eq!(c1.finality_id, c2.finality_id);
    }

    #[test]
    fn n41_tampered_evidence_rejected() {
        let mut cert = issue_cert(1);
        cert.evidence_root = [0xFF; 32];
        assert!(!cert.verify());
    }

    #[test]
    fn n41_zero_block_hash_rejected() {
        let mut cert = issue_cert(1);
        cert.block_hash = [0u8; 32];
        assert!(!cert.verify());
    }

    #[test]
    fn n41_finality_chain_continuity() {
        let mut chain = FinalityChain::new();
        let c1 = ConstitutionalFinalityCertificate::issue(
            1, [1u8; 32], [1u8; 32], [1u8; 32], [1u8; 32], [1u8; 32], [0u8; 32], 1000,
        );
        chain.append(c1).unwrap();
        let prev = chain.latest().unwrap().finality_id;
        let c2 = ConstitutionalFinalityCertificate::issue(
            2, [2u8; 32], [2u8; 32], [2u8; 32], [2u8; 32], [2u8; 32], prev, 2000,
        );
        assert!(chain.append(c2).is_ok());
        assert_eq!(chain.len(), 2);
    }
}
