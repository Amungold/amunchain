// N119 — Deterministic Slashing Ledger & Replay Protection
// ==========================================================
// Prevents the same SlashingCertificate from being executed
// more than once, ensuring deterministic slashing across
// all nodes.

use crate::slashing_certificate::SlashingCertificate;
use std::collections::HashSet;

/// N119.3: Computed from canonical certificate data.
pub fn certificate_id(cert: &SlashingCertificate) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"AMUN_SLASH_LEDGER_V1");
    hasher.update(&cert.validator_id);
    hasher.update(&cert.score.to_le_bytes());
    hasher.update(&cert.penalty_bps.to_le_bytes());
    hasher.update(&cert.amount_slashed.to_le_bytes());
    hasher.update(&cert.executed_at_height.to_le_bytes());
    hasher.update(&cert.timestamp.to_le_bytes());
    for id in &cert.evidence_ids {
        hasher.update(id);
    }
    hasher.update(&cert.signer_public_key);
    hasher.update(&cert.signature);
    hasher.finalize().into()
}

/// N119.5: Record of an executed slash for auditability.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ExecutedSlash {
    #[serde(with = "serde_bytes")]
    pub certificate_id: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub validator_id: [u8; 32],
    pub amount: u64,
    pub height: u64,
    pub timestamp: u64,
}

/// N119.1: Persistent ledger preventing replay of slashing certificates.
#[derive(Debug, Clone)]
pub struct SlashingLedger {
    executed_ids: HashSet<[u8; 32]>,
    /// N119.5: Audit trail of all executed slashes.
    pub history: Vec<ExecutedSlash>,
}

impl SlashingLedger {
    pub fn new() -> Self {
        Self {
            executed_ids: HashSet::new(),
            history: Vec::new(),
        }
    }

    /// N119.1: Check if a certificate has already been executed.
    pub fn is_executed(&self, id: &[u8; 32]) -> bool {
        self.executed_ids.contains(id)
    }

    /// N119.2: Execute a slash and record it in the ledger.
    /// Returns Err if the certificate was already executed (replay protection).
    pub fn execute<F, T>(&mut self, cert: &SlashingCertificate, execute_fn: F) -> Result<T, String>
    where
        F: FnOnce() -> Result<T, String>,
    {
        let id = certificate_id(cert);

        // N119.2: Replay protection
        if self.executed_ids.contains(&id) {
            return Err(format!(
                "N119: certificate already executed: {:02x?}",
                &id[..4]
            ));
        }

        // Execute the slash
        let result = execute_fn()?;

        // N119.1: Record execution
        self.executed_ids.insert(id);

        // N119.5: Record in audit trail
        self.history.push(ExecutedSlash {
            certificate_id: id,
            validator_id: cert.validator_id,
            amount: cert.amount_slashed,
            height: cert.executed_at_height,
            timestamp: cert.timestamp,
        });

        Ok(result)
    }

    /// N119.1: Get the number of executed slashes.
    pub fn executed_count(&self) -> usize {
        self.executed_ids.len()
    }

    /// N119.5: Get all executed slashes for a validator.
    pub fn history_for(&self, validator_id: &[u8; 32]) -> Vec<&ExecutedSlash> {
        self.history
            .iter()
            .filter(|s| s.validator_id == *validator_id)
            .collect()
    }
}

impl Default for SlashingLedger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_store::EvidenceType;
    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
    use crate::ValidatorStatus;

    fn make_cert(id: u64) -> SlashingCertificate {
        SlashingCertificate::from_slash_result(
            [0x42; 32],
            30,
            vec![[0xA1; 32], [0xA2; 32], [0xA3; 32]],
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
            100 + id,
        )
    }

    #[test]
    fn n119_1_certificate_id_stable() {
        let cert = make_cert(0);
        let id1 = certificate_id(&cert);
        let id2 = certificate_id(&cert);
        assert_eq!(
            id1, id2,
            "N119.1 FAIL: certificate_id must be deterministic"
        );
        assert_ne!(
            id1, [0u8; 32],
            "N119.1 FAIL: certificate_id must not be zero"
        );
    }

    #[test]
    fn n119_2_replay_rejected() {
        let cert = make_cert(0);
        let mut ledger = SlashingLedger::new();
        let id = certificate_id(&cert);

        // First execution: success
        let r1 = ledger.execute(&cert, || Ok("executed"));
        assert!(r1.is_ok(), "N119.2 FAIL: first execution must succeed");
        assert!(ledger.is_executed(&id));

        // Second execution: rejected (replay protection)
        let r2 = ledger.execute(&cert, || Ok("executed again"));
        assert!(r2.is_err(), "N119.2 FAIL: replay must be rejected");
        assert!(r2.unwrap_err().contains("already executed"));
        assert_eq!(ledger.executed_count(), 1);
    }

    #[test]
    fn n119_3_ledger_records_execution() {
        let cert = make_cert(0);
        let mut ledger = SlashingLedger::new();

        ledger.execute(&cert, || Ok(())).unwrap();
        assert_eq!(ledger.executed_count(), 1);
        assert_eq!(ledger.history.len(), 1);
        assert_eq!(ledger.history[0].validator_id, [0x42; 32]);
        assert_eq!(ledger.history[0].amount, 15000);
    }

    #[test]
    fn n119_4_different_certificates_different_ids() {
        let cert1 = make_cert(0);
        let cert2 = make_cert(1);
        let id1 = certificate_id(&cert1);
        let id2 = certificate_id(&cert2);
        assert_ne!(
            id1, id2,
            "N119.4 FAIL: different certificates must have different IDs"
        );
    }

    #[test]
    fn n119_5_duplicate_execution_no_effect() {
        let cert = make_cert(0);
        let mut ledger = SlashingLedger::new();

        // First: success
        ledger.execute(&cert, || Ok(100u64)).unwrap();

        // Second: error, no change to ledger
        let r2 = ledger.execute(&cert, || Ok(200u64));
        assert!(r2.is_err());
        assert_eq!(
            ledger.executed_count(),
            1,
            "N119.5 FAIL: duplicate must not increase count"
        );
        assert_eq!(ledger.history.len(), 1);
    }

    #[test]
    fn n119_5_audit_trail_by_validator() {
        let cert1 = make_cert(0);
        let cert2 = SlashingCertificate::from_slash_result(
            [0x99; 32],
            20,
            vec![[0xB1; 32]],
            vec![EvidenceCount {
                evidence_type: EvidenceType::InvalidSignature,
                count: 1,
                weight: 2,
            }],
            500,
            5000,
            95000,
            1,
            ValidatorStatus::Warned,
            200,
        );
        let mut ledger = SlashingLedger::new();

        ledger.execute(&cert1, || Ok(())).unwrap();
        ledger.execute(&cert2, || Ok(())).unwrap();

        let v1 = ledger.history_for(&[0x42; 32]);
        let v2 = ledger.history_for(&[0x99; 32]);
        assert_eq!(v1.len(), 1);
        assert_eq!(v2.len(), 1);
        assert_eq!(v1[0].validator_id, [0x42; 32]);
        assert_eq!(v2[0].validator_id, [0x99; 32]);
    }
}
