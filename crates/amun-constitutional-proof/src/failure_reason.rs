use serde::{Deserialize, Serialize};

/// Describes why a particular obligation failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureReason {
    /// A stable machine-readable code (e.g. "MISSING_EVIDENCE").
    pub code: String,
    /// A human-readable explanation of the failure.
    pub description: String,
}

impl FailureReason {
    pub fn new(code: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
        }
    }
}
