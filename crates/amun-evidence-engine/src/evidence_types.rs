use amun_resource_core::ResourceId;
use serde::{Deserialize, Serialize};

/// The three constitutional evidence categories from N48.5-E Section 2.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstitutionalEvidence {
    /// Operational failure — out of gas, panic, handle leak, etc.
    ExecutionFailure {
        reason: String,
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
        gas_consumed: u64,
    },
    /// Resource law violation — R1-R6, L1-L5, T1.
    ConstitutionalViolation {
        law: String,
        resource_ids: Vec<ResourceId>,
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
    },
    /// Contract invariant failure — evaluated post-commit.
    InvariantViolation {
        obligation_id: String,
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
        state_root: [u8; 32],
    },
}

impl ConstitutionalEvidence {
    pub fn evidence_id(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_EVIDENCE_V1");
        match self {
            Self::ExecutionFailure { transaction_hash, .. } => {
                hasher.update(b"EXEC_FAILURE");
                hasher.update(transaction_hash);
            }
            Self::ConstitutionalViolation { law, resource_ids, transaction_hash, .. } => {
                hasher.update(b"CONST_VIOLATION");
                hasher.update(law.as_bytes());
                hasher.update(transaction_hash);
                for id in resource_ids {
                    hasher.update(id.as_bytes());
                }
            }
            Self::InvariantViolation { obligation_id, transaction_hash, .. } => {
                hasher.update(b"INVARIANT_VIOLATION");
                hasher.update(obligation_id.as_bytes());
                hasher.update(transaction_hash);
            }
        }
        let hash = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(hash.as_bytes());
        bytes
    }

    pub fn category(&self) -> &str {
        match self {
            Self::ExecutionFailure { .. } => "execution_failure",
            Self::ConstitutionalViolation { .. } => "constitutional_violation",
            Self::InvariantViolation { .. } => "invariant_violation",
        }
    }

    pub fn causes_revert(&self) -> bool {
        matches!(self, Self::ExecutionFailure { .. } | Self::ConstitutionalViolation { .. })
    }
}
