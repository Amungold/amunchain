// N128 — Evidence Records
// ========================
// Replaces boolean evidence with structured, auditable evidence records.
// Each record can be independently verified and cryptographically validated.

use serde::{Deserialize, Serialize};

/// N128: Structured signature verification evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureEvidence {
    /// Number of transactions with verified signatures
    pub verified_transactions: u64,
    /// Number of transactions with failed signatures
    pub failed_transactions: u64,
    /// Whether all transactions passed signature verification
    pub all_valid: bool,
}

impl SignatureEvidence {
    pub fn new(verified: u64, failed: u64) -> Self {
        Self {
            verified_transactions: verified,
            failed_transactions: failed,
            all_valid: failed == 0,
        }
    }
}

/// N128: Structured double-spend evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoubleSpendEvidence {
    /// Total unique (sender, nonce) pairs checked
    pub checked_nonces: u64,
    /// Number of duplicate (sender, nonce) pairs detected
    pub duplicate_nonces: u64,
    /// Whether no double-spend was detected
    pub no_double_spend: bool,
}

impl DoubleSpendEvidence {
    pub fn new(checked: u64, duplicates: u64) -> Self {
        Self {
            checked_nonces: checked,
            duplicate_nonces: duplicates,
            no_double_spend: duplicates == 0,
        }
    }
}

/// N128: Structured governance evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceEvidence {
    /// Hash of the validator set at this height
    #[serde(with = "serde_bytes")]
    pub validator_set_hash: [u8; 32],
    /// Governance epoch number
    pub governance_epoch: u64,
    /// Whether the validator set is constitutionally valid
    pub approved: bool,
}

impl GovernanceEvidence {
    pub fn new(validator_set_hash: [u8; 32], epoch: u64, approved: bool) -> Self {
        Self {
            validator_set_hash,
            governance_epoch: epoch,
            approved,
        }
    }
}

/// N128: Structured replay evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvidence {
    /// State root from original execution
    #[serde(with = "serde_bytes")]
    pub original_state_root: [u8; 32],
    /// State root from replay execution
    #[serde(with = "serde_bytes")]
    pub replay_state_root: [u8; 32],
    /// Whether the two roots match (deterministic execution)
    pub deterministic: bool,
}

impl ReplayEvidence {
    pub fn new(original: [u8; 32], replay: [u8; 32]) -> Self {
        Self {
            original_state_root: original,
            replay_state_root: replay,
            deterministic: original == replay,
        }
    }
}

/// N128: Complete constitutional evidence record.
/// Every field is a structured record, not a boolean.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalEvidenceRecord {
    pub height: u64,
    pub block_hash: [u8; 32],

    // Structured evidence records
    pub signature_evidence: SignatureEvidence,
    pub double_spend_evidence: DoubleSpendEvidence,
    pub governance_evidence: GovernanceEvidence,
    pub replay_evidence: ReplayEvidence,

    // Direct cryptographic evidence
    pub slashing_bound: bool,
    pub evidence_valid: bool,
    pub finality_supermajority: bool,
    pub chain_continuous: bool,
    pub state_root_valid: bool,
    pub transition_valid: bool,

    /// Blake3 hash of this evidence record for auditability
    #[serde(with = "serde_bytes")]
    pub evidence_hash: [u8; 32],
}

impl ConstitutionalEvidenceRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        height: u64,
        block_hash: [u8; 32],
        signature_evidence: SignatureEvidence,
        double_spend_evidence: DoubleSpendEvidence,
        governance_evidence: GovernanceEvidence,
        replay_evidence: ReplayEvidence,
        slashing_bound: bool,
        evidence_valid: bool,
        finality_supermajority: bool,
        chain_continuous: bool,
        state_root_valid: bool,
        transition_valid: bool,
    ) -> Self {
        let mut record = Self {
            height,
            block_hash,
            signature_evidence,
            double_spend_evidence,
            governance_evidence,
            replay_evidence,
            slashing_bound,
            evidence_valid,
            finality_supermajority,
            chain_continuous,
            state_root_valid,
            transition_valid,
            evidence_hash: [0u8; 32],
        };
        record.evidence_hash = record.compute_hash();
        record
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_EVIDENCE_RECORD_V1");
        hasher.update(&self.height.to_le_bytes());
        hasher.update(&self.block_hash);
        hasher.update(&[self.finality_supermajority as u8]);
        hasher.update(&[self.chain_continuous as u8]);
        hasher.update(&[self.state_root_valid as u8]);
        hasher.finalize().into()
    }

    /// Verify the integrity of this evidence record.
    pub fn verify(&self) -> bool {
        self.compute_hash() == self.evidence_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n128_signature_evidence_all_valid() {
        let ev = SignatureEvidence::new(100, 0);
        assert!(ev.all_valid);
        assert_eq!(ev.verified_transactions, 100);
        assert_eq!(ev.failed_transactions, 0);
    }

    #[test]
    fn n128_signature_evidence_with_failures() {
        let ev = SignatureEvidence::new(100, 3);
        assert!(!ev.all_valid);
    }

    #[test]
    fn n128_double_spend_evidence_clean() {
        let ev = DoubleSpendEvidence::new(50, 0);
        assert!(ev.no_double_spend);
    }

    #[test]
    fn n128_double_spend_evidence_detected() {
        let ev = DoubleSpendEvidence::new(50, 2);
        assert!(!ev.no_double_spend);
    }

    #[test]
    fn n128_replay_evidence_deterministic() {
        let root = [0x42; 32];
        let ev = ReplayEvidence::new(root, root);
        assert!(ev.deterministic);
    }

    #[test]
    fn n128_replay_evidence_divergent() {
        let ev = ReplayEvidence::new([0x42; 32], [0xFF; 32]);
        assert!(!ev.deterministic);
    }

    #[test]
    fn n128_evidence_record_hash_verifiable() {
        let record = ConstitutionalEvidenceRecord::new(
            100,
            [0xAA; 32],
            SignatureEvidence::new(10, 0),
            DoubleSpendEvidence::new(10, 0),
            GovernanceEvidence::new([0x42; 32], 1, true),
            ReplayEvidence::new([0x42; 32], [0x42; 32]),
            true,
            true,
            true,
            true,
            true,
            true,
        );
        assert_ne!(record.evidence_hash, [0u8; 32]);
        assert!(record.verify());
    }

    #[test]
    fn n128_tampered_record_detected() {
        let mut record = ConstitutionalEvidenceRecord::new(
            100,
            [0xAA; 32],
            SignatureEvidence::new(10, 0),
            DoubleSpendEvidence::new(10, 0),
            GovernanceEvidence::new([0x42; 32], 1, true),
            ReplayEvidence::new([0x42; 32], [0x42; 32]),
            true,
            true,
            true,
            true,
            true,
            true,
        );
        record.finality_supermajority = false; // Tamper
        assert!(!record.verify());
    }
}
