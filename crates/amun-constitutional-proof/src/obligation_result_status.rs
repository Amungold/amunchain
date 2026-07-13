use serde::{Deserialize, Serialize};

/// The evaluation status of a single obligation within a verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationResultStatus {
    /// The obligation has been satisfied with evidence.
    #[serde(rename = "satisfied")]
    Satisfied,
    /// The obligation failed and the reason is documented.
    #[serde(rename = "failed")]
    Failed,
    /// The evaluation could not reach a conclusive result.
    #[serde(rename = "inconclusive")]
    Inconclusive,
    /// The obligation was waived by a constitutional decision (only for Advisory).
    #[serde(rename = "waived")]
    Waived,
    /// The obligation is not applicable to the subject under evaluation.
    #[serde(rename = "not_applicable")]
    NotApplicable,
}
