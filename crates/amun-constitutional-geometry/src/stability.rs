/// A stability basin is a region in constitutional phase space
/// where civilizations remain stable without external intervention.
#[derive(Debug, Clone)]
pub struct StabilityBasin {
    pub basin_id: [u8; 32],
    pub center_hash: [u8; 32],
    pub radius: f64,
    pub stability_score: f64,
    pub population: u64,
}

impl StabilityBasin {
    pub fn new(center: [u8; 32], radius: f64, stability: f64) -> Self {
        Self {
            basin_id: [0u8; 32],
            center_hash: center,
            radius,
            stability_score: stability,
            population: 0,
        }
    }

    /// Check if a civilization (identified by its distance from center)
    /// is within this stability basin.
    pub fn contains(&self, distance_from_center: f64) -> bool {
        distance_from_center <= self.radius
    }

    /// The attractor strength: how strongly this basin pulls civilizations.
    pub fn attractor_strength(&self) -> f64 {
        self.stability_score * (1.0 - (self.population as f64 / 1000.0).min(0.9))
    }
}

/// A stability attractor is a point in constitutional phase space
/// that civilizations naturally evolve toward.
#[derive(Debug, Clone)]
pub struct StabilityAttractor {
    pub attractor_hash: [u8; 32],
    pub attractor_type: AttractorType,
    pub pull_strength: f64,
    pub basin_radius: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttractorType {
    /// Strong constitutional invariants create a deep attractor
    InvariantAttractor,
    /// Replay determinism creates a stable attractor
    ReplayAttractor,
    /// Governance coherence creates an attractor
    GovernanceAttractor,
    /// Treaty networks create attractors
    TreatyAttractor,
    /// Entropy resistance creates negative attractors (repulsion from chaos)
    EntropyRepulsor,
}

impl StabilityAttractor {
    pub fn new(hash: [u8; 32], attractor_type: AttractorType, strength: f64) -> Self {
        Self {
            attractor_hash: hash,
            attractor_type,
            pull_strength: strength,
            basin_radius: strength * 10.0,
        }
    }
}
