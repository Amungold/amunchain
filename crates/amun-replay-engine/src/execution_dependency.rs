//! ExecutionDependencyType — execution-layer semantics only.
//!
//! These are EXECUTION dependencies, NOT constitutional causal dependencies.
//! They describe what a vertex needs before it can execute, not why
//! an artifact is constitutionally valid.
//!
//! CRITICAL DISTINCTION:
//!   - CausalDependency: why validity exists (constitutional layer)
//!   - ExecutionDependency: what must execute first (runtime layer)
//!   - These are DIFFERENT graphs with DIFFERENT semantics.

/// The type of execution dependency between vertices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionDependencyType {
    /// This vertex requires an artifact produced by another vertex.
    RequiresArtifact,

    /// This vertex requires a witness to be extracted first.
    RequiresWitness,

    /// This vertex requires a snapshot to be available.
    RequiresSnapshot,

    /// This vertex requires restoration to complete first.
    RequiresRestoration,

    /// This vertex has a soft dependency — it prefers but does not require
    /// the dependency to complete first. The scheduler may parallelize.
    SoftDependency,

    /// This vertex is fully parallelizable with respect to its dependency.
    /// The dependency is informational only (e.g., for provenance tracking).
    Parallelizable,
}

impl ExecutionDependencyType {
    /// Returns true if this dependency is mandatory for execution.
    pub fn is_mandatory(&self) -> bool {
        matches!(
            self,
            ExecutionDependencyType::RequiresArtifact
                | ExecutionDependencyType::RequiresWitness
                | ExecutionDependencyType::RequiresSnapshot
                | ExecutionDependencyType::RequiresRestoration
        )
    }

    /// Returns true if the scheduler may ignore this dependency.
    pub fn is_optional(&self) -> bool {
        matches!(
            self,
            ExecutionDependencyType::SoftDependency | ExecutionDependencyType::Parallelizable
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandatory_dependencies() {
        assert!(ExecutionDependencyType::RequiresArtifact.is_mandatory());
        assert!(ExecutionDependencyType::RequiresWitness.is_mandatory());
        assert!(!ExecutionDependencyType::SoftDependency.is_mandatory());
    }

    #[test]
    fn test_optional_dependencies() {
        assert!(ExecutionDependencyType::Parallelizable.is_optional());
        assert!(!ExecutionDependencyType::RequiresSnapshot.is_optional());
    }
}
