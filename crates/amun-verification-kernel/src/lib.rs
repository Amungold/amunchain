use blake3::Hasher;
use serde::{Deserialize, Serialize};

/// A verifiable claim about the system's constitutional properties.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstitutionalClaim {
    pub claim_id: String,
    pub claim_type: ClaimType,
    pub description: String,
    pub phase: String,
    pub evidence_refs: Vec<String>,
    pub status: ClaimStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimType {
    Safety,
    Liveness,
    Finality,
    Integrity,
    Recovery,
    Performance,
    Security,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimStatus {
    Proven,
    Validated,
    Assumed,
    Open,
    Disputed,
}

/// Evidence that supports or refutes a claim.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Evidence {
    pub evidence_id: String,
    pub claim_id: String,
    pub evidence_type: EvidenceType,
    pub description: String,
    pub data_hash: String,
    pub source: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EvidenceType {
    TestResult,
    FormalProof,
    Benchmark,
    Simulation,
    AuditFinding,
    CoverageReport,
}

/// A certificate binding claims to evidence with cryptographic integrity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationCertificate {
    pub certificate_id: String,
    pub phase: String,
    pub claims: Vec<ConstitutionalClaim>,
    pub evidence: Vec<Evidence>,
    pub certificate_hash: String,
    pub issued_at: u64,
    pub verifier: String,
}

impl VerificationCertificate {
    /// Issue a new verification certificate for a phase.
    pub fn issue(
        phase: &str,
        claims: Vec<ConstitutionalClaim>,
        evidence: Vec<Evidence>,
        verifier: &str,
        timestamp: u64,
    ) -> Self {
        let mut cert = Self {
            certificate_id: String::new(),
            phase: phase.to_string(),
            claims,
            evidence,
            certificate_hash: String::new(),
            issued_at: timestamp,
            verifier: verifier.to_string(),
        };
        cert.certificate_id = cert.compute_id();
        cert.certificate_hash = cert.compute_hash();
        cert
    }

    fn compute_id(&self) -> String {
        let mut hasher = Hasher::new();
        hasher.update(b"AMUN_VERIFICATION_CERTIFICATE_V1");
        hasher.update(self.phase.as_bytes());
        hasher.update(&self.issued_at.to_le_bytes());
        hasher.update(self.verifier.as_bytes());
        for claim in &self.claims {
            hasher.update(claim.claim_id.as_bytes());
        }
        hex::encode(hasher.finalize().as_bytes())
    }

    fn compute_hash(&self) -> String {
        let mut hasher = Hasher::new();
        hasher.update(self.certificate_id.as_bytes());
        for claim in &self.claims {
            hasher.update(claim.claim_id.as_bytes());
            hasher.update(claim.description.as_bytes());
        }
        for ev in &self.evidence {
            hasher.update(ev.evidence_id.as_bytes());
            hasher.update(ev.data_hash.as_bytes());
        }
        hex::encode(hasher.finalize().as_bytes())
    }

    /// Verify the certificate's integrity.
    pub fn verify(&self) -> bool {
        self.certificate_id == self.compute_id() && self.certificate_hash == self.compute_hash()
    }

    /// Count claims by status.
    pub fn count_by_status(&self, status: ClaimStatus) -> usize {
        self.claims.iter().filter(|c| c.status == status).count()
    }
}

/// Registry of all verification certificates.
#[derive(Debug, Clone, Default)]
pub struct VerificationRegistry {
    pub certificates: Vec<VerificationCertificate>,
}

impl VerificationRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, cert: VerificationCertificate) -> Result<(), String> {
        if !cert.verify() {
            return Err("Certificate verification failed".into());
        }
        self.certificates.push(cert);
        Ok(())
    }

    pub fn phase_certificate(&self, phase: &str) -> Option<&VerificationCertificate> {
        self.certificates.iter().find(|c| c.phase == phase)
    }

    pub fn count(&self) -> usize {
        self.certificates.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_claim(id: &str, phase: &str, status: ClaimStatus) -> ConstitutionalClaim {
        ConstitutionalClaim {
            claim_id: id.into(),
            claim_type: ClaimType::Safety,
            description: format!("Claim {} for {}", id, phase),
            phase: phase.into(),
            evidence_refs: vec![format!("ev-{}", id)],
            status,
        }
    }

    fn make_evidence(id: &str, claim_id: &str) -> Evidence {
        Evidence {
            evidence_id: id.into(),
            claim_id: claim_id.into(),
            evidence_type: EvidenceType::TestResult,
            description: format!("Evidence {} for {}", id, claim_id),
            data_hash: format!("hash-{}", id),
            source: "cargo test".into(),
            timestamp: 1000,
        }
    }

    #[test]
    fn n46_5_issue_certificate() {
        let claims = vec![
            make_claim("C1", "N46", ClaimStatus::Proven),
            make_claim("C2", "N46", ClaimStatus::Validated),
        ];
        let evidence = vec![make_evidence("E1", "C1"), make_evidence("E2", "C2")];
        let cert = VerificationCertificate::issue("N46", claims, evidence, "verifier-1", 1000);
        assert!(cert.verify());
        assert_eq!(cert.count_by_status(ClaimStatus::Proven), 1);
        assert_eq!(cert.count_by_status(ClaimStatus::Validated), 1);
    }

    #[test]
    fn n46_5_tampered_certificate_rejected() {
        let mut cert = VerificationCertificate::issue(
            "N46",
            vec![make_claim("C1", "N46", ClaimStatus::Proven)],
            vec![make_evidence("E1", "C1")],
            "verifier-1",
            1000,
        );
        cert.claims[0].description = "tampered".into();
        assert!(!cert.verify());
    }

    #[test]
    fn n46_5_registry_accepts_valid_certificates() {
        let mut registry = VerificationRegistry::new();
        let cert = VerificationCertificate::issue(
            "N46",
            vec![make_claim("C1", "N46", ClaimStatus::Proven)],
            vec![make_evidence("E1", "C1")],
            "verifier-1",
            1000,
        );
        assert!(registry.register(cert).is_ok());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn n46_5_registry_rejects_invalid_certificates() {
        let mut registry = VerificationRegistry::new();
        let mut cert = VerificationCertificate::issue(
            "N46",
            vec![make_claim("C1", "N46", ClaimStatus::Proven)],
            vec![make_evidence("E1", "C1")],
            "verifier-1",
            1000,
        );
        cert.claims[0].description = "broken".into();
        assert!(registry.register(cert).is_err());
    }

    #[test]
    fn n46_5_phase_lookup() {
        let mut registry = VerificationRegistry::new();
        let cert = VerificationCertificate::issue(
            "N46",
            vec![make_claim("C1", "N46", ClaimStatus::Proven)],
            vec![make_evidence("E1", "C1")],
            "verifier-1",
            1000,
        );
        registry.register(cert).unwrap();
        let found = registry.phase_certificate("N46");
        assert!(found.is_some());
        assert_eq!(found.unwrap().phase, "N46");
        assert!(registry.phase_certificate("N99").is_none());
    }
}
