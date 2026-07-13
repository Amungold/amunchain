pub mod alerts;
pub mod monitor;
pub mod recovery;

use amun_orchestrator_core::event::{EventBus, OrchestratorEvent};
use amun_orchestrator_core::health::HealthScorer;
use amun_orchestrator_core::state::{
    HealthReport, NetworkMetrics, OperationalState, OrchestratorState, ServiceKind, ServiceStatus,
};
use amun_orchestrator_service::ServiceManager;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

pub struct HealthSupervisor {
    event_bus: Arc<EventBus>,
    service_manager: Arc<ServiceManager>,
    check_interval_secs: u64,
    #[allow(dead_code)]
    max_restart_attempts: u32,
    #[allow(dead_code)]
    cooldown_secs: u64,
    last_report: RwLock<Option<HealthReport>>,
    data_dir: String,
}

impl HealthSupervisor {
    pub fn new(
        event_bus: Arc<EventBus>,
        service_manager: Arc<ServiceManager>,
        check_interval_secs: u64,
        max_restart_attempts: u32,
        cooldown_secs: u64,
        data_dir: &str,
    ) -> Self {
        Self {
            event_bus,
            service_manager,
            check_interval_secs,
            max_restart_attempts,
            cooldown_secs,
            last_report: RwLock::new(None),
            data_dir: data_dir.to_string(),
        }
    }

    pub async fn run(&self) {
        let mut tick = interval(Duration::from_secs(self.check_interval_secs));

        loop {
            tick.tick().await;
            self.event_bus.emit(
                "health-supervisor",
                OrchestratorEvent::HealthCheckCycleStarted,
            );

            let report = self.check_all().await;

            if report.overall_health {
                self.event_bus.emit(
                    "health-supervisor",
                    OrchestratorEvent::HealthCheckPassed {
                        components_checked: report.validators.len() + report.services.len(),
                        health_score: report.health_score,
                    },
                );
            } else {
                for component in &report.degraded_components {
                    self.event_bus.emit(
                        "health-supervisor",
                        OrchestratorEvent::HealthCheckFailed {
                            component: component.clone(),
                            reason: "Service degraded or down".into(),
                        },
                    );
                }
            }

            *self.last_report.write().await = Some(report);
        }
    }

    pub async fn check_all(&self) -> HealthReport {
        let mut validators = HashMap::new();
        let mut services = HashMap::new();
        let mut degraded = Vec::new();
        let mut alerts = Vec::new();

        let validators_dir = Path::new(&self.data_dir).join("validators");
        if validators_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&validators_dir) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let config_path = entry.path().join("config.toml");

                        let mut status = ServiceStatus::new();
                        if let Ok(content) = std::fs::read_to_string(&config_path) {
                            let p2p_port = extract_toml_value(&content, "listen_port")
                                .and_then(|v| v.parse::<u16>().ok())
                                .unwrap_or(0);

                            // Real TCP health check on P2P port
                            let p2p_healthy = monitor::check_p2p_health(p2p_port, 2).await;

                            status.running = p2p_healthy;
                            status.healthy = p2p_healthy;

                            if !status.healthy {
                                degraded.push(name.clone());
                                alerts.push(format!("{} P2P port {} unreachable", name, p2p_port));
                            }
                        }

                        validators.insert(name, status);
                    }
                }
            }
        }

        // Check managed services
        let service_list = self.service_manager.list_services().await;
        for kind in &service_list {
            if let Some(status) = self.service_manager.service_status(kind).await {
                if !status.healthy {
                    degraded.push(kind.name().to_string());
                }
                services.insert(kind.clone(), status);
            }
        }

        let healthy_validator_count = validators.values().filter(|s| s.healthy).count();
        let total_validators = validators.len();

        // Determine operational state
        let operational_state = if total_validators == 0 || healthy_validator_count == 0 {
            OperationalState::Unavailable // No validators or all down
        } else if !degraded.is_empty() {
            OperationalState::Degraded // Some validators or services are down
        } else {
            OperationalState::Healthy
        };

        let network = NetworkMetrics {
            validator_count: total_validators,
            connected_peers: healthy_validator_count,
            quorum_reached: healthy_validator_count > 0,
            rpc_online: services
                .get(&ServiceKind::Rpc)
                .map(|s| s.healthy)
                .unwrap_or(false),
            explorer_online: services
                .get(&ServiceKind::ExplorerApi)
                .map(|s| s.healthy)
                .unwrap_or(false),
            websocket_connected: services
                .get(&ServiceKind::WebSocket)
                .map(|s| s.healthy)
                .unwrap_or(false),
            ..Default::default()
        };

        let health_score = HealthScorer::calculate(&validators, &services, &network);
        let overall_health = operational_state == OperationalState::Healthy;

        HealthReport {
            timestamp: chrono::Utc::now().timestamp() as u64,
            state: OrchestratorState::new(),
            operational_state,
            overall_health,
            health_score,
            degraded_components: degraded,
            alerts,
            validators,
            services,
            network,
        }
    }
}

/// Extract a value from a TOML config line (top-level key).
fn extract_toml_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(pos) = trimmed.find(&format!("{} = ", key)) {
            return Some(
                trimmed[pos + key.len() + 3..]
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        }
        if let Some(pos) = trimmed.find(&format!("{} =", key)) {
            return Some(
                trimmed[pos + key.len() + 2..]
                    .trim()
                    .trim_matches('"')
                    .to_string(),
            );
        }
    }
    None
}
