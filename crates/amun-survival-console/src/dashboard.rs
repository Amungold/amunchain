use crate::metrics::SurvivabilityMetrics;
use amun_invariants::registry::{InvariantHealth, InvariantRegistry};
use amun_complexity_budget::budget::ComplexityBudget;
use amun_complexity_budget::metrics::ComplexityMetrics;
use amun_truth_engine::TruthEngine;
use amun_failure_memory::ontology::FailureOntology;
use amun_attack_lab::simulator::AttackSimulator;

pub struct SurvivalDashboard {
    invariant_registry: InvariantRegistry,
    complexity_budget: ComplexityBudget,
    truth_engine: TruthEngine,
    failure_ontology: FailureOntology,
    attack_simulator: AttackSimulator,
}

#[derive(Debug)]
pub enum SurvivabilityConfidence {
    High,
    Medium,
    Low,
    Critical,
}

impl SurvivalDashboard {
    pub fn new(
        invariant_registry: InvariantRegistry,
        complexity_budget: ComplexityBudget,
        truth_engine: TruthEngine,
        failure_ontology: FailureOntology,
        attack_simulator: AttackSimulator,
    ) -> Self {
        Self {
            invariant_registry,
            complexity_budget,
            truth_engine,
            failure_ontology,
            attack_simulator,
        }
    }

    pub fn survivability_metrics(
        &self,
        consensus_states: usize,
        consensus_transitions: usize,
    ) -> SurvivabilityMetrics {
        let health = self.invariant_registry.overall_health();
        let posture = self.failure_ontology.posture_summary();
        let truth_ok = self.truth_engine.compute_chain_root(0).is_ok();
        let metrics = ComplexityMetrics::measure(
            consensus_states,
            consensus_transitions,
            self.invariant_registry.invariant_count(),
            self.attack_simulator.scenario_count(),
            1,
        );
        let budget_ok = self.complexity_budget.exceeds_budget(&metrics).is_none();

        let invariants_hold = health == InvariantHealth::AllInvariantsHold;
        let fatal_clear = posture.all_fatal_mitigated;

        let confidence = if invariants_hold && truth_ok && fatal_clear && budget_ok {
            SurvivabilityConfidence::High
        } else if invariants_hold && truth_ok && budget_ok {
            SurvivabilityConfidence::Medium
        } else if invariants_hold {
            SurvivabilityConfidence::Low
        } else {
            SurvivabilityConfidence::Critical
        };

        SurvivabilityMetrics {
            invariants_hold,
            truth_verified: truth_ok,
            fatal_failures_mitigated: fatal_clear,
            budget_within_limits: budget_ok,
            complexity_mass_milli: metrics.total_mass_milli,
            ontology_hash: posture.ontology_hash,
            confidence,
        }
    }
}
