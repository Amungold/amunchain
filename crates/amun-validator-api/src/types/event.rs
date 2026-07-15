use crate::types::id::ValidatorId;
use crate::types::state::RuntimeState;

#[derive(Debug, Clone)]
pub enum PlatformEvent {
    ValidatorCreated {
        id: ValidatorId,
        name: String,
        timestamp: u64,
    },
    StateTransition {
        from: RuntimeState,
        to: RuntimeState,
        timestamp: u64,
    },
    EnrollmentPhaseChanged {
        validator_id: ValidatorId,
        phase: String,
        timestamp: u64,
    },
    AuditCompleted {
        validator_id: ValidatorId,
        passed: bool,
        timestamp: u64,
    },
    RollbackExecuted {
        validator_id: ValidatorId,
        from_phase: String,
        reason: String,
        timestamp: u64,
    },
    PolicyViolation {
        validator_id: ValidatorId,
        policy: String,
        reason: String,
        timestamp: u64,
    },
}

#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    RuntimeStarted {
        timestamp: u64,
    },
    RuntimeStopped {
        reason: String,
        timestamp: u64,
    },
    StateMachineError {
        error: String,
        timestamp: u64,
    },
    ServiceReady {
        service_name: String,
        timestamp: u64,
    },
    ServiceFailed {
        service_name: String,
        error: String,
        timestamp: u64,
    },
}

#[derive(Debug, Clone)]
pub enum IdentityEvent {
    IdentityVerified {
        certificate_hash: [u8; 32],
        timestamp: u64,
    },
    KeysRotated {
        new_public_key: [u8; 32],
        timestamp: u64,
    },
    CertificateExpiring {
        days_remaining: u32,
        timestamp: u64,
    },
}

#[derive(Debug, Clone)]
pub enum StorageEvent {
    SnapshotCreated {
        path: String,
        size_bytes: u64,
        timestamp: u64,
    },
    WalReplayed {
        entries: u64,
        timestamp: u64,
    },
    CompactionCompleted {
        freed_bytes: u64,
        timestamp: u64,
    },
    StorageWarning {
        reason: String,
        timestamp: u64,
    },
}

#[derive(Debug, Clone)]
pub enum NetworkEvent {
    PeerConnected {
        peer_id: [u8; 32],
        address: String,
        timestamp: u64,
    },
    PeerDisconnected {
        peer_id: [u8; 32],
        reason: String,
        timestamp: u64,
    },
    DiscoveryStarted {
        timestamp: u64,
    },
    DiscoveryFinished {
        peer_count: usize,
        timestamp: u64,
    },
    SyncStarted {
        from_height: u64,
        to_height: u64,
        timestamp: u64,
    },
    SyncProgress {
        current: u64,
        total: u64,
        timestamp: u64,
    },
    SyncFinished {
        blocks_synced: u64,
        duration_ms: u64,
        timestamp: u64,
    },
}

#[derive(Debug, Clone)]
pub enum ConsensusEvent {
    ConsensusJoined {
        validator_id: [u8; 32],
        timestamp: u64,
    },
    ConsensusLeft {
        validator_id: [u8; 32],
        reason: String,
        timestamp: u64,
    },
    ValidatorSuspended {
        validator_id: [u8; 32],
        reason: String,
        timestamp: u64,
    },
    RoundCompleted {
        height: u64,
        round: u64,
        timestamp: u64,
    },
    QuorumReached {
        height: u64,
        approval_power: u64,
        timestamp: u64,
    },
}

#[derive(Debug, Clone)]
pub enum RecoveryEvent {
    RecoveryStarted {
        reason: String,
        from_checkpoint: u64,
        timestamp: u64,
    },
    RecoveryProgress {
        replayed: u64,
        total: u64,
        timestamp: u64,
    },
    RecoveryFinished {
        state_matches: bool,
        timestamp: u64,
    },
}

#[derive(Debug, Clone)]
pub enum AuditEvent {
    AuditStarted {
        domain: String,
        timestamp: u64,
    },
    AuditFinding {
        finding_id: [u8; 16],
        severity: String,
        status: String,
        timestamp: u64,
    },
    AuditCompleted {
        domain: String,
        passed: bool,
        timestamp: u64,
    },
    MainnetReadinessCheck {
        passed: bool,
        timestamp: u64,
    },
}

#[derive(Debug, Clone)]
pub enum EnrollmentEvent {
    EnrollmentStarted {
        config_name: String,
        timestamp: u64,
    },
    PhaseStarted {
        phase: String,
        timestamp: u64,
    },
    PhaseCompleted {
        phase: String,
        timestamp: u64,
    },
    PhaseFailed {
        phase: String,
        error: String,
        timestamp: u64,
    },
    RollbackStarted {
        from_phase: String,
        to_phase: String,
        timestamp: u64,
    },
    RollbackCompleted {
        target_phase: String,
        timestamp: u64,
    },
    ManifestGenerated {
        validator_id: ValidatorId,
        timestamp: u64,
    },
}
