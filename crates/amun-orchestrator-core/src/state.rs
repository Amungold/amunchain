use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::error::OrchestratorError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeState {
    Stopped,
    Booting,
    Running,
    Degraded(String),
    Recovering,
    Paused(String),
    ReadOnly,
    Stopping,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentState {
    Idle,
    Building,
    GeneratingCertificates,
    GeneratingGenesis,
    LaunchingValidators,
    LaunchingServices,
    Upgrading,
    RollingBack,
    Maintenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrchestratorState {
    pub runtime: RuntimeState,
    pub deployment: DeploymentState,
}

impl OrchestratorState {
    pub fn new() -> Self {
        Self {
            runtime: RuntimeState::Stopped,
            deployment: DeploymentState::Idle,
        }
    }

    pub fn transition_runtime(&self, next: RuntimeState) -> crate::error::Result<RuntimeState> {
        use RuntimeState::*;
        let valid = matches!(
            (&self.runtime, &next),
            (Stopped, Booting)
                | (Stopped, ReadOnly)
                | (Booting, Running)
                | (Booting, ReadOnly)
                | (Running, Degraded(_))
                | (Running, Paused(_))
                | (Running, ReadOnly)
                | (Running, Stopping)
                | (Paused(_), Running)
                | (Paused(_), Stopping)
                | (Paused(_), ReadOnly)
                | (ReadOnly, Running)
                | (ReadOnly, Stopping)
                | (Degraded(_), Recovering)
                | (Recovering, Running)
                | (Recovering, Booting)
                | (Stopping, Stopped)
                | (_, Stopped)
        );
        if valid {
            tracing::info!(from = ?self.runtime, to = ?next, "Runtime transition");
            Ok(next)
        } else {
            Err(OrchestratorError::StateTransition {
                from: format!("{:?}", self.runtime),
                to: format!("{:?}", next),
            })
        }
    }

    pub fn transition_deployment(
        &self,
        next: DeploymentState,
    ) -> crate::error::Result<DeploymentState> {
        use DeploymentState::*;
        let valid = matches!(
            (&self.deployment, &next),
            (Idle, Building)
                | (Building, GeneratingCertificates)
                | (Building, Idle)
                | (GeneratingCertificates, GeneratingGenesis)
                | (GeneratingGenesis, LaunchingValidators)
                | (LaunchingValidators, LaunchingServices)
                | (LaunchingServices, Idle)
                | (Idle, Upgrading)
                | (Upgrading, Building)
                | (Upgrading, RollingBack)
                | (RollingBack, Idle)
                | (Idle, Maintenance)
                | (Maintenance, Idle)
                | (_, Idle)
        );
        if valid {
            tracing::info!(from = ?self.deployment, to = ?next, "Deployment transition");
            Ok(next)
        } else {
            Err(OrchestratorError::StateTransition {
                from: format!("{:?}", self.deployment),
                to: format!("{:?}", next),
            })
        }
    }
}

impl Default for OrchestratorState {
    fn default() -> Self {
        Self::new()
    }
}

// ── Service Kind ─────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceKind {
    Rpc,
    ExplorerApi,
    ExplorerUi,
    WebSocket,
    Other(String),
}

impl ServiceKind {
    pub fn name(&self) -> &str {
        match self {
            ServiceKind::Rpc => "rpc",
            ServiceKind::ExplorerApi => "explorer-api",
            ServiceKind::ExplorerUi => "explorer-ui",
            ServiceKind::WebSocket => "websocket",
            ServiceKind::Other(s) => s.as_str(),
        }
    }
}

impl std::fmt::Display for ServiceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

// ── Service Status ───────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceStatus {
    pub running: bool,
    pub healthy: bool,
    pub pid: Option<u32>,
    pub restart_count: u32,
    pub crash_count: u32,
    pub exit_code: Option<i32>,
    pub last_exit_reason: Option<String>,
    pub last_error: Option<String>,
    pub uptime_secs: u64,
    pub started_at: Option<SystemTime>,
    pub last_restart: Option<SystemTime>,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
}

impl ServiceStatus {
    pub fn new() -> Self {
        Self {
            running: false,
            healthy: false,
            pid: None,
            restart_count: 0,
            crash_count: 0,
            exit_code: None,
            last_exit_reason: None,
            last_error: None,
            uptime_secs: 0,
            started_at: None,
            last_restart: None,
            cpu_percent: 0.0,
            memory_bytes: 0,
        }
    }

    pub fn running(pid: u32) -> Self {
        Self {
            running: true,
            healthy: true,
            pid: Some(pid),
            started_at: Some(SystemTime::now()),
            ..Self::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub operational_state: OperationalState,
    pub timestamp: u64,
    pub state: OrchestratorState,
    pub overall_health: bool,
    pub health_score: u8,
    pub degraded_components: Vec<String>,
    pub alerts: Vec<String>,
    pub validators: HashMap<String, ServiceStatus>,
    pub services: HashMap<ServiceKind, ServiceStatus>,
    pub network: NetworkMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NetworkMetrics {
    pub validator_count: usize,
    pub connected_peers: usize,
    pub finalized_height: u64,
    pub latest_height: u64,
    pub average_block_time_ms: u64,
    pub average_tps: f64,
    pub websocket_connected: bool,
    pub rpc_online: bool,
    pub explorer_online: bool,
    pub quorum_reached: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeConfig {
    pub auto_restart: bool,
    pub auto_snapshot: bool,
    pub auto_backup: bool,
    pub auto_prune: bool,
    pub snapshot_interval_blocks: u64,
    pub backup_interval_secs: u64,
    pub prune_keep_blocks: u64,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            auto_restart: true,
            auto_snapshot: true,
            auto_backup: true,
            auto_prune: true,
            snapshot_interval_blocks: 1000,
            backup_interval_secs: 3600,
            prune_keep_blocks: 100_000,
        }
    }
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::new()
    }
}

/// Operational state of the network, derived from health checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationalState {
    /// Network is fully operational.
    Healthy,
    /// Network is running but some components are degraded.
    Degraded,
    /// Network is unable to function (e.g., no consensus).
    Unavailable,
}

impl std::fmt::Display for OperationalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationalState::Healthy => write!(f, "Healthy"),
            OperationalState::Degraded => write!(f, "Degraded"),
            OperationalState::Unavailable => write!(f, "Unavailable"),
        }
    }
}
