//! SemanticContaminationBoundary — prevents operational→semantic escalation.
//!
//! The boundary ensures that runtime anomalies remain operational concerns.
//! No anomaly, quarantine, or suspicion may silently become a
//! constitutional judgment about validity, admissibility, or truth.

use crate::runtime_anomaly::RuntimeAnomalySurface;

/// The status of a worker relative to the semantic contamination boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainmentStatus {
    /// Worker is operating normally.
    Operational,
    /// Worker is under observation due to anomalies.
    Observed,
    /// Worker is operationally quarantined — isolated from propagation.
    /// This is OPERATIONAL isolation, NOT constitutional invalidation.
    Quarantined,
    /// Worker is evicted from the runtime federation.
    /// Its produced artifacts remain constitutionally valid if they pass kernel verification.
    Evicted,
}

/// The semantic contamination boundary.
///
/// This boundary PREVENTS operational states from becoming semantic judgments.
/// A quarantined worker's artifacts are NOT automatically invalid.
/// An evicted worker's past proofs are NOT retroactively rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticContaminationBoundary {
    /// The worker this boundary applies to.
    pub worker_id: u64,
    /// Current containment status.
    pub status: ContainmentStatus,
    /// The anomaly surface that led to this status.
    pub anomaly_surface: RuntimeAnomalySurface,
    /// Whether this worker's artifacts require additional verification.
    /// This is an OPERATIONAL flag, not a validity judgment.
    pub requires_extra_verification: bool,
}

impl SemanticContaminationBoundary {
    pub fn new(worker_id: u64, anomaly_surface: RuntimeAnomalySurface) -> Self {
        let status = if anomaly_surface.should_quarantine() {
            ContainmentStatus::Quarantined
        } else if anomaly_surface.total_anomalies > 0 {
            ContainmentStatus::Observed
        } else {
            ContainmentStatus::Operational
        };

        Self {
            worker_id, status, anomaly_surface,
            requires_extra_verification: matches!(status, ContainmentStatus::Quarantined | ContainmentStatus::Observed),
        }
    }

    /// Returns true if the worker is operationally contained.
    pub fn is_contained(&self) -> bool {
        matches!(self.status, ContainmentStatus::Quarantined | ContainmentStatus::Evicted)
    }

    /// CRITICAL: This method does NOT invalidate artifacts.
    /// It only signals that extra verification is prudent.
    /// The constitutional kernel remains the sole judge of validity.
    pub fn is_operationally_restricted(&self) -> bool {
        self.is_contained()
    }
}

/// A Byzantine containment zone — isolates workers without semantic invalidation.
#[derive(Debug, Clone, Default)]
pub struct ByzantineContainmentZone {
    boundaries: Vec<SemanticContaminationBoundary>,
}

impl ByzantineContainmentZone {
    pub fn new() -> Self { Self { boundaries: Vec::new() } }

    pub fn add_boundary(&mut self, boundary: SemanticContaminationBoundary) {
        self.boundaries.push(boundary);
    }

    pub fn get_status(&self, worker_id: u64) -> ContainmentStatus {
        self.boundaries.iter()
            .find(|b| b.worker_id == worker_id)
            .map(|b| b.status)
            .unwrap_or(ContainmentStatus::Operational)
    }

    pub fn contained_workers(&self) -> Vec<u64> {
        self.boundaries.iter()
            .filter(|b| b.is_contained())
            .map(|b| b.worker_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_anomaly::{RuntimeAnomaly, AnomalyType, RuntimeAnomalySurface};

    #[test]
    fn test_clean_worker_is_operational() {
        let surface = RuntimeAnomalySurface::new(100);
        let boundary = SemanticContaminationBoundary::new(100, surface);
        assert_eq!(boundary.status, ContainmentStatus::Operational);
        assert!(!boundary.is_contained());
    }

    #[test]
    fn test_hostile_worker_is_quarantined() {
        let mut surface = RuntimeAnomalySurface::new(200);
        surface.record(RuntimeAnomaly::new(1, 200, AnomalyType::AdmissibilitySpoof, [0xAB; 32]));
        surface.record(RuntimeAnomaly::new(2, 200, AnomalyType::FrontierConflict, [0xAB; 32]));
        surface.record(RuntimeAnomaly::new(3, 200, AnomalyType::EquivalencePoisoning, [0xAB; 32]));
        let boundary = SemanticContaminationBoundary::new(200, surface);
        assert_eq!(boundary.status, ContainmentStatus::Quarantined);
        assert!(boundary.is_contained());
    }

    #[test]
    fn test_containment_is_not_invalidation() {
        let mut surface = RuntimeAnomalySurface::new(300);
        surface.record(RuntimeAnomaly::new(1, 300, AnomalyType::AdmissibilitySpoof, [0xAB; 32]));
        let boundary = SemanticContaminationBoundary::new(300, surface);
        // Worker is observed, not yet quarantined
        assert_eq!(boundary.status, ContainmentStatus::Observed);
        // But this does NOT invalidate its artifacts
        // The constitutional kernel remains the sole judge
        assert!(boundary.requires_extra_verification);
    }

    #[test]
    fn test_containment_zone() {
        let mut zone = ByzantineContainmentZone::new();
        let mut surface = RuntimeAnomalySurface::new(400);
        surface.record(RuntimeAnomaly::new(1, 400, AnomalyType::AdmissibilitySpoof, [0xAB; 32]));
        surface.record(RuntimeAnomaly::new(2, 400, AnomalyType::FrontierConflict, [0xAB; 32]));
        surface.record(RuntimeAnomaly::new(3, 400, AnomalyType::EquivalencePoisoning, [0xAB; 32]));
        zone.add_boundary(SemanticContaminationBoundary::new(400, surface));
        assert_eq!(zone.contained_workers(), vec![400]);
        assert_eq!(zone.get_status(999), ContainmentStatus::Operational);
    }
}
