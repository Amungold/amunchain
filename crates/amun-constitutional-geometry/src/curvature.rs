/// Legitimacy curvature measures how "bent" the constitutional space is
/// around a civilization. High curvature = fragile legitimacy.
#[derive(Debug, Clone)]
pub struct LegitimacyCurvature {
    pub curvature_value: f64,
    pub is_stable: bool,
    pub is_critical: bool,
    pub is_collapsing: bool,
}

impl LegitimacyCurvature {
    pub fn new(curvature: f64) -> Self {
        Self {
            curvature_value: curvature,
            is_stable: curvature < 0.3,
            is_critical: curvature > 0.7,
            is_collapsing: curvature > 0.9,
        }
    }
}

/// Replay curvature measures deformation in the causal geometry
/// caused by replay divergence.
#[derive(Debug, Clone)]
pub struct ReplayCurvature {
    pub curvature_value: f64,
    pub has_divergence: bool,
    pub reconciliation_possible: bool,
}

impl ReplayCurvature {
    pub fn new(divergence_count: u64, total_transitions: u64) -> Self {
        let curvature = if total_transitions > 0 {
            divergence_count as f64 / total_transitions as f64
        } else {
            0.0
        };
        Self {
            curvature_value: curvature,
            has_divergence: curvature > 0.0,
            reconciliation_possible: curvature < 0.5,
        }
    }
}

/// Causal curvature measures how "curved" the causal topology is.
/// High causal curvature = many forks, merges, and complex ancestry.
#[derive(Debug, Clone)]
pub struct CausalCurvature {
    pub curvature_value: f64,
    pub fork_density: f64,
    pub merge_density: f64,
    pub complexity_class: CausalComplexity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CausalComplexity {
    Simple,    // Linear or near-linear
    Branching, // Some forks
    Complex,   // Many forks and merges
    Chaotic,   // Unstable causal structure
}

impl CausalCurvature {
    pub fn new(forks: u64, merges: u64, total: u64) -> Self {
        let fork_density = if total > 0 {
            forks as f64 / total as f64
        } else {
            0.0
        };
        let merge_density = if total > 0 {
            merges as f64 / total as f64
        } else {
            0.0
        };
        let curvature = (fork_density + merge_density).min(1.0);
        let complexity = if curvature < 0.1 {
            CausalComplexity::Simple
        } else if curvature < 0.4 {
            CausalComplexity::Branching
        } else if curvature < 0.7 {
            CausalComplexity::Complex
        } else {
            CausalComplexity::Chaotic
        };
        Self {
            curvature_value: curvature,
            fork_density,
            merge_density,
            complexity_class: complexity,
        }
    }
}
