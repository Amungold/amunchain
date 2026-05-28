//! RuntimeCapability — explicit runtime permissions.
//!
//! The runtime is granted SPECIFIC capabilities. Anything not
//! explicitly granted is PROHIBITED. This prevents capability bleed
//! where runtime operations accidentally influence constitutional outcomes.


/// A capability granted to a runtime worker.
///
/// Capabilities are EXPLICIT and MINIMAL.
/// The runtime cannot perform operations outside its granted capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeCapability {
    /// Worker may execute state transitions within a boundary.
    ExecuteTransitions,

    /// Worker may read state for verification purposes.
    ReadState,

    /// Worker may produce constitutional artifacts (journal entries, evidence).
    ProduceArtifacts,

    /// Worker may request witness extraction from the constitutional kernel.
    RequestWitness,

    /// Worker may verify existing artifacts against the constitutional kernel.
    VerifyArtifacts,

    /// Worker may restore state from a snapshot.
    RestoreFromSnapshot,
}

/// The set of capabilities granted to a worker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilitySet {
    pub capabilities: Vec<RuntimeCapability>,
}

impl CapabilitySet {
    pub fn new(capabilities: Vec<RuntimeCapability>) -> Self {
        Self { capabilities }
    }

    pub fn has(&self, cap: RuntimeCapability) -> bool {
        self.capabilities.contains(&cap)
    }

    /// Create a minimal capability set for a worker that only produces artifacts.
    pub fn artifact_producer() -> Self {
        Self::new(vec![
            RuntimeCapability::ExecuteTransitions,
            RuntimeCapability::ProduceArtifacts,
        ])
    }

    /// Create a verification-only capability set.
    pub fn verifier() -> Self {
        Self::new(vec![
            RuntimeCapability::ReadState,
            RuntimeCapability::VerifyArtifacts,
            RuntimeCapability::RequestWitness,
        ])
    }

    /// Create a full capability set (for recovery/restoration workers).
    pub fn recovery() -> Self {
        Self::new(vec![
            RuntimeCapability::ReadState,
            RuntimeCapability::ProduceArtifacts,
            RuntimeCapability::RestoreFromSnapshot,
            RuntimeCapability::RequestWitness,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producer_cannot_verify() {
        let caps = CapabilitySet::artifact_producer();
        assert!(!caps.has(RuntimeCapability::VerifyArtifacts));
        assert!(caps.has(RuntimeCapability::ProduceArtifacts));
    }

    #[test]
    fn test_verifier_cannot_execute() {
        let caps = CapabilitySet::verifier();
        assert!(!caps.has(RuntimeCapability::ExecuteTransitions));
        assert!(caps.has(RuntimeCapability::VerifyArtifacts));
    }

    #[test]
    fn test_recovery_can_restore() {
        let caps = CapabilitySet::recovery();
        assert!(caps.has(RuntimeCapability::RestoreFromSnapshot));
    }
}
