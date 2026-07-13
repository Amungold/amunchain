use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventSeverity {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    Audit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrchestratorEvent {
    BuildStarted {
        crate_count: usize,
    },
    BuildSucceeded {
        duration_ms: u64,
        crates_rebuilt: usize,
    },
    BuildSkipped {
        reason: String,
    },
    BuildFailed {
        error: String,
    },

    GenesisGenerated {
        path: String,
    },
    GenesisAlreadyExists {
        path: String,
    },
    GenesisValidationFailed {
        error: String,
    },

    CertificateGenerated {
        validator: String,
        path: String,
    },
    CertificateAlreadyExists {
        validator: String,
    },
    AllCertificatesReady {
        count: usize,
    },

    ValidatorStarted {
        name: String,
        pid: u32,
        listen_port: u16,
        rpc_port: u16,
    },
    ValidatorStopped {
        name: String,
        reason: String,
        exit_code: Option<i32>,
    },
    ValidatorCrashed {
        name: String,
        error: String,
        crash_count: u32,
    },
    ValidatorRecovered {
        name: String,
    },
    ValidatorAdded {
        name: String,
    },
    ValidatorRemoved {
        name: String,
    },

    RpcStarted {
        port: u16,
    },
    RpcStopped,
    RpcFailed {
        error: String,
    },
    ExplorerApiStarted {
        port: u16,
    },
    ExplorerApiStopped,
    ExplorerUiStarted {
        port: u16,
    },
    ExplorerUiStopped,
    WebSocketStarted {
        port: u16,
    },
    WebSocketStopped,

    HealthCheckCycleStarted,
    HealthCheckPassed {
        components_checked: usize,
        health_score: u8,
    },
    HealthCheckFailed {
        component: String,
        reason: String,
    },
    HealthCheckRecovered {
        component: String,
    },
    AutoRecoveryTriggered {
        component: String,
        attempt: u32,
    },

    RuntimeStateTransition {
        from: String,
        to: String,
    },
    DeploymentStateTransition {
        from: String,
        to: String,
    },

    QuorumReached {
        validators: usize,
        total_power: u64,
    },
    QuorumLost,
    PeerConnected {
        peer_id: String,
    },
    PeerDisconnected {
        peer_id: String,
    },

    DeploymentStarted {
        environment: String,
    },
    DeploymentCompleted {
        environment: String,
    },
    DeploymentFailed {
        error: String,
    },
    RollbackStarted,
    RollbackCompleted,

    MaintenanceModeEntered {
        reason: String,
    },
    MaintenanceModeExited,
    SnapshotCreated {
        path: String,
        size_bytes: u64,
    },
    BackupCreated {
        path: String,
        size_bytes: u64,
    },
}

impl OrchestratorEvent {
    pub fn severity(&self) -> EventSeverity {
        use OrchestratorEvent::*;
        match self {
            AutoRecoveryTriggered { .. } => EventSeverity::Critical,
            ValidatorCrashed { .. } => EventSeverity::Error,
            BuildFailed { .. }
            | GenesisValidationFailed { .. }
            | RpcFailed { .. }
            | HealthCheckFailed { .. }
            | DeploymentFailed { .. }
            | QuorumLost => EventSeverity::Warning,
            GenesisGenerated { .. } | CertificateGenerated { .. } => EventSeverity::Audit,
            _ => EventSeverity::Info,
        }
    }
}

// Global sequence — SeqCst ensures audit-grade ordering
static NEXT_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub sequence: u64,
    pub timestamp: SystemTime,
    pub source: String,
    pub correlation_id: Option<Uuid>,
    pub severity: EventSeverity,
    pub event: OrchestratorEvent,
}

impl EventEnvelope {
    pub fn new(source: impl Into<String>, event: OrchestratorEvent) -> Self {
        let severity = event.severity();
        Self {
            id: Uuid::new_v4(),
            sequence: NEXT_SEQUENCE.fetch_add(1, Ordering::SeqCst),
            timestamp: SystemTime::now(),
            source: source.into(),
            correlation_id: None,
            severity,
            event,
        }
    }

    pub fn with_correlation(mut self, id: Uuid) -> Self {
        self.correlation_id = Some(id);
        self
    }
}

pub struct EventBus {
    sender: broadcast::Sender<EventEnvelope>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn publish(&self, envelope: EventEnvelope) {
        tracing::debug!(id = %envelope.id, seq = envelope.sequence, source = %envelope.source,
                        severity = ?envelope.severity, event = ?envelope.event, "Event published");
        let _ = self.sender.send(envelope);
    }

    pub fn emit(&self, source: impl Into<String>, event: OrchestratorEvent) {
        self.publish(EventEnvelope::new(source, event));
    }

    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.sender.subscribe()
    }

    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(512)
    }
}
