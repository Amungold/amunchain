use crate::state::SimulationState;

pub struct Observables;

impl Observables {
    pub fn mean_legitimacy(state: &SimulationState) -> f64 {
        if state.effectiveness.is_empty() {
            return 0.0;
        }
        state.effectiveness.iter().sum::<f64>() / state.effectiveness.len() as f64
    }

    pub fn legitimacy_variance(state: &SimulationState) -> f64 {
        let mean = Self::mean_legitimacy(state);
        if state.effectiveness.is_empty() {
            return 0.0;
        }
        state
            .effectiveness
            .iter()
            .map(|e| (e - mean).powi(2))
            .sum::<f64>()
            / state.effectiveness.len() as f64
    }

    pub fn recognition_connectivity(state: &SimulationState) -> f64 {
        let n = state.sovereigns.len();
        if n == 0 {
            return 0.0;
        }
        let total: usize = state
            .recognition
            .iter()
            .map(|row| row.iter().filter(|&&r| r).count())
            .sum();
        total as f64 / (n * (n - 1)) as f64
    }

    /// C_T: Real treaty coherence — proportion of treaty pairs without conflict
    pub fn treaty_coherence(state: &SimulationState) -> f64 {
        let n = state.sovereigns.len();
        if n < 2 {
            return 1.0;
        }
        let mut conflicts = 0;
        let mut total = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    // Conflict: i has treaty with j but not with k, and j has treaty with k
                    if state.treaties[i][j] && state.treaties[j][k] && !state.treaties[i][k] {
                        conflicts += 1;
                    }
                    total += 1;
                }
            }
        }
        if total == 0 {
            return 1.0;
        }
        1.0 - (conflicts as f64 / total as f64)
    }
}
