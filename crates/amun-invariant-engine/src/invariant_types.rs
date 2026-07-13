use serde::{Deserialize, Serialize};

/// Severity of a constitutional invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantSeverity {
    /// Failure means system security collapse.
    Critical,
    /// Failure means a major functional breach.
    Major,
    /// Failure should be documented but does not block execution.
    Minor,
    /// Informational only.
    Advisory,
}

/// Scope of an invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvariantScope {
    /// Single field or value within a contract.
    Local,
    /// Multiple fields within a single contract's state.
    State,
    /// Cross-contract economic equilibrium (evaluated by N47).
    Economic,
    /// System-wide constitutional property (evaluated by N47).
    Constitutional,
}

/// A declared invariant with its obligation ID and severity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantDeclaration {
    pub obligation_id: String,
    pub description: String,
    pub severity: InvariantSeverity,
    pub scope: InvariantScope,
}

/// Result of evaluating a single invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvariantResult {
    pub obligation_id: String,
    pub passed: bool,
    pub severity: InvariantSeverity,
}
