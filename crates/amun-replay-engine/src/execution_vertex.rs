//! ExecutionVertex — an execution event within the distributed DAG.
//!
//! CRITICAL: A vertex represents EXECUTION only. It does NOT carry:
//!   - Constitutional validity
//!   - Admissibility judgments
//!   - Truth status
//!   - Semantic authority
//!
//! The constitutional kernel judges artifacts AFTER execution.
//! The vertex records WHAT was executed, not WHETHER it was valid.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;

/// The type of execution event this vertex represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexType {
    /// A state transition was executed.
    StateTransition,
    /// A witness was extracted.
    WitnessExtraction,
    /// An artifact was verified.
    Verification,
    /// A snapshot was taken.
    Snapshot,
    /// State was restored from a snapshot.
    Restoration,
}

/// An execution vertex in the distributed DAG.
///
/// This is an EXECUTION record, not a truth record.
/// The produced artifact hashes are passed to the constitutional kernel
/// for judgment. The vertex itself makes no validity claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionVertex {
    /// Unique vertex identifier (operational, not constitutional).
    pub vertex_id: u64,

    /// What kind of execution occurred.
    pub vertex_type: VertexType,

    /// The worker that executed this vertex (operational provenance).
    pub worker_id: u64,

    /// The constitutional context this execution occurred within.
    pub context_hash: ConstitutionalHash,

    /// The boundary active during this execution.
    pub boundary_hash: ConstitutionalHash,

    /// Hashes of artifacts produced by this execution.
    /// These are passed to the constitutional kernel for judgment.
    pub produced_artifacts: Vec<ConstitutionalHash>,

    /// Execution dependencies — vertices that must complete before this one.
    /// These are EXECUTION dependencies, not constitutional causal dependencies.
    pub execution_dependencies: Vec<u64>,

    /// Execution timestamp (operational, informational only — CIR-001).
    pub execution_timestamp: Option<u64>,
}

impl ExecutionVertex {
    pub fn new(
        vertex_id: u64,
        vertex_type: VertexType,
        worker_id: u64,
        context_hash: ConstitutionalHash,
        boundary_hash: ConstitutionalHash,
    ) -> Self {
        Self {
            vertex_id,
            vertex_type,
            worker_id,
            context_hash,
            boundary_hash,
            produced_artifacts: Vec::new(),
            execution_dependencies: Vec::new(),
            execution_timestamp: None,
        }
    }

    /// Add a produced artifact hash.
    pub fn with_artifact(mut self, hash: ConstitutionalHash) -> Self {
        self.produced_artifacts.push(hash);
        self
    }

    /// Add an execution dependency on another vertex.
    pub fn with_dependency(mut self, vertex_id: u64) -> Self {
        self.execution_dependencies.push(vertex_id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let v = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
            .with_artifact([0xCD; 32])
            .with_dependency(0);
        assert_eq!(v.vertex_id, 1);
        assert_eq!(v.worker_id, 100);
        assert_eq!(v.produced_artifacts.len(), 1);
        assert_eq!(v.execution_dependencies.len(), 1);
    }

    #[test]
    fn test_vertex_is_not_truth() {
        // A vertex with no artifacts is still a valid execution record.
        // It just means nothing was produced — the constitutional kernel
        // will determine if that's admissible.
        let v = ExecutionVertex::new(2, VertexType::Verification, 200, [0xAB; 32], [0xBC; 32]);
        assert!(v.produced_artifacts.is_empty());
    }
}
