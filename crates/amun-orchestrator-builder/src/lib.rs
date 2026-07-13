pub mod dependency_graph;
pub mod fingerprinter;

use amun_orchestrator_core::builder::{BuildResult, BuildStatus, Builder};
use amun_orchestrator_core::OrchestratorError;
use async_trait::async_trait;
use std::collections::HashSet;
use std::path::PathBuf;
use tokio::sync::Mutex;

pub struct IncrementalBuilder {
    pub workspace_root: PathBuf,
    graph: dependency_graph::DependencyGraph,
    fingerprinter: Mutex<fingerprinter::SourceFingerprinter>,
}

impl IncrementalBuilder {
    pub async fn new(workspace_root: PathBuf) -> Result<Self, OrchestratorError> {
        let graph = dependency_graph::DependencyGraph::build(&workspace_root).await?;
        let fingerprinter =
            fingerprinter::SourceFingerprinter::new(&workspace_root, &graph.workspace_members);
        Ok(Self {
            workspace_root,
            graph,
            fingerprinter: Mutex::new(fingerprinter),
        })
    }

    pub async fn changed_crates_detailed(&self) -> Result<HashSet<String>, OrchestratorError> {
        self.fingerprinter.lock().await.find_changed().await
    }

    pub fn affected_crates(&self, changed: &HashSet<String>) -> HashSet<String> {
        self.graph.transitive_dependents(changed)
    }
}

#[async_trait]
impl Builder for IncrementalBuilder {
    async fn changed_crates(&self) -> Result<Vec<String>, OrchestratorError> {
        let changed = self.changed_crates_detailed().await?;
        let affected = self.affected_crates(&changed);
        let all: HashSet<_> = changed.union(&affected).cloned().collect();
        Ok(all.into_iter().collect())
    }

    async fn build(&self, crates: &[String]) -> Result<BuildResult, OrchestratorError> {
        if crates.is_empty() || crates.iter().all(|c| c == "workspace") {
            return self.build_all().await;
        }

        let start = std::time::Instant::now();
        let mut cmd = tokio::process::Command::new("cargo");
        cmd.arg("build");
        for c in crates {
            cmd.args(["-p", c]);
        }
        cmd.current_dir(&self.workspace_root);

        let output = cmd.output().await.map_err(|e| OrchestratorError::Process {
            command: format!("cargo build -p {}", crates.join(",")),
            message: e.to_string(),
        })?;

        let duration_ms = start.elapsed().as_millis() as u64;

        if output.status.success() {
            self.fingerprinter.lock().await.persist().await?;
            Ok(BuildResult {
                status: BuildStatus::Success,
                duration_ms,
                crates_rebuilt: crates.len(),
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
        let start = std::time::Instant::now();
        let output = tokio::process::Command::new("cargo")
            .args(["build", "--workspace"])
            .current_dir(&self.workspace_root)
            .output()
            .await
            .map_err(|e| OrchestratorError::Process {
                command: "cargo build --workspace".into(),
                message: e.to_string(),
            })?;

        let duration_ms = start.elapsed().as_millis() as u64;

        if output.status.success() {
            self.fingerprinter.lock().await.persist().await?;
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
}
