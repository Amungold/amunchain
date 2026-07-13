pub mod generator;
pub mod types;
pub mod validator;

use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::traits::GenesisProvider;
use amun_orchestrator_core::types::{PublicKey, ValidatorId};
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use types::Genesis;

/// Manages genesis lifecycle: generation, validation, and updates.
pub struct GenesisEngine {
    base_dir: PathBuf,
    chain_id: String,
    /// In-memory genesis state
    genesis: RwLock<Option<Genesis>>,
}

impl GenesisEngine {
    pub fn new(base_dir: PathBuf, chain_id: String) -> Self {
        Self {
            base_dir,
            chain_id,
            genesis: RwLock::new(None),
        }
    }

    /// Path to the genesis file.
    pub fn genesis_path(&self) -> PathBuf {
        self.base_dir.join("genesis.json")
    }

    /// Load existing genesis from disk.
    pub async fn load(&self) -> Result<Genesis, OrchestratorError> {
        let path = self.genesis_path();
        if !path.exists() {
            return Err(OrchestratorError::Genesis("Genesis file not found".into()));
        }

        let data = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| OrchestratorError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;

        let genesis: Genesis = serde_json::from_str(&data)
            .map_err(|e| OrchestratorError::Genesis(format!("Invalid genesis JSON: {}", e)))?;

        // Cache in memory
        *self.genesis.write().await = Some(genesis.clone());

        Ok(genesis)
    }

    /// Generate a fresh genesis with the given validators.
    pub async fn generate(
        &self,
        validators: &[(ValidatorId, PublicKey, u64)],
    ) -> Result<Genesis, OrchestratorError> {
        let genesis = generator::create_genesis(&self.chain_id, validators);

        // Write to disk
        self.write_genesis(&genesis).await?;

        // Cache
        *self.genesis.write().await = Some(genesis.clone());

        tracing::info!(
            chain_id = %self.chain_id,
            validator_count = validators.len(),
            "Genesis generated"
        );

        Ok(genesis)
    }

    /// Add a validator to existing genesis.
    pub async fn add_validator(
        &self,
        validator_id: ValidatorId,
        public_key: PublicKey,
        voting_power: u64,
    ) -> Result<Genesis, OrchestratorError> {
        let mut genesis = self
            .genesis
            .read()
            .await
            .clone()
            .unwrap_or_else(|| self.load_blocking());

        if genesis
            .validators
            .iter()
            .any(|v| v.validator_id == validator_id)
        {
            return Err(OrchestratorError::Genesis(format!(
                "Validator {} already in genesis",
                validator_id
            )));
        }

        genesis.validators.push(types::GenesisValidator {
            validator_id,
            public_key,
            voting_power,
            name: format!("validator-{}", hex::encode(&validator_id.0[..4])),
        });

        generator::recompute_hash(&mut genesis);
        self.write_genesis(&genesis).await?;
        *self.genesis.write().await = Some(genesis.clone());

        Ok(genesis)
    }

    /// Remove a validator from genesis.
    pub async fn remove_validator(
        &self,
        validator_id: &ValidatorId,
    ) -> Result<Genesis, OrchestratorError> {
        let mut genesis = self
            .genesis
            .read()
            .await
            .clone()
            .unwrap_or_else(|| self.load_blocking());

        let before = genesis.validators.len();
        genesis
            .validators
            .retain(|v| v.validator_id != *validator_id);

        if genesis.validators.len() == before {
            return Err(OrchestratorError::Genesis(format!(
                "Validator {} not found in genesis",
                validator_id
            )));
        }

        generator::recompute_hash(&mut genesis);
        self.write_genesis(&genesis).await?;
        *self.genesis.write().await = Some(genesis.clone());

        Ok(genesis)
    }

    /// Get current in-memory genesis (or load from disk).
    pub async fn current(&self) -> Result<Genesis, OrchestratorError> {
        if let Some(ref g) = *self.genesis.read().await {
            return Ok(g.clone());
        }
        self.load().await
    }

    async fn write_genesis(&self, genesis: &Genesis) -> Result<(), OrchestratorError> {
        let path = self.genesis_path();
        let json = serde_json::to_string_pretty(genesis)
            .map_err(|e| OrchestratorError::Serialization(e.to_string()))?;

        tokio::fs::write(&path, json)
            .await
            .map_err(|e| OrchestratorError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;

        Ok(())
    }

    fn load_blocking(&self) -> Genesis {
        let path = self.genesis_path();
        if path.exists() {
            let data = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_else(|_| Genesis::default())
        } else {
            Genesis::default()
        }
    }
}

// ── GenesisProvider trait implementation ──────

#[async_trait]
impl GenesisProvider for GenesisEngine {
    async fn generate_genesis(
        &self,
        _chain_id: &str,
        validators: &[(ValidatorId, PublicKey, u64)],
    ) -> Result<PathBuf, OrchestratorError> {
        self.generate(validators).await?;
        Ok(self.genesis_path())
    }

    async fn validate_genesis(&self, path: &Path) -> Result<bool, OrchestratorError> {
        let data = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| OrchestratorError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;
        let genesis: Genesis = serde_json::from_str(&data)
            .map_err(|_| OrchestratorError::Genesis("Invalid genesis format".into()))?;
        Ok(validator::validate(&genesis))
    }
}
