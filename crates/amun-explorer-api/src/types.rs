use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSummary {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub state_root: String,
    pub timestamp: u64,
    pub transaction_count: usize,
    pub has_finality_certificate: bool,
    pub has_replay_evidence: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub hash: String,
    pub block_height: u64,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    pub address: String,
    pub balance: u64,
    pub nonce: u64,
    pub transaction_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalityCertificateSummary {
    pub certificate_id: String,
    pub block_hash: String,
    pub block_height: u64,
    pub quorum_size: usize,
    pub signed_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalVerdictSummary {
    pub verdict_id: String,
    pub phase: String,
    pub obligations_checked: u64,
    pub obligations_satisfied: u64,
    pub overall_result: String,
    pub issued_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRecordSummary {
    pub evidence_id: String,
    pub evidence_type: String,
    pub phase: String,
    pub status: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalDashboard {
    pub total_obligations: usize,
    pub total_verdicts: usize,
    pub total_evidence: usize,
    pub phases_verified: Vec<String>,
    pub overall_status: String,
}
