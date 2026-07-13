use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::traits::ProcessManager;
use std::sync::Arc;

/// Create a service definition and start it.
pub async fn create_service(
    pm: &Arc<dyn ProcessManager>,
    service_name: &str,
    config_path: &str,
) -> Result<(), OrchestratorError> {
    let args = vec!["--config".to_string(), config_path.to_string()];
    pm.start(service_name, &args).await?;
    tracing::info!(%service_name, "Service created and started");
    Ok(())
}

/// Start an existing service.
pub async fn start_service(
    pm: &Arc<dyn ProcessManager>,
    service_name: &str,
    config_path: &str,
) -> Result<u32, OrchestratorError> {
    if pm.is_running(service_name).await? {
        tracing::info!(%service_name, "Already running");
        return pm
            .pid(service_name)
            .await?
            .ok_or_else(|| OrchestratorError::Service {
                service: service_name.to_string(),
                message: "Running but no PID found".into(),
            });
    }

    let args = vec!["--config".to_string(), config_path.to_string()];
    pm.start(service_name, &args).await
}
