//! ExecutionIsolationBoundary — capability fencing for runtime workers.
//!
//! An isolation boundary defines what a worker CAN and CANNOT do.
//! Any operation outside the boundary is an IsolationViolation.
//!
//! The boundary is ENFORCED at the runtime level, but its existence
//! is a constitutional requirement (Invariant 15).

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use crate::runtime_capability::{CapabilitySet, RuntimeCapability};

/// A violation of runtime isolation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsolationViolation {
    /// Worker attempted an operation outside its capability set.
    CapabilityExceeded {
        worker_id: u64,
        attempted: RuntimeCapability,
        granted: Vec<RuntimeCapability>,
    },

    /// Worker attempted to modify constitutional state directly.
    ConstitutionalStateModification {
        worker_id: u64,
        artifact_hash: ConstitutionalHash,
    },

    /// Worker attempted to influence witness identity.
    WitnessTampering {
        worker_id: u64,
        witness_id: u64,
    },

    /// Worker crossed into another worker's isolation context.
    ContextBreach {
        worker_id: u64,
        target_context_hash: ConstitutionalHash,
    },
}

impl IsolationViolation {
    pub fn describe(&self) -> Vec<u8> {
        match self {
            IsolationViolation::CapabilityExceeded { .. } => b"Capability exceeded".to_vec(),
            IsolationViolation::ConstitutionalStateModification { .. } => b"Constitutional state modification attempted".to_vec(),
            IsolationViolation::WitnessTampering { .. } => b"Witness tampering attempted".to_vec(),
            IsolationViolation::ContextBreach { .. } => b"Context breach attempted".to_vec(),
        }
    }
}

/// An execution isolation boundary for a runtime worker.
///
/// The boundary defines:
///   - What capabilities the worker has
///   - What context the worker operates within
///   - What operations are prohibited
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionIsolationBoundary {
    /// Unique identifier for this boundary.
    pub boundary_id: u64,

    /// The worker this boundary constrains.
    pub worker_id: u64,

    /// The capabilities granted to this worker.
    pub capabilities: CapabilitySet,

    /// The constitutional context this worker is bound to.
    pub context_hash: ConstitutionalHash,

    /// Whether the worker may communicate with other workers.
    pub inter_worker_communication: bool,

    /// Whether the worker may access shared state.
    pub shared_state_access: bool,
}

impl ExecutionIsolationBoundary {
    pub fn new(
        boundary_id: u64,
        worker_id: u64,
        capabilities: CapabilitySet,
        context_hash: ConstitutionalHash,
    ) -> Self {
        Self {
            boundary_id,
            worker_id,
            capabilities,
            context_hash,
            inter_worker_communication: false,
            shared_state_access: false,
        }
    }

    /// Check if the worker is allowed to perform a capability.
    /// Returns Ok if allowed, Err with IsolationViolation if not.
    pub fn check_capability(&self, requested: RuntimeCapability) -> Result<(), IsolationViolation> {
        if self.capabilities.has(requested) {
            Ok(())
        } else {
            Err(IsolationViolation::CapabilityExceeded {
                worker_id: self.worker_id,
                attempted: requested,
                granted: self.capabilities.capabilities.clone(),
            })
        }
    }

    /// Check if the worker is allowed to access a context.
    pub fn check_context(&self, context_hash: ConstitutionalHash) -> Result<(), IsolationViolation> {
        if context_hash == self.context_hash {
            Ok(())
        } else {
            Err(IsolationViolation::ContextBreach {
                worker_id: self.worker_id,
                target_context_hash: context_hash,
            })
        }
    }

    /// Create a strictly isolated boundary — no inter-worker communication,
    /// no shared state, minimal capabilities.
    pub fn strict(
        boundary_id: u64,
        worker_id: u64,
        capabilities: CapabilitySet,
        context_hash: ConstitutionalHash,
    ) -> Self {
        Self {
            boundary_id,
            worker_id,
            capabilities,
            context_hash,
            inter_worker_communication: false,
            shared_state_access: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_check_allowed() {
        let caps = CapabilitySet::artifact_producer();
        let boundary = ExecutionIsolationBoundary::strict(1, 100, caps, [0xAB; 32]);
        assert!(boundary.check_capability(RuntimeCapability::ProduceArtifacts).is_ok());
    }

    #[test]
    fn test_capability_check_denied() {
        let caps = CapabilitySet::artifact_producer();
        let boundary = ExecutionIsolationBoundary::strict(1, 100, caps, [0xAB; 32]);
        assert!(boundary.check_capability(RuntimeCapability::VerifyArtifacts).is_err());
    }

    #[test]
    fn test_context_check_allowed() {
        let caps = CapabilitySet::verifier();
        let boundary = ExecutionIsolationBoundary::strict(1, 100, caps, [0xAB; 32]);
        assert!(boundary.check_context([0xAB; 32]).is_ok());
    }

    #[test]
    fn test_context_check_denied() {
        let caps = CapabilitySet::verifier();
        let boundary = ExecutionIsolationBoundary::strict(1, 100, caps, [0xAB; 32]);
        assert!(boundary.check_context([0xCD; 32]).is_err());
    }
}
