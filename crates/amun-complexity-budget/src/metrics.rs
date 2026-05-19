#[derive(Debug, Clone)]
pub struct ComplexityMetrics {
    pub consensus_states: usize,
    pub consensus_transitions: usize,
    pub invariant_count: usize,
    pub attack_scenarios: usize,
    pub total_mass_milli: u128,
    pub measured_at_epoch: u64,
}

impl ComplexityMetrics {
    pub fn measure(
        consensus_states: usize,
        consensus_transitions: usize,
        invariant_count: usize,
        attack_scenarios: usize,
        epoch: u64,
    ) -> Self {
        let total_mass_milli = 
            (consensus_states as u128).saturating_mul(10_000) +
            (consensus_transitions as u128).saturating_mul(5_000) +
            (invariant_count as u128).saturating_mul(20_000) +
            (attack_scenarios as u128).saturating_mul(2_000);

        Self {
            consensus_states,
            consensus_transitions,
            invariant_count,
            attack_scenarios,
            total_mass_milli,
            measured_at_epoch: epoch,
        }
    }
}
