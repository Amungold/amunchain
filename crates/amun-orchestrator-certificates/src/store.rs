use amun_networking::validator_certificate::ValidatorCertificate;
use amun_orchestrator_core::error::OrchestratorError;
use std::path::Path;

pub async fn save_certificate(
    cert: &ValidatorCertificate,
    path: &Path,
) -> Result<(), OrchestratorError> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| OrchestratorError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
    }

    let json = serde_json::to_string_pretty(cert)
        .map_err(|e| OrchestratorError::Serialization(e.to_string()))?;

    tokio::fs::write(path, json)
        .await
        .map_err(|e| OrchestratorError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

    Ok(())
}

pub async fn load_certificate(path: &Path) -> Result<ValidatorCertificate, OrchestratorError> {
    let data = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| OrchestratorError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

    serde_json::from_str(&data).map_err(|e| OrchestratorError::Serialization(e.to_string()))
}
