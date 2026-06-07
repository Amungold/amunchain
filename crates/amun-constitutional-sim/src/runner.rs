use crate::state::SimulationState;
use crate::evolution::EvolutionOperator;
use crate::effectiveness::EffectivenessEngine;
use crate::observables::Observables;
use crate::divergence::DivergenceEngine;
use crate::protocol::ExperimentalProtocol;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStep {
    pub step: usize,
    pub mean_legitimacy: f64,
    pub legitimacy_variance: f64,
    pub recognition_connectivity: f64,
    pub divergence_from_start: f64,
    pub collapse_risk: f64,
}

pub struct SimulationRunner;

impl SimulationRunner {
    pub fn run(protocol: &ExperimentalProtocol) -> Vec<SimulationStep> {
        let mut state = SimulationState::new(
            protocol.num_sovereigns,
            protocol.initial_recognition_density,
            protocol.initial_treaty_density,
        );
        EffectivenessEngine::update_all(&mut state);
        let reference = state.clone();
        let mut results = Vec::new();
        let collapse_threshold = 0.5;

        for step in 0..protocol.num_steps {
            EvolutionOperator::apply(
                &mut state,
                protocol.erosion_rate,
                protocol.formation_base_rate,
                protocol.formation_legitimacy_factor,
                protocol.formation_reciprocity_bias,
                protocol.treaty_failure_rate,
            );
            // Single update after all structural changes
            EffectivenessEngine::update_all(&mut state);

            let div = DivergenceEngine::compute(&state, &reference);
            results.push(SimulationStep {
                step,
                mean_legitimacy: Observables::mean_legitimacy(&state),
                legitimacy_variance: Observables::legitimacy_variance(&state),
                recognition_connectivity: Observables::recognition_connectivity(&state),
                divergence_from_start: div,
                collapse_risk: DivergenceEngine::collapse_probability(&state, &reference, collapse_threshold),
            });
        }
        results
    }
}
