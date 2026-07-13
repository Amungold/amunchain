pub mod metrics;
pub mod policy;
pub mod trigger;

use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::event::{EventBus, OrchestratorEvent};
use amun_orchestrator_core::state::{NetworkMetrics, ServiceKind};
use amun_orchestrator_service::ServiceManager;
use policy::ScalingPolicy;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

/// The auto-scaler monitors network conditions and adjusts validator count.
pub struct AutoScaler {
    event_bus: Arc<EventBus>,
    service_manager: Arc<ServiceManager>,
    policy: RwLock<ScalingPolicy>,
    check_interval_secs: u64,
    cooldown_secs: u64,
    last_scale_action: RwLock<Option<chrono::DateTime<chrono::Utc>>>,
}

impl AutoScaler {
    pub fn new(
        event_bus: Arc<EventBus>,
        service_manager: Arc<ServiceManager>,
        policy: ScalingPolicy,
        check_interval_secs: u64,
        cooldown_secs: u64,
    ) -> Self {
        Self {
            event_bus,
            service_manager,
            policy: RwLock::new(policy),
            check_interval_secs,
            cooldown_secs,
            last_scale_action: RwLock::new(None),
        }
    }

    /// Start the auto-scaling loop. Runs until cancelled.
    pub async fn run(&self) {
        let mut tick = interval(Duration::from_secs(self.check_interval_secs));

        loop {
            tick.tick().await;
            if let Err(e) = self.evaluate_and_scale().await {
                tracing::warn!(error = %e, "Auto-scale evaluation failed");
            }
        }
    }

    /// Evaluate current metrics against policy and scale if needed.
    async fn evaluate_and_scale(&self) -> Result<(), OrchestratorError> {
        // Check cooldown
        if let Some(last) = *self.last_scale_action.read().await {
            let elapsed = chrono::Utc::now() - last;
            if elapsed.num_seconds() < self.cooldown_secs as i64 {
                tracing::debug!("Auto-scale cooldown active — skipping evaluation");
                return Ok(());
            }
        }

        let policy = self.policy.read().await.clone();
        let current_validators = self.current_validator_count().await;
        let metrics = self.collect_metrics().await;

        let decision = policy.evaluate(current_validators, &metrics);

        match decision {
            policy::ScaleDecision::ScaleUp { count, reason } => {
                tracing::info!(%count, %reason, "Scaling up");
                self.scale_up(count).await?;
                *self.last_scale_action.write().await = Some(chrono::Utc::now());

                self.event_bus.emit(
                    "auto-scaler",
                    OrchestratorEvent::ValidatorAdded {
                        name: format!("auto-scaled-{}", count),
                    },
                );
            }
            policy::ScaleDecision::ScaleDown { count, reason } => {
                tracing::info!(%count, %reason, "Scaling down");
                self.scale_down(count).await?;
                *self.last_scale_action.write().await = Some(chrono::Utc::now());

                self.event_bus.emit(
                    "auto-scaler",
                    OrchestratorEvent::ValidatorRemoved {
                        name: format!("auto-scaled-{}", count),
                    },
                );
            }
            policy::ScaleDecision::NoChange => {
                tracing::trace!("No scaling action needed");
            }
        }

        Ok(())
    }

    /// Scale up by adding validators.
    async fn scale_up(&self, count: usize) -> Result<(), OrchestratorError> {
        for _ in 0..count {
            // Generate validator name based on current count
            let current = self.current_validator_count().await;
            let name = format!("validator-{}", current + 1);

            // Start the validator
            self.service_manager
                .start_service(
                    &name,
                    ServiceKind::Other(name.clone()),
                    "amun-node",
                    &["--config".into(), format!("config/{}.toml", name)],
                )
                .await?;

            tracing::info!(%name, "Validator added via auto-scale");
        }
        Ok(())
    }

    /// Scale down by removing validators.
    async fn scale_down(&self, count: usize) -> Result<(), OrchestratorError> {
        let current = self.current_validator_count().await;
        let to_remove = std::cmp::min(count, current.saturating_sub(1)); // Keep at least 1

        for i in 0..to_remove {
            let name = format!("validator-{}", current - i);
            self.service_manager
                .stop_service(&name, &ServiceKind::Other(name.clone()))
                .await?;
            tracing::info!(%name, "Validator removed via auto-scale");
        }
        Ok(())
    }

    /// Get current validator count from running services.
    async fn current_validator_count(&self) -> usize {
        let services = self.service_manager.list_services().await;
        services
            .iter()
            .filter(|k| matches!(k, ServiceKind::Other(name) if name.starts_with("validator")))
            .count()
    }

    /// Collect network metrics for scaling decisions.
    async fn collect_metrics(&self) -> NetworkMetrics {
        // Metrics would come from health supervisor or direct queries
        NetworkMetrics::default()
    }

    /// Update the scaling policy at runtime.
    pub async fn update_policy(&self, new_policy: ScalingPolicy) {
        tracing::info!(?new_policy, "Scaling policy updated");
        *self.policy.write().await = new_policy;
    }

    /// Get the current scaling policy.
    pub async fn current_policy(&self) -> ScalingPolicy {
        self.policy.read().await.clone()
    }
}
