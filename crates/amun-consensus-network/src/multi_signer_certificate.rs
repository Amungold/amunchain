use crate::slashing_certificate::SlashingCertificate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiSignerCertificate {
    pub certificate: SlashingCertificate,
    pub approver_public_keys: Vec<[u8; 32]>,
    pub approver_signatures: Vec<[u8; 64]>,
    pub quorum_threshold: usize,
    pub total_validators: usize,
}

impl MultiSignerCertificate {
    pub fn new(
        certificate: SlashingCertificate,
        quorum_threshold: usize,
        total_validators: usize,
    ) -> Self {
        Self {
            certificate,
            approver_public_keys: Vec::new(),
            approver_signatures: Vec::new(),
            quorum_threshold,
            total_validators,
        }
    }

    pub fn add_approval(
        &mut self,
        public_key: [u8; 32],
        signing_key: &ed25519_dalek::SigningKey,
    ) -> Result<(), String> {
        use ed25519_dalek::Signer;

        if self.approver_public_keys.contains(&public_key) {
            return Err("N116: validator already approved".into());
        }

        let payload = self.certificate.signing_bytes();
        let signature = signing_key.sign(&payload).to_bytes();

        self.approver_public_keys.push(public_key);
        self.approver_signatures.push(signature);

        Ok(())
    }

    pub fn verify_approvals(&self) -> Result<usize, String> {
        use ed25519_dalek::Verifier;

        if self.approver_public_keys.len() != self.approver_signatures.len() {
            return Err("N116: public keys and signatures count mismatch".into());
        }

        let payload = self.certificate.signing_bytes();
        let mut valid_count = 0;

        for (i, public_key) in self.approver_public_keys.iter().enumerate() {
            let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(public_key)
                .map_err(|e| format!("N116: invalid public key at index {}: {}", i, e))?;
            let sig = ed25519_dalek::Signature::from_bytes(&self.approver_signatures[i]);
            if verifying_key.verify(&payload, &sig).is_ok() {
                valid_count += 1;
            }
        }

        Ok(valid_count)
    }

    pub fn has_quorum(&self) -> Result<bool, String> {
        let valid = self.verify_approvals()?;
        Ok(valid >= self.quorum_threshold)
    }

    pub fn has_approved(&self, public_key: &[u8; 32]) -> bool {
        self.approver_public_keys.contains(public_key)
    }

    pub fn approval_count(&self) -> usize {
        self.approver_public_keys.len()
    }

    pub fn needs_more_approvals(&self) -> bool {
        self.approver_public_keys.len() < self.quorum_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slashing_certificate::{SlashingCertificate, EvidenceCount};
    use crate::evidence_store::EvidenceType;
    use crate::ValidatorStatus;
    use ed25519_dalek::SigningKey;

    fn make_certificate() -> SlashingCertificate {
        SlashingCertificate::from_slash_result(
            [0x42; 32], 30,
            vec![[0xA1; 32], [0xA2; 32], [0xA3; 32]],
            vec![EvidenceCount { evidence_type: EvidenceType::DoubleVote, count: 3, weight: 30 }],
            1500, 15000, 85000, 3,
            ValidatorStatus::SlashEligible, 100,
        )
    }

    fn make_key(seed: u8) -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[seed; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    #[test]
    fn n116_single_approval_works() {
        let cert = make_certificate();
        let mut ms = MultiSignerCertificate::new(cert, 3, 5);
        let (sk, pk) = make_key(1);
        ms.add_approval(pk, &sk).unwrap();
        assert_eq!(ms.approval_count(), 1);
        assert!(ms.needs_more_approvals());
        assert!(ms.has_approved(&pk));
    }

    #[test]
    fn n116_duplicate_approval_rejected() {
        let cert = make_certificate();
        let mut ms = MultiSignerCertificate::new(cert, 3, 5);
        let (sk, pk) = make_key(1);
        ms.add_approval(pk, &sk).unwrap();
        assert!(ms.add_approval(pk, &sk).is_err());
        assert_eq!(ms.approval_count(), 1);
    }

    #[test]
    fn n116_quorum_reached_with_enough_approvals() {
        let cert = make_certificate();
        let mut ms = MultiSignerCertificate::new(cert, 3, 5);
        let (sk1, pk1) = make_key(1);
        let (sk2, pk2) = make_key(2);
        let (sk3, pk3) = make_key(3);
        ms.add_approval(pk1, &sk1).unwrap();
        ms.add_approval(pk2, &sk2).unwrap();
        assert!(ms.needs_more_approvals());
        ms.add_approval(pk3, &sk3).unwrap();
        assert!(!ms.needs_more_approvals());
        assert!(ms.has_quorum().unwrap());
        assert_eq!(ms.approval_count(), 3);
    }

    #[test]
    fn n116_verify_approvals_counts_correctly() {
        let cert = make_certificate();
        let mut ms = MultiSignerCertificate::new(cert, 2, 5);
        let (sk1, pk1) = make_key(1);
        let (sk2, pk2) = make_key(2);
        ms.add_approval(pk1, &sk1).unwrap();
        ms.add_approval(pk2, &sk2).unwrap();
        let valid = ms.verify_approvals().unwrap();
        assert_eq!(valid, 2);
    }

    #[test]
    fn n116_invalid_signature_detected() {
        let cert = make_certificate();
        let mut ms = MultiSignerCertificate::new(cert, 2, 5);
        let (sk, pk) = make_key(1);
        ms.add_approval(pk, &sk).unwrap();
        ms.approver_signatures[0] = [0xFF; 64];
        let valid = ms.verify_approvals().unwrap();
        assert_eq!(valid, 0, "Tampered signature must not count as valid");
    }
}
