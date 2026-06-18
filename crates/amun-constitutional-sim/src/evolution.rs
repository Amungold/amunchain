use crate::state::SimulationState;
use crate::types::ClaimAction;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub struct EvolutionOperator;

impl EvolutionOperator {
    pub fn recognition_erosion(state: &mut SimulationState, erosion_rate: f64) {
        let mut rng = StdRng::seed_from_u64(42);
        let n = state.sovereigns.len();
        for i in 0..n {
            for j in 0..n {
                if i != j && state.recognition[i][j] && rng.gen::<f64>() < erosion_rate {
                    state.recognition[i][j] = false;
                }
            }
        }
    }

    pub fn recognition_formation(
        state: &mut SimulationState,
        base_rate: f64,
        legitimacy_factor: f64,
        reciprocity_bias: f64,
    ) {
        let mut rng = StdRng::seed_from_u64(42);
        let n = state.sovereigns.len();

        // Use average effectiveness per entity (not total)
        let mut entity_avg = vec![0.0; n];
        let mut entity_count = vec![0; n];
        for (claim, &eff) in state.claims.iter().zip(state.effectiveness.iter()) {
            entity_avg[claim.issuer] += eff;
            entity_count[claim.issuer] += 1;
        }
        for i in 0..n {
            if entity_count[i] > 0 {
                entity_avg[i] /= entity_count[i] as f64;
            }
        }

        for i in 0..n {
            for j in 0..n {
                if i != j && !state.recognition[i][j] {
                    let target_legitimacy = entity_avg[j];
                    let target_reciprocity = if state.recognition[j][i] { 1.0 } else { 0.0 };
                    let prob = base_rate
                        + legitimacy_factor * target_legitimacy
                        + reciprocity_bias * target_reciprocity;
                    if rng.gen::<f64>() < prob.min(1.0) {
                        state.recognition[i][j] = true;
                    }
                }
            }
        }
    }

    pub fn treaty_evolution(state: &mut SimulationState, failure_rate: f64) {
        let mut rng = StdRng::seed_from_u64(42);
        let n = state.sovereigns.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if state.treaties[i][j] && rng.gen::<f64>() < failure_rate {
                    state.treaties[i][j] = false;
                    state.treaties[j][i] = false;
                }
            }
        }
    }

    pub fn jurisdiction_evolution(state: &mut SimulationState, shift_rate: f64) {
        let mut rng = StdRng::seed_from_u64(42);
        for j in &mut state.jurisdictions {
            if rng.gen::<f64>() < shift_rate {
                let new_region = rng.gen_range(0..10);
                if !j.regions.contains(&new_region) {
                    j.regions.push(new_region);
                }
            }
        }
    }

    pub fn claim_generation(state: &mut SimulationState, generation_rate: f64) {
        let mut rng = StdRng::seed_from_u64(42);
        let n = state.sovereigns.len();
        if n < 2 {
            return;
        }
        let actions = vec![
            ClaimAction::Govern,
            ClaimAction::Trade,
            ClaimAction::Treaty,
            ClaimAction::Recognize,
            ClaimAction::Tax,
            ClaimAction::Defend,
        ];
        if rng.gen::<f64>() < generation_rate {
            let issuer = rng.gen_range(0..n);
            let mut subject = rng.gen_range(0..n);
            while subject == issuer {
                subject = rng.gen_range(0..n);
            }
            let claim = crate::state::LegitimacyClaim {
                issuer,
                subject,
                action: actions[rng.gen_range(0..actions.len())].clone(),
                scope: state.jurisdictions[issuer].clone(),
                epoch_start: 0,
                epoch_end: 10000,
            };
            state.claims.push(claim);
            state.effectiveness.push(0.0);
        }
        if state.claims.len() > 1000 {
            let excess = state.claims.len() - 1000;
            state.claims.drain(0..excess);
            state.effectiveness.drain(0..excess);
        }
    }

    pub fn apply(
        state: &mut SimulationState,
        erosion_rate: f64,
        formation_base: f64,
        formation_legitimacy: f64,
        formation_reciprocity: f64,
        treaty_failure_rate: f64,
    ) {
        // Use current effectiveness for formation decisions
        Self::recognition_erosion(state, erosion_rate);
        Self::recognition_formation(
            state,
            formation_base,
            formation_legitimacy,
            formation_reciprocity,
        );
        Self::treaty_evolution(state, treaty_failure_rate);
        Self::jurisdiction_evolution(state, 0.05);
        Self::claim_generation(state, 0.1);
        // Single update after all structural changes
    }
}
