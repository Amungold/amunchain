use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::OrchestratorConfig;
use std::path::PathBuf;

pub async fn create_snapshot(config: &OrchestratorConfig) -> Result<PathBuf, OrchestratorError> {
    let snapshot_dir = config.network.base_dir.join("snapshots");
    tokio::fs::create_dir_all(&snapshot_dir)
        .await
        .map_err(|e| OrchestratorError::Io {
            path: snapshot_dir.clone(),
            source: e,
        })?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let snapshot_path = snapshot_dir.join(format!("deploy_{}.json", timestamp));

    let config_json = serde_json::to_string_pretty(config)
        .map_err(|e| OrchestratorError::Serialization(e.to_string()))?;

    tokio::fs::write(&snapshot_path, config_json)
        .await
        .map_err(|e| OrchestratorError::Io {
            path: snapshot_path.clone(),
            source: e,
        })?;

    tracing::info!(path = %snapshot_path.display(), "Deployment snapshot created");
    Ok(snapshot_path)
}
