use crate::dashboard::SurvivabilityConfidence;

#[derive(Debug)]
pub struct SurvivabilityMetrics {
    pub invariants_hold: bool,
    pub truth_verified: bool,
    pub fatal_failures_mitigated: bool,
    pub budget_within_limits: bool,
    pub complexity_mass_milli: u128,
    pub ontology_hash: [u8; 32],
    pub confidence: SurvivabilityConfidence,
}

impl SurvivabilityMetrics {
    pub fn is_survivable(&self) -> bool {
        self.invariants_hold && self.truth_verified && self.budget_within_limits
    }

    pub fn summary(&self) -> &'static str {
        match self.confidence {
            SurvivabilityConfidence::High => {
                "CONFIDENCE: HIGH - all invariants hold, truth verified"
            }
            SurvivabilityConfidence::Medium => {
                "CONFIDENCE: MEDIUM - core invariants hold, monitor complexity"
            }
            SurvivabilityConfidence::Low => {
                "CONFIDENCE: LOW - invariants hold but verification degraded"
            }
            SurvivabilityConfidence::Critical => "CONFIDENCE: CRITICAL - invariants violated",
        }
    }
}
