use amun_orchestrator_core::state::HealthReport;
use tokio::time::{interval, Duration};

pub struct HealthChecker {
    interval_secs: u64,
}

impl HealthChecker {
    pub fn new(interval_secs: u64) -> Self {
        Self { interval_secs }
    }

    pub async fn run<F>(&self, check_fn: F)
    where
        F: Fn() -> HealthReport + Send + Sync + 'static,
    {
        let mut tick = interval(Duration::from_secs(self.interval_secs));
        loop {
            tick.tick().await;
            let report = check_fn();
            if !report.overall_health {
                tracing::warn!(
                    health_score = report.health_score,
                    degraded = ?report.degraded_components,
                    "Health check warning"
                );
            }
        }
    }
}
