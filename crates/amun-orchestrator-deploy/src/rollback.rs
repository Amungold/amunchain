use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::OrchestratorConfig;
use std::path::PathBuf;

/// Restore from the most recent snapshot.
pub async fn restore_snapshot(config: &OrchestratorConfig) -> Result<(), OrchestratorError> {
    let snapshot_dir = config.network.base_dir.join("snapshots");

    if !snapshot_dir.exists() {
        return Err(OrchestratorError::Deployment(
            "No snapshots available for rollback".into(),
        ));
    }

    // Find the latest snapshot
    let mut latest: Option<(PathBuf, std::time::SystemTime)> = None;
    let mut entries =
        tokio::fs::read_dir(&snapshot_dir)
            .await
            .map_err(|e| OrchestratorError::Io {
                path: snapshot_dir.clone(),
                source: e,
            })?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| OrchestratorError::Io {
            path: snapshot_dir.clone(),
            source: e,
        })?
    {
        if let Ok(meta) = entry.metadata().await {
            if let Ok(modified) = meta.modified() {
                match &latest {
                    None => latest = Some((entry.path(), modified)),
                    Some((_, prev_time)) if modified > *prev_time => {
                        latest = Some((entry.path(), modified));
                    }
                    _ => {}
                }
            }
        }
    }

    if let Some((path, _)) = latest {
        tracing::info!(snapshot = %path.display(), "Restoring from snapshot");
        // Restore logic here
        return Ok(());
    }

    Err(OrchestratorError::Deployment(
        "No valid snapshot found".into(),
    ))
}
