// ============================================================================
// N110.3 — Certificate Gossip
// ============================================================================
// Manages SlashingCertificate propagation, validation, dedup, and storage.
// ============================================================================

use crate::slashing_certificate::SlashingCertificate;
use std::collections::HashMap;

pub struct CertificateGossip {
    pub certificates: HashMap<[u8; 32], SlashingCertificate>,
    pub broadcasted: HashMap<[u8; 32], bool>,
    pub pending: Vec<[u8; 32]>,
}

impl CertificateGossip {
    pub fn new() -> Self {
        Self {
            certificates: HashMap::new(),
            broadcasted: HashMap::new(),
            pending: Vec::new(),
        }
    }

    pub fn receive_certificate(&mut self, cert: SlashingCertificate) -> Result<bool, String> {
        cert.verify()?;
        let hash = cert.certificate_hash;
        if self.certificates.contains_key(&hash) {
            return Ok(false);
        }
        self.certificates.insert(hash, cert);
        self.pending.push(hash);
        Ok(true)
    }

    pub fn mark_broadcasted(&mut self, hash: &[u8; 32]) {
        self.broadcasted.insert(*hash, true);
    }

    pub fn should_broadcast(&self, hash: &[u8; 32]) -> bool {
        !self.broadcasted.contains_key(hash)
    }

    pub fn get_pending(&self) -> Vec<&SlashingCertificate> {
        self.pending
            .iter()
            .filter_map(|h| self.certificates.get(h))
            .collect()
    }

    pub fn mark_included(&mut self, hash: &[u8; 32]) {
        self.pending.retain(|h| h != hash);
    }

    pub fn len(&self) -> usize {
        self.certificates.len()
    }
    pub fn is_empty(&self) -> bool {
        self.certificates.is_empty()
    }
}

impl Default for CertificateGossip {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;
    use crate::misbehavior_registry::ValidatorStatus;
    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};

    fn make_test_cert(id: [u8; 32]) -> SlashingCertificate {
        SlashingCertificate::from_slash_result(
            id,
            30,
            vec![[0xA1; 32]],
            vec![EvidenceCount {
                evidence_type: EvidenceType::DoubleVote,
                count: 3,
                weight: 30,
            }],
            1500,
            15000,
            85000,
            3,
            ValidatorStatus::SlashEligible,
            100,
        )
    }

    #[test]
    fn n110_3_receive_valid_certificate() {
        let mut g = CertificateGossip::new();
        let c = make_test_cert([0x42; 32]);
        assert!(g.receive_certificate(c.clone()).unwrap());
        assert_eq!(g.len(), 1);
    }

    #[test]
    fn n110_3_duplicate_deduplicated() {
        let mut g = CertificateGossip::new();
        let c = make_test_cert([0x42; 32]);
        assert!(g.receive_certificate(c.clone()).unwrap());
        assert!(!g.receive_certificate(c).unwrap());
        assert_eq!(g.len(), 1);
    }

    #[test]
    fn n110_3_broadcast_tracking() {
        let mut g = CertificateGossip::new();
        let c = make_test_cert([0x42; 32]);
        let h = c.certificate_hash;
        g.receive_certificate(c).unwrap();
        assert!(g.should_broadcast(&h));
        g.mark_broadcasted(&h);
        assert!(!g.should_broadcast(&h));
    }

    #[test]
    fn n110_3_pending_then_included() {
        let mut g = CertificateGossip::new();
        let c = make_test_cert([0x42; 32]);
        let h = c.certificate_hash;
        g.receive_certificate(c).unwrap();
        assert_eq!(g.get_pending().len(), 1);
        g.mark_included(&h);
        assert_eq!(g.get_pending().len(), 0);
    }

    #[test]
    fn n110_3_tampered_rejected() {
        let mut g = CertificateGossip::new();
        let mut c = make_test_cert([0x42; 32]);
        c.amount_slashed = 0;
        assert!(g.receive_certificate(c).is_err());
    }
}
