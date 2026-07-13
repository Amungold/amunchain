use serde::{Deserialize, Serialize};

/// Lifecycle status of a piece of constitutional evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStatus {
    #[serde(rename = "collected")]
    Collected,
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "archived")]
    Archived,
    #[serde(rename = "superseded")]
    Superseded,
    #[serde(rename = "rejected")]
    Rejected,
}
