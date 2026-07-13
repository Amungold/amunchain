use async_trait::async_trait;
use std::path::PathBuf;

use crate::error::OrchestratorError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildStatus {
    Success,
    Failed,
    Cached,
    Skipped,
}

#[derive(Debug, Clone)]
pub struct BuildResult {
    pub status: BuildStatus,
    pub duration_ms: u64,
    pub crates_rebuilt: usize,
    pub error: Option<String>,
}

#[async_trait]
pub trait Builder: Send + Sync {
    async fn changed_crates(&self) -> Result<Vec<String>, OrchestratorError>;
    async fn build(&self, crates: &[String]) -> Result<BuildResult, OrchestratorError>;
    async fn build_all(&self) -> Result<BuildResult, OrchestratorError>;
}

pub struct CargoBuilder {
    pub workspace_root: PathBuf,
}

impl CargoBuilder {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }
}

#[async_trait]
impl Builder for CargoBuilder {
    async fn changed_crates(&self) -> Result<Vec<String>, OrchestratorError> {
        Ok(vec!["workspace".into()])
    }

    async fn build(&self, _crates: &[String]) -> Result<BuildResult, OrchestratorError> {
        let start = std::time::Instant::now();
        let output = std::process::Command::new("cargo")
            .args(["build", "--workspace"])
            .current_dir(&self.workspace_root)
            .output()
            .map_err(|e| OrchestratorError::Process {
                command: "cargo build".into(),
                message: e.to_string(),
            })?;
        let duration_ms = start.elapsed().as_millis() as u64;
        if output.status.success() {
            Ok(BuildResult {
                status: BuildStatus::Success,
                duration_ms,
                crates_rebuilt: 0,
                error: None,
            })
        } else {
            Ok(BuildResult {
                status: BuildStatus::Failed,
                duration_ms,
                crates_rebuilt: 0,
                error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
            })
        }
    }

    async fn build_all(&self) -> Result<BuildResult, OrchestratorError> {
        self.build(&["workspace".into()]).await
    }
}
