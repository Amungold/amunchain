use crate::commitment::ConstitutionalCommitment;
use crate::Hash32;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ConstitutionalStatus {
    pub height: u64,
    pub version: u16,
    pub identity_root: String,
    pub evidence_root: String,
    pub governance_root: String,
    pub economic_root: String,
    pub constitutional_root: String,
    pub app_hash: String,
}

fn hash_to_hex(hash: &Hash32) -> String {
    format!("0x{}", hex::encode(hash))
}

impl ConstitutionalStatus {
    pub fn new(height: u64, commitment: &ConstitutionalCommitment, app_hash: Hash32) -> Self {
        Self {
            height,
            version: commitment.version,
            identity_root: hash_to_hex(&commitment.identity_root),
            evidence_root: hash_to_hex(&commitment.evidence_root),
            governance_root: hash_to_hex(&commitment.governance_root),
            economic_root: hash_to_hex(&commitment.economic_root),
            constitutional_root: hash_to_hex(&commitment.constitutional_root),
            app_hash: hash_to_hex(&app_hash),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}
