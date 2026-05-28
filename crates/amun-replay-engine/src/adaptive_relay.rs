//! Adaptive Proof Relay — topology-aware delivery without semantic preference.
//!
//! The relay adapts to operational conditions (congestion, latency, backpressure)
//! but MUST NOT adapt to semantic conditions (admissibility quality, proof strength).
//!
//! CRITICAL: Adaptivity optimizes delivery, not truth.
//! Popular relays are not privileged relays.

use amun_constitutional::prelude::*;
use amun_constitutional::kernel_types::ConstitutionalHash;
use crate::derivational_frontier::DerivationalFrontier;
use crate::proof_routing::RoutedProofType;

/// A snapshot of the relay topology — strictly operational.
#[derive(Debug, Clone, Default)]
pub struct RelayTopologyView {
    /// Known relays and their operational metrics.
    pub relays: Vec<RelayMetrics>,
}

/// Operational metrics for a relay — NO semantic information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelayMetrics {
    pub relay_id: u64,
    /// Current congestion level (0 = idle, 255 = saturated).
    pub congestion: u8,
    /// Whether this relay is reachable.
    pub reachable: bool,
    /// Routes currently in flight through this relay.
    pub active_routes: u64,
}

impl RelayTopologyView {
    pub fn new() -> Self { Self { relays: Vec::new() } }

    pub fn add_relay(&mut self, metrics: RelayMetrics) {
        self.relays.push(metrics);
    }

    /// Select the best relay for a route based on OPERATIONAL metrics only.
    /// Returns relay_id with lowest congestion that is reachable.
    pub fn select_relay(&self, avoid_relays: &[u64]) -> Option<u64> {
        self.relays.iter()
            .filter(|r| r.reachable && !avoid_relays.contains(&r.relay_id))
            .min_by_key(|r| r.congestion)
            .map(|r| r.relay_id)
    }
}

/// Frontier pressure metrics — where are derivational bottlenecks?
#[derive(Debug, Clone, Default)]
pub struct FrontierPressureMetrics {
    /// Workers with blocking unresolved dependencies.
    pub blocked_workers: Vec<u64>,
    /// Density of unresolved frontiers per context.
    pub frontier_density: u64,
}

impl FrontierPressureMetrics {
    pub fn new() -> Self { Self { blocked_workers: Vec::new(), frontier_density: 0 } }

    /// Compute pressure from a set of frontiers.
    pub fn compute(frontiers: &[(u64, &DerivationalFrontier)]) -> Self {
        let blocked: Vec<u64> = frontiers.iter()
            .filter(|(_, f)| f.blocking_dependencies().len() > 0)
            .map(|(id, _)| *id)
            .collect();
        Self {
            frontier_density: blocked.len() as u64,
            blocked_workers: blocked,
        }
    }

    /// Returns true if the system is under high derivational pressure.
    pub fn is_under_pressure(&self) -> bool {
        self.frontier_density > 10
    }
}

/// Adaptive compression policy — decides what to send based on operational pressure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionDecision {
    /// Send full witness surface.
    SendFull,
    /// Send closure delta only (minimal frontier reduction).
    SendDelta,
    /// Send equivalence fingerprint only (no surface data).
    SendFingerprint,
}

/// Determine adaptive compression based on operational pressure.
///
/// Under high pressure: prefer fingerprints and deltas.
/// Under low pressure: full surfaces are acceptable.
/// CRITICAL: This adapts to OPERATIONAL conditions, not semantic ones.
pub fn adaptive_compression_policy(
    pressure: &FrontierPressureMetrics,
    route_type: RoutedProofType,
) -> CompressionDecision {
    match route_type {
        RoutedProofType::EquivalenceFingerprint => CompressionDecision::SendFingerprint,
        RoutedProofType::FrontierDescription => {
            if pressure.is_under_pressure() {
                CompressionDecision::SendDelta
            } else {
                CompressionDecision::SendFull
            }
        }
        RoutedProofType::ClosureDelta => CompressionDecision::SendDelta,
        RoutedProofType::FullWitness => {
            if pressure.is_under_pressure() {
                CompressionDecision::SendDelta
            } else {
                CompressionDecision::SendFull
            }
        }
    }
}

/// Relay backpressure — prevent proof storms.
#[derive(Debug, Clone)]
pub struct RelayBackpressure {
    /// Maximum concurrent routes allowed system-wide.
    pub max_concurrent_routes: usize,
    /// Current active routes.
    pub active_routes: usize,
    /// Whether backpressure is active.
    pub backpressure_active: bool,
}

impl RelayBackpressure {
    pub fn new(max_concurrent: usize) -> Self {
        Self { max_concurrent_routes: max_concurrent, active_routes: 0, backpressure_active: false }
    }

    /// Check if a new route can be accepted.
    pub fn can_accept(&self) -> bool {
        self.active_routes < self.max_concurrent_routes
    }

    /// Activate backpressure when approaching capacity.
    pub fn update(&mut self) {
        self.backpressure_active = self.active_routes >= self.max_concurrent_routes * 8 / 10;
    }
}

/// Equivalence relay cache — prevent re-transporting identical admissibility.
#[derive(Debug, Clone, Default)]
pub struct EquivalenceRelayCache {
    /// Known admissibility fingerprints.
    pub fingerprints: Vec<ConstitutionalHash>,
}

impl EquivalenceRelayCache {
    pub fn new() -> Self { Self { fingerprints: Vec::new() } }

    pub fn has(&self, fp: &ConstitutionalHash) -> bool {
        self.fingerprints.contains(fp)
    }

    pub fn insert(&mut self, fp: ConstitutionalHash) {
        if !self.fingerprints.contains(&fp) {
            self.fingerprints.push(fp);
        }
    }
}

/// Anti-Centrality Guard — prevents relay hubs from acquiring authority.
///
/// Tracks relay usage and detects centrality concentration.
/// Does NOT block hubs — but WARNS when topology becomes too centralized.
#[derive(Debug, Clone, Default)]
pub struct AntiCentralityGuard {
    /// Route count per relay.
    pub relay_route_counts: Vec<(u64, u64)>,
    /// Centrality threshold: if one relay handles > this fraction, warn.
    pub centrality_threshold: f64,
}

impl AntiCentralityGuard {
    pub fn new(threshold: f64) -> Self {
        Self { relay_route_counts: Vec::new(), centrality_threshold: threshold }
    }

    /// Record a route assigned to a relay.
    pub fn record_route(&mut self, relay_id: u64) {
        for (id, count) in &mut self.relay_route_counts {
            if *id == relay_id { *count += 1; return; }
        }
        self.relay_route_counts.push((relay_id, 1));
    }

    /// Returns true if any relay exceeds the centrality threshold.
    /// This is a WARNING — not a blocking action.
    pub fn is_centralized(&self) -> bool {
        let total: u64 = self.relay_route_counts.iter().map(|(_, c)| c).sum();
        if total == 0 { return false; }
        self.relay_route_counts.iter()
            .any(|(_, count)| (*count as f64 / total as f64) > self.centrality_threshold)
    }

    /// Returns the relay ID with the highest route count (if any).
    pub fn dominant_relay(&self) -> Option<u64> {
        self.relay_route_counts.iter()
            .max_by_key(|(_, c)| c)
            .map(|(id, _)| *id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derivational_frontier::FrontierDependencyReason;

    fn make_frontier(worker: u64, unresolved: Vec<([u8; 32], FrontierDependencyReason, bool)>) -> DerivationalFrontier {
        let mut f = DerivationalFrontier::new(worker, [0xAA; 32]);
        for (h, reason, blocking) in &unresolved {
            f.unresolved.push(crate::derivational_frontier::FrontierDependency { artifact_hash: *h, dependency_reason: *reason, is_blocking: *blocking });
        }
        f.recompute_completeness();
        f
    }

    #[test]
    fn test_relay_selection_avoids_congested() {
        let mut view = RelayTopologyView::new();
        view.add_relay(RelayMetrics { relay_id: 1, congestion: 200, reachable: true, active_routes: 10 });
        view.add_relay(RelayMetrics { relay_id: 2, congestion: 10, reachable: true, active_routes: 2 });
        let selected = view.select_relay(&[]);
        assert_eq!(selected, Some(2)); // lower congestion
    }

    #[test]
    fn test_relay_avoids_avoid_list() {
        let mut view = RelayTopologyView::new();
        view.add_relay(RelayMetrics { relay_id: 1, congestion: 10, reachable: true, active_routes: 1 });
        view.add_relay(RelayMetrics { relay_id: 2, congestion: 5, reachable: true, active_routes: 1 });
        let selected = view.select_relay(&[2]); // avoid relay 2
        assert_eq!(selected, Some(1));
    }

    #[test]
    fn test_frontier_pressure() {
        let f1 = make_frontier(100, vec![([0x01; 32], FrontierDependencyReason::HardWitnessRequired, true)]);
        let f2 = make_frontier(200, vec![]);
        let pressure = FrontierPressureMetrics::compute(&[(100, &f1), (200, &f2)]);
        assert_eq!(pressure.frontier_density, 1);
        assert!(pressure.blocked_workers.contains(&100));
        assert!(!pressure.is_under_pressure());
    }

    #[test]
    fn test_adaptive_compression_under_pressure() {
        let pressure = FrontierPressureMetrics { blocked_workers: vec![1; 15], frontier_density: 15 };
        let decision = adaptive_compression_policy(&pressure, RoutedProofType::FullWitness);
        assert_eq!(decision, CompressionDecision::SendDelta);
    }

    #[test]
    fn test_backpressure() {
        let mut bp = RelayBackpressure::new(10);
        assert!(bp.can_accept());
        bp.active_routes = 8;
        bp.update();
        assert!(bp.backpressure_active);
    }

    #[test]
    fn test_equivalence_cache() {
        let mut cache = EquivalenceRelayCache::new();
        let fp = [0xAA; 32];
        assert!(!cache.has(&fp));
        cache.insert(fp);
        assert!(cache.has(&fp));
    }

    #[test]
    fn test_anti_centrality_guard() {
        let mut guard = AntiCentralityGuard::new(0.5);
        guard.record_route(1);
        guard.record_route(1);
        guard.record_route(1);
        guard.record_route(2);
        // Relay 1 has 3/4 = 75% > 50%
        assert!(guard.is_centralized());
        assert_eq!(guard.dominant_relay(), Some(1));
    }

    #[test]
    fn test_no_centrality_when_balanced() {
        let mut guard = AntiCentralityGuard::new(0.5);
        guard.record_route(1);
        guard.record_route(2);
        guard.record_route(3);
        guard.record_route(4);
        assert!(!guard.is_centralized());
    }
}
