//! Proof Routing Fabric — constitutional proof logistics.
//!
//! Routes proofs through the network WITHOUT acquiring semantic authority.
//! Routing is an optimization layer: it moves proofs efficiently.
//! It does NOT judge admissibility, validity, or truth.
//!
//! CRITICAL: Shortest path ≠ best proof. Fastest ≠ strongest.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use crate::derivational_frontier::DerivationalFrontier;

/// A proof route descriptor — where a proof goes and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofRoute {
    pub route_id: u64,
    /// The target artifact whose admissibility proof is being routed.
    pub target_artifact_hash: ConstitutionalHash,
    /// The context this route operates within.
    pub context_hash: ConstitutionalHash,
    /// The type of proof being routed.
    pub proof_type: RoutedProofType,
    /// Routing priority — operational, NOT semantic.
    pub priority: RoutePriority,
    /// Workers this route should avoid (quarantined/suspicious).
    pub avoid_workers: Vec<u64>,
    /// Maximum hops before route expires.
    pub max_hops: u64,
    /// Current hop count.
    pub current_hops: u64,
}

/// What kind of proof is being routed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutedProofType {
    /// A full witness for admissibility verification.
    FullWitness,
    /// A closure delta — minimal frontier reduction.
    ClosureDelta,
    /// An equivalence fingerprint — no surface data needed.
    EquivalenceFingerprint,
    /// A derivational frontier description.
    FrontierDescription,
}

/// Operational routing priority — NOT semantic preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutePriority {
    /// Route when bandwidth is available.
    Low,
    /// Standard routing priority.
    Normal,
    /// Route urgently — frontier is blocking admissibility for a worker.
    /// This is OPERATIONAL urgency, not semantic importance.
    FrontierBlocking,
}

impl ProofRoute {
    pub fn new(
        route_id: u64, target: ConstitutionalHash, context: ConstitutionalHash,
        proof_type: RoutedProofType,
    ) -> Self {
        Self {
            route_id, target_artifact_hash: target, context_hash: context,
            proof_type, priority: RoutePriority::Normal,
            avoid_workers: Vec::new(), max_hops: 20, current_hops: 0,
        }
    }

    /// Set priority based on the receiving worker's frontier.
    pub fn with_frontier_priority(mut self, frontier: &DerivationalFrontier) -> Self {
        if frontier.blocking_dependencies().len() > 0 {
            self.priority = RoutePriority::FrontierBlocking;
        }
        self
    }

    /// Avoid quarantined workers.
    pub fn with_avoid_workers(mut self, workers: Vec<u64>) -> Self {
        self.avoid_workers = workers;
        self
    }

    /// Increment hop count. Returns true if route is still valid.
    pub fn increment_hop(&mut self) -> bool {
        self.current_hops += 1;
        self.current_hops <= self.max_hops
    }

    /// Returns true if this route has expired.
    pub fn is_expired(&self) -> bool { self.current_hops > self.max_hops }
}

/// Closure-aware routing decision — where to send a proof next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingDecision {
    /// Workers that should receive this proof.
    pub target_workers: Vec<u64>,
    /// Whether the proof should be compressed before sending.
    pub should_compress: bool,
    /// Whether this route can be skipped (redundant admissibility).
    pub is_redundant: bool,
    /// Reason for this routing decision.
    pub reason: RoutingReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingReason {
    /// Target worker has an unresolved frontier that this proof resolves.
    FrontierReduction,
    /// Target worker already has an equivalent admissibility surface.
    RedundantEquivalence,
    /// Target worker is quarantined — skip.
    QuarantineSkip,
    /// Normal propagation.
    NormalPropagation,
    /// Route expired — drop.
    RouteExpired,
}

/// Make a closure-aware routing decision.
///
/// Determines whether a proof should be sent to a target worker
/// based on the worker's current derivational frontier.
/// Does NOT make semantic judgments about proof validity.
pub fn closure_aware_routing(
    route: &ProofRoute,
    target_frontiers: &[(u64, &DerivationalFrontier)],
    quarantine_list: &[u64],
    known_fingerprints: &[ConstitutionalHash],
) -> Vec<RoutingDecision> {
    let mut decisions = Vec::new();

    for (worker_id, frontier) in target_frontiers {
        // Skip quarantined workers
        if quarantine_list.contains(worker_id) {
            decisions.push(RoutingDecision {
                target_workers: vec![*worker_id],
                should_compress: false,
                is_redundant: true,
                reason: RoutingReason::QuarantineSkip,
            });
            continue;
        }

        // Check if worker already has equivalent admissibility
        if known_fingerprints.contains(&frontier.target_artifact_hash) {
            decisions.push(RoutingDecision {
                target_workers: vec![*worker_id],
                should_compress: false,
                is_redundant: true,
                reason: RoutingReason::RedundantEquivalence,
            });
            continue;
        }

        // Check if this proof resolves a frontier for the worker
        let resolves_frontier = frontier.unresolved.iter()
            .any(|d| d.is_blocking && route.target_artifact_hash == d.artifact_hash);

        if resolves_frontier {
            decisions.push(RoutingDecision {
                target_workers: vec![*worker_id],
                should_compress: false,
                is_redundant: false,
                reason: RoutingReason::FrontierReduction,
            });
        } else {
            decisions.push(RoutingDecision {
                target_workers: vec![*worker_id],
                should_compress: true,
                is_redundant: false,
                reason: RoutingReason::NormalPropagation,
            });
        }
    }

    decisions
}

/// Determine if a proof should be compressed based on equivalence awareness.
/// If the target already has a surface in the same equivalence class, skip.
pub fn equivalence_aware_compression(
    target_fingerprint: ConstitutionalHash,
    known_fingerprints: &[ConstitutionalHash],
) -> bool {
    known_fingerprints.contains(&target_fingerprint)
}

/// Propagation cost boundary — prevent routing amplification.
/// Returns true if this route should be throttled.
pub fn within_propagation_budget(
    current_routes: usize,
    max_concurrent_routes: usize,
) -> bool {
    current_routes < max_concurrent_routes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivational_frontier::{FrontierDependencyReason};

    fn make_frontier(worker: u64, resolved: Vec<[u8; 32]>, unresolved: Vec<([u8; 32], FrontierDependencyReason, bool)>) -> DerivationalFrontier {
        let mut f = DerivationalFrontier::new(worker, [0xAA; 32]);
        for h in &resolved { f.resolve(*h); }
        for (h, reason, blocking) in &unresolved {
            f.unresolved.push(crate::derivational_frontier::FrontierDependency { artifact_hash: *h, dependency_reason: *reason, is_blocking: *blocking });
        }
        f.recompute_completeness();
        f
    }

    #[test]
    fn test_quarantine_skip() {
        let route = ProofRoute::new(1, [0xBB; 32], [0xAB; 32], RoutedProofType::ClosureDelta);
        let frontier = make_frontier(100, vec![], vec![([0xBB; 32], FrontierDependencyReason::HardWitnessRequired, true)]);
        let decisions = closure_aware_routing(
            &route,
            &[(100, &frontier)],
            &[100], // worker 100 is quarantined
            &[],
        );
        assert_eq!(decisions[0].reason, RoutingReason::QuarantineSkip);
    }

    #[test]
    fn test_redundant_equivalence() {
        let route = ProofRoute::new(1, [0xBB; 32], [0xAB; 32], RoutedProofType::EquivalenceFingerprint);
        let frontier = make_frontier(200, vec![[0xBB; 32]], vec![]);
        let decisions = closure_aware_routing(
            &route,
            &[(200, &frontier)],
            &[],
            &[[0xAA; 32]], // worker already has this fingerprint
        );
        assert_eq!(decisions[0].reason, RoutingReason::RedundantEquivalence);
    }

    #[test]
    fn test_frontier_reduction() {
        let route = ProofRoute::new(1, [0xBB; 32], [0xAB; 32], RoutedProofType::ClosureDelta);
        let frontier = make_frontier(300, vec![], vec![([0xBB; 32], FrontierDependencyReason::HardWitnessRequired, true)]);
        let decisions = closure_aware_routing(
            &route,
            &[(300, &frontier)],
            &[],
            &[],
        );
        assert_eq!(decisions[0].reason, RoutingReason::FrontierReduction);
    }

    #[test]
    fn test_hop_expiry() {
        let mut route = ProofRoute::new(1, [0xBB; 32], [0xAB; 32], RoutedProofType::FullWitness);
        route.max_hops = 3;
        assert!(route.increment_hop());
        assert!(route.increment_hop());
        assert!(route.increment_hop());
        assert!(!route.increment_hop()); // expired
        assert!(route.is_expired());
    }

    #[test]
    fn test_propagation_budget() {
        assert!(within_propagation_budget(5, 10));
        assert!(!within_propagation_budget(10, 10));
    }

    #[test]
    fn test_equivalence_compression() {
        let fp = [0xCC; 32];
        let known = vec![[0xAA; 32], [0xBB; 32], [0xCC; 32]];
        assert!(equivalence_aware_compression(fp, &known));
        assert!(!equivalence_aware_compression([0xDD; 32], &known));
    }
}
