use crate::ObligationId;
use serde::{Deserialize, Serialize};

/// Lifecycle status of an obligation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deprecated")]
    Deprecated { superseded_by: ObligationId },
    #[serde(rename = "frozen")]
    Frozen,
}
