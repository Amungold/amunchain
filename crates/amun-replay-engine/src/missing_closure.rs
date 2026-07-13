//! MissingClosureRequest — "I have partial derivability. I need these hashes."
//!
//! This is the most important primitive in the distributed era.
//! Instead of "sync blocks" or "sync state", workers request
//! specific constitutional artifacts needed to complete proof closure.
//!
//! A MissingClosureRequest is NOT an invalidity claim.
//! It says: "I cannot yet derive admissibility" — not "this is invalid."

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;

/// A request for missing constitutional artifacts needed for proof closure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingClosureRequest {
    /// Unique request identifier.
    pub request_id: u64,

    /// The worker that needs closure.
    pub requesting_worker_id: u64,

    /// The target artifact we're trying to prove admissibility for.
    pub target_artifact_hash: ConstitutionalHash,

    /// The context this request belongs to.
    pub context_hash: ConstitutionalHash,

    /// Hashes of artifacts we HAVE (partial proof surface).
    pub available_hashes: Vec<ConstitutionalHash>,

    /// Hashes of artifacts we NEED to complete closure.
    pub missing_hashes: Vec<ConstitutionalHash>,

    /// The type of closure we're trying to achieve.
    pub closure_type: ClosureType,

    /// Maximum number of workers this request should reach.
    pub propagation_limit: u64,

    /// How many hops this request has traveled.
    pub hop_count: u64,
}

/// What kind of constitutional closure is being requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureType {
    /// Need all hard dependencies for witness completeness.
    HardDependencyClosure,
    /// Need sufficient proof surface for admissibility verification.
    AdmissibilityClosure,
    /// Need full causal chain for audit purposes.
    CausalChainClosure,
    /// Need restoration lineage for snapshot recovery.
    RestorationClosure,
}

impl MissingClosureRequest {
    pub fn new(
        request_id: u64,
        requesting_worker_id: u64,
        target_artifact_hash: ConstitutionalHash,
        context_hash: ConstitutionalHash,
        closure_type: ClosureType,
    ) -> Self {
        Self {
            request_id,
            requesting_worker_id,
            target_artifact_hash,
            context_hash,
            available_hashes: Vec::new(),
            missing_hashes: Vec::new(),
            closure_type,
            propagation_limit: 10,
            hop_count: 0,
        }
    }

    /// Add an artifact hash we already have.
    pub fn with_available(mut self, hash: ConstitutionalHash) -> Self {
        self.available_hashes.push(hash);
        self
    }

    /// Add an artifact hash we need.
    pub fn with_missing(mut self, hash: ConstitutionalHash) -> Self {
        self.missing_hashes.push(hash);
        self
    }

    /// Increment hop count as the request propagates.
    pub fn increment_hop(&mut self) {
        self.hop_count += 1;
    }

    /// Returns true if this request has exceeded its propagation limit.
    pub fn is_exhausted(&self) -> bool {
        self.hop_count >= self.propagation_limit
    }

    /// Returns true if all missing hashes have been found.
    pub fn is_satisfied(&self) -> bool {
        self.missing_hashes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_request() {
        let req = MissingClosureRequest::new(
            1, 100, [0xAA; 32], [0xAB; 32], ClosureType::HardDependencyClosure,
        )
        .with_available([0x01; 32])
        .with_missing([0x02; 32])
        .with_missing([0x03; 32]);

        assert_eq!(req.available_hashes.len(), 1);
        assert_eq!(req.missing_hashes.len(), 2);
        assert!(!req.is_satisfied());
    }

    #[test]
    fn test_hop_exhaustion() {
        let mut req = MissingClosureRequest::new(
            1, 100, [0xAA; 32], [0xAB; 32], ClosureType::AdmissibilityClosure,
        );
        req.propagation_limit = 3;
        req.increment_hop();
        req.increment_hop();
        req.increment_hop();
        assert!(req.is_exhausted());
    }
}
