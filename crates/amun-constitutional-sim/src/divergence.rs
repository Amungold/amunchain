use crate::state::SimulationState;

pub struct DivergenceEngine;

impl DivergenceEngine {
    /// D_R: Recognition divergence
    pub fn recognition_divergence(s1: &SimulationState, s2: &SimulationState) -> f64 {
        let n = s1.sovereigns.len().min(s2.sovereigns.len());
        if n == 0 { return 0.0; }
        let mut diff = 0;
        for i in 0..n {
            for j in 0..n {
                if i != j && s1.recognition[i][j] != s2.recognition[i][j] {
                    diff += 1;
                }
            }
        }
        diff as f64 / (n * (n - 1)) as f64
    }

    /// D_T: Treaty divergence
    pub fn treaty_divergence(s1: &SimulationState, s2: &SimulationState) -> f64 {
        let n = s1.sovereigns.len().min(s2.sovereigns.len());
        if n == 0 { return 0.0; }
        let mut diff = 0;
        let mut total = 0;
        for i in 0..n {
            for j in (i+1)..n {
                if s1.treaties[i][j] != s2.treaties[i][j] { diff += 1; }
                total += 1;
            }
        }
        if total == 0 { return 0.0; }
        diff as f64 / total as f64
    }

    /// D_E: Effectiveness divergence
    pub fn effectiveness_divergence(s1: &SimulationState, s2: &SimulationState) -> f64 {
        let n = s1.effectiveness.len().min(s2.effectiveness.len());
        if n == 0 { return 0.0; }
        s1.effectiveness.iter().zip(s2.effectiveness.iter())
            .map(|(e1, e2)| (e1 - e2).abs())
            .sum::<f64>() / n as f64
    }

    /// D_L = α·D_R + β·D_T + γ·D_J + δ·D_E
    pub fn compute(s1: &SimulationState, s2: &SimulationState) -> f64 {
        let alpha = 0.3;
        let beta = 0.2;
        let gamma = 0.1;
        let delta = 0.4;

        alpha * Self::recognition_divergence(s1, s2)
            + beta * Self::treaty_divergence(s1, s2)
            + gamma * 0.0 // D_J simplified for now
            + delta * Self::effectiveness_divergence(s1, s2)
    }

    /// Collapse detection
    pub fn is_collapse(divergence: f64, threshold: f64) -> bool {
        divergence > threshold
    }

    /// Collapse probability estimate
    pub fn collapse_probability(current: &SimulationState, reference: &SimulationState, threshold: f64) -> f64 {
        let d = Self::compute(current, reference);
        (d / threshold).min(1.0)
    }
}
