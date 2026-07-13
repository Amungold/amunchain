use serde::{Deserialize, Serialize};

/// The overall result of a constitutional verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerdictResult {
    /// All critical and major obligations are satisfied.
    #[serde(rename = "pass")]
    Pass,
    /// All critical obligations are satisfied, but some major/minor
    /// obligations failed or are inconclusive. Conditions are documented.
    #[serde(rename = "conditional_pass")]
    ConditionalPass(Vec<String>),
    /// One or more critical obligations have failed.
    #[serde(rename = "fail")]
    Fail(Vec<String>),
}
