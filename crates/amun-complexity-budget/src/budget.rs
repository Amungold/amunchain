pub struct ComplexityBudget {
    pub max_consensus_states: usize,
    pub max_consensus_transitions: usize,
    pub max_invariants: usize,
    pub max_attack_scenarios: usize,
    pub max_total_complexity_mass_milli: u128,
}

impl ComplexityBudget {
    pub fn sovereign_default() -> Self {
        Self {
            max_consensus_states: 20,
            max_consensus_transitions: 50,
            max_invariants: 8,
            max_attack_scenarios: 100,
            max_total_complexity_mass_milli: 1_000_000,
        }
    }

    pub fn exceeds_budget(&self, metrics: &ComplexityMetrics) -> Option<BudgetViolation> {
        if metrics.consensus_states > self.max_consensus_states {
            return Some(BudgetViolation {
                field: "consensus_states",
            });
        }
        if metrics.consensus_transitions > self.max_consensus_transitions {
            return Some(BudgetViolation {
                field: "consensus_transitions",
            });
        }
        if metrics.invariant_count > self.max_invariants {
            return Some(BudgetViolation {
                field: "invariant_count",
            });
        }
        if metrics.attack_scenarios > self.max_attack_scenarios {
            return Some(BudgetViolation {
                field: "attack_scenarios",
            });
        }
        if metrics.total_mass_milli > self.max_total_complexity_mass_milli {
            return Some(BudgetViolation {
                field: "total_complexity_mass",
            });
        }
        None
    }
}

pub struct BudgetViolation {
    pub field: &'static str,
}

use crate::metrics::ComplexityMetrics;
