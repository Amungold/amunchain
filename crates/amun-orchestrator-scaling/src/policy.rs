use amun_orchestrator_core::state::NetworkMetrics;
use serde::{Deserialize, Serialize};

/// A scaling policy defines when and how to scale.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScalingPolicy {
    pub min_validators: usize,
    pub max_validators: usize,
    pub scale_up_threshold_tps: f64,
    pub scale_down_threshold_tps: f64,
    pub scale_step: usize,
}

impl Default for ScalingPolicy {
    fn default() -> Self {
        Self {
            min_validators: 4,
            max_validators: 21,
            scale_up_threshold_tps: 1000.0,
            scale_down_threshold_tps: 100.0,
            scale_step: 1,
        }
    }
}

/// Result of a scaling evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum ScaleDecision {
    ScaleUp { count: usize, reason: String },
    ScaleDown { count: usize, reason: String },
    NoChange,
}

impl ScalingPolicy {
    /// Evaluate the current state against this policy.
    pub fn evaluate(&self, current_validators: usize, metrics: &NetworkMetrics) -> ScaleDecision {
        let tps = metrics.average_tps;

        // Scale up if TPS exceeds threshold and we haven't reached max
        if tps > self.scale_up_threshold_tps && current_validators < self.max_validators {
            let available = self.max_validators - current_validators;
            let count = std::cmp::min(self.scale_step, available);
            return ScaleDecision::ScaleUp {
                count,
                reason: format!(
                    "TPS ({:.1}) exceeds threshold ({:.1})",
                    tps, self.scale_up_threshold_tps
                ),
            };
        }

        // Scale down if TPS is below threshold and we're above min
        if tps < self.scale_down_threshold_tps && current_validators > self.min_validators {
            let available = current_validators - self.min_validators;
            let count = std::cmp::min(self.scale_step, available);
            return ScaleDecision::ScaleDown {
                count,
                reason: format!(
                    "TPS ({:.1}) below threshold ({:.1})",
                    tps, self.scale_down_threshold_tps
                ),
            };
        }

        ScaleDecision::NoChange
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_up_when_tps_high() {
        let policy = ScalingPolicy::default();
        let metrics = NetworkMetrics {
            average_tps: 1500.0,
            ..Default::default()
        };
        let decision = policy.evaluate(4, &metrics);
        assert!(matches!(decision, ScaleDecision::ScaleUp { .. }));
    }

    #[test]
    fn test_scale_down_when_tps_low() {
        let policy = ScalingPolicy::default();
        let metrics = NetworkMetrics {
            average_tps: 50.0,
            ..Default::default()
        };
        let decision = policy.evaluate(10, &metrics);
        assert!(matches!(decision, ScaleDecision::ScaleDown { .. }));
    }

    #[test]
    fn test_no_change_when_tps_normal() {
        let policy = ScalingPolicy::default();
        let metrics = NetworkMetrics {
            average_tps: 500.0,
            ..Default::default()
        };
        let decision = policy.evaluate(4, &metrics);
        assert_eq!(decision, ScaleDecision::NoChange);
    }

    #[test]
    fn test_respects_max_validators() {
        let policy = ScalingPolicy::default();
        let metrics = NetworkMetrics {
            average_tps: 2000.0,
            ..Default::default()
        };
        let decision = policy.evaluate(21, &metrics);
        assert_eq!(decision, ScaleDecision::NoChange);
    }

    #[test]
    fn test_respects_min_validators() {
        let policy = ScalingPolicy::default();
        let metrics = NetworkMetrics {
            average_tps: 10.0,
            ..Default::default()
        };
        let decision = policy.evaluate(4, &metrics);
        assert_eq!(decision, ScaleDecision::NoChange);
    }
}
