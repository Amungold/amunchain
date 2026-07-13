/// Metrics for constitutional stability over time.
#[derive(Debug, Clone)]
pub struct ConstitutionalStabilityMetrics {
    pub amendment_velocity: f64,
    pub governance_coherence: f64,
    pub replay_integrity_score: f64,
    pub lineage_fragmentation_index: f64,
    pub civilization_continuity_score: f64,
    pub total_transitions: u64,
    pub total_forks: u64,
    pub total_mergers: u64,
    pub active_amendments: u64,
}

impl Default for ConstitutionalStabilityMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstitutionalStabilityMetrics {
    pub fn new() -> Self {
        Self {
            amendment_velocity: 0.0,
            governance_coherence: 1.0,
            replay_integrity_score: 1.0,
            lineage_fragmentation_index: 0.0,
            civilization_continuity_score: 1.0,
            total_transitions: 0,
            total_forks: 0,
            total_mergers: 0,
            active_amendments: 0,
        }
    }
}
