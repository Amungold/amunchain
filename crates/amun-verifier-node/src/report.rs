use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationReport {
    pub block_height: u64,
    pub block_hash: String,
    pub block_valid: bool,
    pub state_proofs_valid: bool,
    pub governance_proofs_valid: bool,
    pub execution_proofs_valid: bool,
    pub lineage_valid: bool,
    pub overall_valid: bool,
}

impl VerificationReport {
    pub fn new(block_height: u64, block_hash: String) -> Self {
        Self {
            block_height,
            block_hash,
            block_valid: false,
            state_proofs_valid: false,
            governance_proofs_valid: false,
            execution_proofs_valid: false,
            lineage_valid: false,
            overall_valid: false,
        }
    }

    pub fn finalize(&mut self) {
        self.overall_valid = self.block_valid
            && self.state_proofs_valid
            && self.governance_proofs_valid
            && self.execution_proofs_valid
            && self.lineage_valid;
    }
}
