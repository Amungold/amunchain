pub mod environment;
pub mod rollback;
pub mod snapshot;
pub mod upgrade;

use amun_orchestrator_core::config::Environment;
use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::event::{EventBus, OrchestratorEvent};
use amun_orchestrator_core::OrchestratorConfig;
use amun_orchestrator_service::ServiceManager;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Tracks deployment state across environments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeploymentStatus {
    Idle,
    Building,
    Deploying,
    Migrating,
    Verifying,
    Active,
    Failed(String),
    RollingBack,
}

/// The deployment engine orchestrates testnet and mainnet deployments.
pub struct DeploymentEngine {
    event_bus: Arc<EventBus>,
    service_manager: Arc<ServiceManager>,
    config: RwLock<OrchestratorConfig>,
    status: RwLock<DeploymentStatus>,
    deployment_history: RwLock<Vec<DeploymentRecord>>,
}

#[derive(Debug, Clone)]
pub struct DeploymentRecord {
    pub id: String,
    pub timestamp: String,
    pub environment: Environment,
    pub version: String,
    pub status: DeploymentStatus,
    pub duration_secs: u64,
}

impl DeploymentEngine {
    pub fn new(
        event_bus: Arc<EventBus>,
        service_manager: Arc<ServiceManager>,
        config: OrchestratorConfig,
    ) -> Self {
        Self {
            event_bus,
            service_manager,
            config: RwLock::new(config),
            status: RwLock::new(DeploymentStatus::Idle),
            deployment_history: RwLock::new(Vec::new()),
        }
    }

    /// Deploy to the configured environment.
    pub async fn deploy(&self) -> Result<DeploymentRecord, OrchestratorError> {
        let start = std::time::Instant::now();
        let config = self.config.read().await.clone();
        let env = format!("{:?}", config.deployment.environment);

        *self.status.write().await = DeploymentStatus::Deploying;

        self.event_bus.emit(
            "deployment-engine",
            OrchestratorEvent::DeploymentStarted {
                environment: env.clone(),
            },
        );

        tracing::info!(%env, "Starting deployment");

        // Step 1: Build
        *self.status.write().await = DeploymentStatus::Building;
        // Builder would be called here

        // Step 2: Generate genesis if needed
        if config.deployment.auto_genesis {
            environment::ensure_genesis(&config).await?;
        }

        // Step 3: Generate certificates if needed
        if config.deployment.auto_certs {
            environment::ensure_certificates(&config).await?;
        }

        // Step 4: Verify deployment
        *self.status.write().await = DeploymentStatus::Verifying;
        environment::verify_deployment(&config).await?;

        // Step 5: Mark active
        *self.status.write().await = DeploymentStatus::Active;

        let duration_secs = start.elapsed().as_secs();
        let record = DeploymentRecord {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            environment: config.deployment.environment.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            status: DeploymentStatus::Active,
            duration_secs,
        };

        self.deployment_history.write().await.push(record.clone());

        self.event_bus.emit(
            "deployment-engine",
            OrchestratorEvent::DeploymentCompleted { environment: env },
        );

        tracing::info!(duration_secs, "Deployment complete");
        Ok(record)
    }

    /// Perform a rolling upgrade of all validators.
    pub async fn rolling_upgrade(&self) -> Result<(), OrchestratorError> {
        let config = self.config.read().await.clone();
        let validators = config.validators.clone();

        tracing::info!(count = validators.len(), "Starting rolling upgrade");

        for (i, validator) in validators.iter().enumerate() {
            tracing::info!(validator = %validator.name, progress = format!("{}/{}", i + 1, validators.len()));

            // Stop validator
            self.service_manager
                .stop_service(
                    &validator.name,
                    &amun_orchestrator_core::state::ServiceKind::Other(validator.name.clone()),
                )
                .await?;

            // Wait for drain
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            // Build (incremental)
            // Start validator
            self.service_manager
                .start_service(
                    &validator.name,
                    amun_orchestrator_core::state::ServiceKind::Other(validator.name.clone()),
                    "amun-node",
                    &["--config".into(), validator.service_name.clone()],
                )
                .await?;

            // Wait for sync
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }

        tracing::info!("Rolling upgrade complete");
        Ok(())
    }

    /// Rollback to the previous deployment.
    pub async fn rollback(&self) -> Result<(), OrchestratorError> {
        *self.status.write().await = DeploymentStatus::RollingBack;

        self.event_bus
            .emit("deployment-engine", OrchestratorEvent::RollbackStarted);

        if let Some(last) = self.deployment_history.write().await.pop() {
            tracing::info!(deployment_id = %last.id, "Rolling back deployment");
            rollback::restore_snapshot(&*self.config.read().await).await?;
        }

        *self.status.write().await = DeploymentStatus::Active;

        self.event_bus
            .emit("deployment-engine", OrchestratorEvent::RollbackCompleted);

        Ok(())
    }

    /// Get deployment history.
    pub async fn history(&self) -> Vec<DeploymentRecord> {
        self.deployment_history.read().await.clone()
    }

    /// Current deployment status.
    pub async fn current_status(&self) -> DeploymentStatus {
        self.status.read().await.clone()
    }

    /// Create a snapshot before deployment.
    pub async fn create_snapshot(&self) -> Result<PathBuf, OrchestratorError> {
        let config = self.config.read().await.clone();
        snapshot::create_snapshot(&config).await
    }
}
