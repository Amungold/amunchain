use crate::state::{SimulationState, LegitimacyClaim};
use crate::types::ClaimAction;

pub struct EffectivenessEngine;

impl EffectivenessEngine {
    pub fn compute(claim: &LegitimacyClaim, state: &SimulationState) -> f64 {
        let f1 = Self::witness_validity(claim);
        let f2 = Self::recognition_support(claim, state);
        let f3 = Self::treaty_coherence(claim, state);
        let f4 = Self::jurisdiction_match(claim, state);
        let f5 = Self::epoch_validity(claim);
        f1 * f2 * f3 * f4 * f5
    }

    fn witness_validity(_claim: &LegitimacyClaim) -> f64 { 1.0 }

    fn recognition_support(claim: &LegitimacyClaim, state: &SimulationState) -> f64 {
        let n = state.sovereigns.len();
        if n == 0 { return 0.0; }
        let recognizers = state.recognition.iter()
            .filter(|row| row[claim.issuer])
            .count();
        recognizers as f64 / n as f64
    }

    fn treaty_coherence(claim: &LegitimacyClaim, state: &SimulationState) -> f64 {
        let n = state.sovereigns.len();
        if n == 0 { return 0.0; }

        if claim.action == ClaimAction::Treaty || claim.action == ClaimAction::Trade {
            let has_treaty = state.treaties[claim.issuer][claim.subject];
            return if has_treaty { 1.0 } else { 0.3 };
        }

        if claim.action == ClaimAction::Govern {
            let mut total_treaties: usize = 0;
            for i in 0..n {
                for j in (i+1)..n {
                    if state.treaties[i][j] { total_treaties += 1; }
                }
            }
            let max = n * (n - 1) / 2;
            if max == 0 { return 0.0; }
            return (total_treaties as f64 / max as f64).min(1.0);
        }

        1.0
    }

    fn jurisdiction_match(claim: &LegitimacyClaim, state: &SimulationState) -> f64 {
        if claim.subject >= state.jurisdictions.len() { return 0.0; }
        let subject_j = &state.jurisdictions[claim.subject];
        claim.scope.overlap(subject_j).min(1.0)
    }

    fn epoch_validity(claim: &LegitimacyClaim) -> f64 {
        if claim.epoch_start <= claim.epoch_end { 1.0 } else { 0.0 }
    }

    pub fn update_all(state: &mut SimulationState) {
        let claims = state.claims.clone();
        state.effectiveness = claims.iter()
            .map(|c| Self::compute(c, state))
            .collect();
    }

    pub fn debug_claim(claim: &LegitimacyClaim, state: &SimulationState) -> (f64, f64, f64, f64, f64, f64) {
        let f1 = Self::witness_validity(claim);
        let f2 = Self::recognition_support(claim, state);
        let f3 = Self::treaty_coherence(claim, state);
        let f4 = Self::jurisdiction_match(claim, state);
        let f5 = Self::epoch_validity(claim);
        let e = f1 * f2 * f3 * f4 * f5;
        (f1, f2, f3, f4, f5, e)
    }
}
