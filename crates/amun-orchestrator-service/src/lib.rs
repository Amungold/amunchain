pub mod health_checker;
pub mod process;
pub mod supervisor;

use amun_orchestrator_core::error::OrchestratorError;
use amun_orchestrator_core::event::EventBus;
use amun_orchestrator_core::state::{ServiceKind, ServiceStatus};
use amun_orchestrator_core::traits::ProcessManager;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ServiceManager {
    event_bus: Arc<EventBus>,
    processes: RwLock<HashMap<String, process::ProcessHandle>>,
    statuses: RwLock<HashMap<ServiceKind, ServiceStatus>>,
    supervisor: RwLock<supervisor::Supervisor>,
}

impl ServiceManager {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            event_bus,
            processes: RwLock::new(HashMap::new()),
            statuses: RwLock::new(HashMap::new()),
            supervisor: RwLock::new(supervisor::Supervisor::new()),
        }
    }

    pub async fn start_service(
        &self,
        name: &str,
        kind: ServiceKind,
        command: &str,
        args: &[String],
    ) -> Result<u32, OrchestratorError> {
        if let Some(handle) = self.processes.read().await.get(name) {
            if handle.is_alive().await {
                tracing::info!(%name, "Service already running");
                return Ok(handle.pid);
            }
        }

        let handle = process::ProcessHandle::spawn(name, command, args).await?;
        let pid = handle.pid;

        self.processes
            .write()
            .await
            .insert(name.to_string(), handle);
        self.statuses
            .write()
            .await
            .insert(kind.clone(), ServiceStatus::running(pid));

        self.emit_service_event(kind, true).await;
        tracing::info!(%name, pid, "Service started");
        Ok(pid)
    }

    pub async fn stop_service(
        &self,
        name: &str,
        kind: &ServiceKind,
    ) -> Result<(), OrchestratorError> {
        if let Some(mut handle) = self.processes.write().await.remove(name) {
            handle.kill().await?;
        }
        if let Some(status) = self.statuses.write().await.get_mut(kind) {
            status.running = false;
            status.healthy = false;
        }
        self.emit_service_event(kind.clone(), false).await;
        tracing::info!(%name, "Service stopped");
        Ok(())
    }

    pub async fn restart_service(
        &self,
        name: &str,
        kind: &ServiceKind,
        command: &str,
        args: &[String],
    ) -> Result<u32, OrchestratorError> {
        self.stop_service(name, kind).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        self.start_service(name, kind.clone(), command, args).await
    }

    pub async fn health_check(&self) -> HashMap<ServiceKind, ServiceStatus> {
        let mut statuses = self.statuses.write().await;
        for (kind, status) in statuses.iter_mut() {
            let name = kind.name().to_string();
            if let Some(handle) = self.processes.read().await.get(&name) {
                let alive = handle.is_alive().await;
                let was_running = status.running;
                status.running = alive;
                status.healthy = alive;

                if !alive && was_running {
                    status.crash_count += 1;
                    status.last_error = Some("Process exited unexpectedly".into());
                    tracing::warn!(service = %name, crash_count = status.crash_count, "Service crashed");
                    self.supervisor.write().await.record_failure(&name);
                }
            }
        }
        statuses.clone()
    }

    pub async fn service_status(&self, kind: &ServiceKind) -> Option<ServiceStatus> {
        self.statuses.read().await.get(kind).cloned()
    }

    pub async fn list_services(&self) -> Vec<ServiceKind> {
        self.statuses.read().await.keys().cloned().collect()
    }

    async fn emit_service_event(&self, kind: ServiceKind, started: bool) {
        use amun_orchestrator_core::event::OrchestratorEvent;
        let event = match kind {
            ServiceKind::Rpc if started => OrchestratorEvent::RpcStarted { port: 0 },
            ServiceKind::Rpc => OrchestratorEvent::RpcStopped,
            ServiceKind::ExplorerApi if started => {
                OrchestratorEvent::ExplorerApiStarted { port: 0 }
            }
            ServiceKind::ExplorerApi => OrchestratorEvent::ExplorerApiStopped,
            ServiceKind::ExplorerUi if started => OrchestratorEvent::ExplorerUiStarted { port: 0 },
            ServiceKind::ExplorerUi => OrchestratorEvent::ExplorerUiStopped,
            ServiceKind::WebSocket if started => OrchestratorEvent::WebSocketStarted { port: 0 },
            ServiceKind::WebSocket => OrchestratorEvent::WebSocketStopped,
            _ => return,
        };
        self.event_bus.emit("service-manager", event);
    }
}

#[async_trait]
impl ProcessManager for ServiceManager {
    async fn start(&self, service_name: &str, args: &[String]) -> Result<u32, OrchestratorError> {
        println!("START service={} args={:?}", service_name, args);
        let command = args
            .first()
            .cloned()
            .unwrap_or_else(|| service_name.to_string());
        let rest: Vec<String> = if args.len() > 1 {
            args[1..].to_vec()
        } else {
            vec![]
        };
        self.start_service(
            service_name,
            ServiceKind::Other(service_name.to_string()),
            &command,
            &rest,
        )
        .await
    }

    async fn stop(&self, service_name: &str) -> Result<(), OrchestratorError> {
        self.stop_service(service_name, &ServiceKind::Other(service_name.to_string()))
            .await
    }

    async fn restart(&self, service_name: &str) -> Result<u32, OrchestratorError> {
        let kind = ServiceKind::Other(service_name.to_string());
        let command = service_name.to_string();
        self.restart_service(service_name, &kind, &command, &[])
            .await
    }

    async fn is_running(&self, service_name: &str) -> Result<bool, OrchestratorError> {
        if let Some(handle) = self.processes.read().await.get(service_name) {
            return Ok(handle.is_alive().await);
        }
        Ok(false)
    }

    async fn pid(&self, service_name: &str) -> Result<Option<u32>, OrchestratorError> {
        if let Some(handle) = self.processes.read().await.get(service_name) {
            return Ok(Some(handle.pid));
        }
        Ok(None)
    }
}
