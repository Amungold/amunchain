use serde::{Deserialize, Serialize};

use crate::{FailureReason, ObligationId, ObligationResultStatus};

/// The result of evaluating a single proof obligation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObligationResult {
    /// The obligation being evaluated.
    pub obligation_id: ObligationId,
    /// The outcome of the evaluation.
    pub status: ObligationResultStatus,
    /// References to the evidence that supports the result.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    /// If the status is Failed, the reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<FailureReason>,
}

impl ObligationResult {
    pub fn satisfied(id: ObligationId, evidence_refs: Vec<String>) -> Self {
        Self {
            obligation_id: id,
            status: ObligationResultStatus::Satisfied,
            evidence_refs,
            failure_reason: None,
        }
    }

    pub fn failed(id: ObligationId, reason: FailureReason, evidence_refs: Vec<String>) -> Self {
        Self {
            obligation_id: id,
            status: ObligationResultStatus::Failed,
            evidence_refs,
            failure_reason: Some(reason),
        }
    }

    pub fn inconclusive(id: ObligationId, evidence_refs: Vec<String>) -> Self {
        Self {
            obligation_id: id,
            status: ObligationResultStatus::Inconclusive,
            evidence_refs,
            failure_reason: None,
        }
    }
}
