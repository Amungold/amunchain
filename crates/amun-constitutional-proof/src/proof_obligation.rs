use serde::{Deserialize, Serialize};

use crate::{ObligationId, ObligationKind, ObligationSeverity, ObligationStatus};

/// A single constitutional proof obligation as defined in Article I.
///
/// Each obligation carries a unique identifier, a human-readable description,
/// a formal statement, severity, lifecycle status, and optional dependencies
/// on other obligations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofObligation {
    pub id: ObligationId,
    pub kind: ObligationKind,
    pub description: String,
    pub formal_statement: String,
    pub severity: ObligationSeverity,
    pub phase: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<ObligationId>,
    pub version: u32,
    pub status: ObligationStatus,
}

impl ProofObligation {
    /// Create a new obligation with sensible defaults.
    pub fn new(
        id: ObligationId,
        kind: ObligationKind,
        description: impl Into<String>,
        formal_statement: impl Into<String>,
        severity: ObligationSeverity,
        phase: impl Into<String>,
    ) -> Self {
        Self {
            id,
            kind,
            description: description.into(),
            formal_statement: formal_statement.into(),
            severity,
            phase: phase.into(),
            depends_on: Vec::new(),
            version: 1,
            status: ObligationStatus::Active,
        }
    }

    /// Builder method: add a dependency.
    pub fn with_dependency(mut self, dep: ObligationId) -> Self {
        self.depends_on.push(dep);
        self
    }

    /// Builder method: set the status.
    pub fn with_status(mut self, status: ObligationStatus) -> Self {
        self.status = status;
        self
    }

    /// Builder method: set the version.
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }
}
