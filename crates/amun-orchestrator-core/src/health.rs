use crate::state::{NetworkMetrics, ServiceKind, ServiceStatus};
use std::collections::HashMap;

/// Deterministic health score formula.
pub struct HealthScorer;

impl HealthScorer {
    pub fn calculate(
        validators: &HashMap<String, ServiceStatus>,
        services: &HashMap<ServiceKind, ServiceStatus>,
        _network: &NetworkMetrics,
    ) -> u8 {
        let mut score: i16 = 100;

        for status in validators.values() {
            if !status.running || !status.healthy {
                score -= 15;
            }
        }

        for (kind, status) in services.iter() {
            if !status.running || !status.healthy {
                match kind {
                    ServiceKind::Rpc => score -= 10,
                    ServiceKind::ExplorerApi | ServiceKind::ExplorerUi | ServiceKind::WebSocket => {
                        score -= 5
                    }
                    ServiceKind::Other(_) => score -= 5,
                }
            }
        }

        if score < 0 {
            0
        } else {
            score as u8
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ServiceStatus;

    fn healthy() -> ServiceStatus {
        ServiceStatus {
            running: true,
            healthy: true,
            ..ServiceStatus::new()
        }
    }
    fn down() -> ServiceStatus {
        ServiceStatus {
            running: false,
            healthy: false,
            ..ServiceStatus::new()
        }
    }

    #[test]
    fn perfect_score() {
        let v = HashMap::from([("v1".into(), healthy())]);
        let s = HashMap::from([(ServiceKind::Rpc, healthy())]);
        let n = NetworkMetrics::default();
        assert_eq!(HealthScorer::calculate(&v, &s, &n), 100);
    }

    #[test]
    fn validator_down_deducts_15() {
        let v = HashMap::from([("v1".into(), down())]);
        let s = HashMap::new();
        let n = NetworkMetrics::default();
        assert_eq!(HealthScorer::calculate(&v, &s, &n), 85);
    }

    #[test]
    fn all_validators_down_deducts_correctly() {
        let v = HashMap::from([
            ("v1".into(), down()),
            ("v2".into(), down()),
            ("v3".into(), down()),
        ]);
        let s = HashMap::new();
        let n = NetworkMetrics::default();
        assert_eq!(HealthScorer::calculate(&v, &s, &n), 55); // 100 - 45
    }

    #[test]
    fn rpc_down_deducts_10() {
        let v = HashMap::new();
        let s = HashMap::from([(ServiceKind::Rpc, down())]);
        let n = NetworkMetrics::default();
        assert_eq!(HealthScorer::calculate(&v, &s, &n), 90);
    }

    #[test]
    fn floor_at_zero() {
        let mut v = HashMap::new();
        for i in 0..10 {
            v.insert(format!("v{}", i), down());
        }
        let s = HashMap::new();
        let n = NetworkMetrics::default();
        assert_eq!(HealthScorer::calculate(&v, &s, &n), 0);
    }
}
