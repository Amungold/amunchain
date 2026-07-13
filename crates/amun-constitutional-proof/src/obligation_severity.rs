use serde::{Deserialize, Serialize};

/// Severity of an obligation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ObligationSeverity {
    /// Failure means system security collapse.
    #[serde(rename = "critical")]
    Critical,
    /// Failure means a major functional breach.
    #[serde(rename = "major")]
    Major,
    /// Failure is a shortcoming that should be documented but does not block certification.
    #[serde(rename = "minor")]
    Minor,
    /// Informational only.
    #[serde(rename = "advisory")]
    Advisory,
}
