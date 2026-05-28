//! DerivationalFrontier — the minimal unresolved constitutional boundary.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use crate::closure_completeness::ClosureCompleteness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontierDependency {
    pub artifact_hash: ConstitutionalHash,
    pub dependency_reason: FrontierDependencyReason,
    pub is_blocking: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierDependencyReason {
    HardWitnessRequired,
    CausalChainIncomplete,
    RestorationLineageMissing,
    BoundaryVerificationNeeded,
    RevisionCompatibilityCheck,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivationalFrontier {
    pub worker_id: u64,
    pub target_artifact_hash: ConstitutionalHash,
    pub unresolved: Vec<FrontierDependency>,
    pub resolved: Vec<ConstitutionalHash>,
    pub completeness: ClosureCompleteness,
}

impl DerivationalFrontier {
    pub fn new(worker_id: u64, target_artifact_hash: ConstitutionalHash) -> Self {
        Self { worker_id, target_artifact_hash, unresolved: Vec::new(), resolved: Vec::new(), completeness: ClosureCompleteness::Partial }
    }
    pub fn with_unresolved(mut self, hash: ConstitutionalHash, reason: FrontierDependencyReason, blocking: bool) -> Self {
        self.unresolved.push(FrontierDependency { artifact_hash: hash, dependency_reason: reason, is_blocking: blocking });
        self
    }
    pub fn resolve(&mut self, hash: ConstitutionalHash) {
        self.unresolved.retain(|d| d.artifact_hash != hash);
        if !self.resolved.contains(&hash) { self.resolved.push(hash); }
        self.recompute_completeness();
    }
    pub fn recompute_completeness(&mut self) {
        let blocking_remaining = self.unresolved.iter().filter(|d| d.is_blocking).count();
        if blocking_remaining == 0 && !self.unresolved.is_empty() { self.completeness = ClosureCompleteness::Sufficient; }
        else if self.unresolved.is_empty() { self.completeness = ClosureCompleteness::Exhaustive; }
        else { self.completeness = ClosureCompleteness::Partial; }
    }
    pub fn is_clear(&self) -> bool { self.unresolved.is_empty() }
    pub fn blocking_dependencies(&self) -> Vec<&FrontierDependency> { self.unresolved.iter().filter(|d| d.is_blocking).collect() }
}
