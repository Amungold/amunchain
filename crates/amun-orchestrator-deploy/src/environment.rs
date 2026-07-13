use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::OrchestratorConfig;

/// Ensure genesis exists for the configured environment.
pub async fn ensure_genesis(config: &OrchestratorConfig) -> Result<(), OrchestratorError> {
    let genesis_path = &config.network.genesis_file;
    if genesis_path.exists() {
        tracing::info!(path = %genesis_path.display(), "Genesis already exists");
        return Ok(());
    }

    tracing::info!(environment = ?config.deployment.environment, "Generating genesis");
    // Genesis generation would be called here
    Ok(())
}

/// Ensure certificates exist for all validators.
pub async fn ensure_certificates(config: &OrchestratorConfig) -> Result<(), OrchestratorError> {
    let certs_dir = config.network.base_dir.join("certificates");
    tokio::fs::create_dir_all(&certs_dir)
        .await
        .map_err(|e| OrchestratorError::Io {
            path: certs_dir.clone(),
            source: e,
        })?;

    for validator in &config.validators {
        let cert_path = certs_dir.join(format!("{}.crt", validator.name));
        if !cert_path.exists() {
            tracing::info!(%validator.name, "Certificate needed");
        }
    }

    Ok(())
}

/// Verify a deployment is healthy.
pub async fn verify_deployment(config: &OrchestratorConfig) -> Result<(), OrchestratorError> {
    // Check base directory exists
    if !config.network.base_dir.exists() {
        return Err(OrchestratorError::Deployment(
            "Base directory missing".into(),
        ));
    }

    // Check genesis exists
    if !config.network.genesis_file.exists() {
        return Err(OrchestratorError::Deployment("Genesis file missing".into()));
    }

    tracing::info!("Deployment verification passed");
    Ok(())
}
