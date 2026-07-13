//! Frontier Reconciliation — constitutional epistemic convergence.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use crate::derivational_frontier::DerivationalFrontier;
use crate::closure_completeness::ClosureCompleteness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontierMergeResult {
    pub merged_frontier: DerivationalFrontier,
    pub newly_resolved: Vec<ConstitutionalHash>,
    pub completeness_improved: bool,
    pub resulting_completeness: ClosureCompleteness,
}

pub fn merge_frontiers(a: &DerivationalFrontier, b: &DerivationalFrontier) -> FrontierMergeResult {
    let mut merged = DerivationalFrontier::new(a.worker_id, a.target_artifact_hash);
    let mut newly_resolved: Vec<ConstitutionalHash> = Vec::new();

    for hash in &a.resolved { merged.resolve(*hash); }
    for hash in &b.resolved {
        if !a.resolved.contains(hash) { newly_resolved.push(*hash); }
        merged.resolve(*hash);
    }

    for dep in &a.unresolved {
        if !b.resolved.contains(&dep.artifact_hash) {
            merged.unresolved.push(dep.clone());
        } else if !newly_resolved.contains(&dep.artifact_hash) {
            newly_resolved.push(dep.artifact_hash);
        }
    }

    for dep in &b.unresolved {
        if !a.resolved.contains(&dep.artifact_hash) && !merged.unresolved.iter().any(|d| d.artifact_hash == dep.artifact_hash) {
            merged.unresolved.push(dep.clone());
        } else if !a.resolved.contains(&dep.artifact_hash) && !newly_resolved.contains(&dep.artifact_hash) {
            newly_resolved.push(dep.artifact_hash);
        }
    }

    merged.recompute_completeness();
    let result_completeness = merged.completeness;
    let improved = result_completeness.level() > a.completeness.level() || result_completeness.level() > b.completeness.level();

    FrontierMergeResult {
        merged_frontier: merged,
        newly_resolved,
        completeness_improved: improved,
        resulting_completeness: result_completeness,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierReduction { AMoreComplete, BMoreComplete, EquallyComplete, Incomparable }

pub fn compare_frontier_reduction(a: &DerivationalFrontier, b: &DerivationalFrontier) -> FrontierReduction {
    let a_resolves_b = b.unresolved.iter().any(|d| a.resolved.contains(&d.artifact_hash));
    let b_resolves_a = a.unresolved.iter().any(|d| b.resolved.contains(&d.artifact_hash));
    let a_blocking = a.blocking_dependencies().len();
    let b_blocking = b.blocking_dependencies().len();

    match (a_resolves_b, b_resolves_a) {
        (true, false) => FrontierReduction::AMoreComplete,
        (false, true) => FrontierReduction::BMoreComplete,
        (true, true) => {
            if a_blocking < b_blocking { FrontierReduction::AMoreComplete }
            else if b_blocking < a_blocking { FrontierReduction::BMoreComplete }
            else { FrontierReduction::EquallyComplete }
        }
        (false, false) => FrontierReduction::Incomparable,
    }
}

pub fn are_derivationally_equivalent(a: &DerivationalFrontier, b: &DerivationalFrontier) -> bool {
    a.completeness.can_derive() == b.completeness.can_derive()
}

pub fn compute_closure_delta(source: &DerivationalFrontier, target: &DerivationalFrontier) -> Vec<ConstitutionalHash> {
    let mut delta: Vec<ConstitutionalHash> = Vec::new();
    for dep in &target.unresolved {
        if dep.is_blocking && source.resolved.contains(&dep.artifact_hash) {
            delta.push(dep.artifact_hash);
        }
    }
    delta
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivational_frontier::FrontierDependencyReason;

    fn make_frontier(worker: u64, resolved: Vec<[u8; 32]>, unresolved: Vec<([u8; 32], FrontierDependencyReason, bool)>) -> DerivationalFrontier {
        let mut f = DerivationalFrontier::new(worker, [0xAA; 32]);
        for h in &resolved { f.resolve(*h); }
        for (h, reason, blocking) in &unresolved {
            f.unresolved.push(crate::derivational_frontier::FrontierDependency { artifact_hash: *h, dependency_reason: *reason, is_blocking: *blocking });
        }
        f.recompute_completeness();
        f
    }

    #[test] fn test_merge() {
        let a = make_frontier(100, vec![[0x01; 32]], vec![([0x02; 32], FrontierDependencyReason::HardWitnessRequired, true)]);
        let b = make_frontier(200, vec![[0x02; 32]], vec![([0x03; 32], FrontierDependencyReason::CausalChainIncomplete, false)]);
        let r = merge_frontiers(&a, &b);
        assert!(r.completeness_improved);
    }
    #[test] fn test_equivalence() {
        let a = make_frontier(100, vec![[0x01; 32], [0x02; 32]], vec![]);
        let b = make_frontier(200, vec![[0x02; 32], [0x03; 32]], vec![]);
        assert!(are_derivationally_equivalent(&a, &b));
    }
    #[test] fn test_delta() {
        let source = make_frontier(100, vec![[0x02; 32], [0x03; 32]], vec![]);
        let target = make_frontier(200, vec![[0x01; 32]], vec![([0x02; 32], FrontierDependencyReason::HardWitnessRequired, true), ([0x03; 32], FrontierDependencyReason::CausalChainIncomplete, false)]);
        let d = compute_closure_delta(&source, &target);
        assert!(d.contains(&[0x02; 32]));
    }
}
