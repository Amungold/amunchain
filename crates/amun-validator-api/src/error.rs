use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorClass {
    Fatal,
    Retryable,
    Recoverable,
    Warning,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlatformError {
    Identity(IdentityError),
    Storage(StorageError),
    Network(NetworkError),
    Genesis(GenesisError),
    Sync(SyncError),
    Health(HealthError),
    Consensus(ConsensusError),
    Recovery(RecoveryError),
    Audit(AuditError),
    Config(ConfigError),
    Enrollment(EnrollmentError),
    StateMachine(StateMachineError),
    Internal(String),
}

impl std::error::Error for PlatformError {}

impl PlatformError {
    pub fn class(&self) -> ErrorClass {
        match self {
            PlatformError::Identity(e) => e.class(),
            PlatformError::Storage(e) => e.class(),
            PlatformError::Network(e) => e.class(),
            PlatformError::Genesis(e) => e.class(),
            PlatformError::Sync(e) => e.class(),
            PlatformError::Health(e) => e.class(),
            PlatformError::Consensus(e) => e.class(),
            PlatformError::Recovery(e) => e.class(),
            PlatformError::Audit(e) => e.class(),
            PlatformError::Config(e) => e.class(),
            PlatformError::Enrollment(e) => e.class(),
            PlatformError::StateMachine(e) => e.class(),
            PlatformError::Internal(_) => ErrorClass::Fatal,
        }
    }
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformError::Identity(e) => write!(f, "Identity error: {}", e),
            PlatformError::Storage(e) => write!(f, "Storage error: {}", e),
            PlatformError::Network(e) => write!(f, "Network error: {}", e),
            PlatformError::Genesis(e) => write!(f, "Genesis error: {}", e),
            PlatformError::Sync(e) => write!(f, "Sync error: {}", e),
            PlatformError::Health(e) => write!(f, "Health error: {}", e),
            PlatformError::Consensus(e) => write!(f, "Consensus error: {}", e),
            PlatformError::Recovery(e) => write!(f, "Recovery error: {}", e),
            PlatformError::Audit(e) => write!(f, "Audit error: {}", e),
            PlatformError::Config(e) => write!(f, "Config error: {}", e),
            PlatformError::Enrollment(e) => write!(f, "Enrollment error: {}", e),
            PlatformError::StateMachine(e) => write!(f, "State machine error: {}", e),
            PlatformError::Internal(e) => write!(f, "Internal error: {}", e),
        }
    }
}

pub type PlatformResult<T> = Result<T, PlatformError>;

macro_rules! define_error {
    ($name:ident, $code_name:ident, $default_class:expr, { $($variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            pub code: $code_name,
            pub message: String,
            pub class: ErrorClass,
        }

        impl std::error::Error for $name {}

        impl $name {
            pub fn new(code: $code_name, message: String) -> Self {
                Self { code, message, class: $default_class }
            }

            pub fn with_class(code: $code_name, message: String, class: ErrorClass) -> Self {
                Self { code, message, class }
            }

            pub fn class(&self) -> ErrorClass {
                self.class
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{:?}] {}", self.code, self.message)
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $code_name {
            $($variant),+
        }
    };
}

define_error!(IdentityError, IdentityErrorCode, ErrorClass::Fatal, {
    CertificateExpired,
    CertificateInvalid,
    AuthorityUnknown,
    ChainIdMismatch,
    KeyRotationFailed,
    HsmError,
    KeyNotFound,
    SignatureInvalid,
});

define_error!(StorageError, StorageErrorCode, ErrorClass::Recoverable, {
    DatabaseCorrupted,
    InsufficientSpace,
    SnapshotFailed,
    SnapshotCorrupted,
    WalReplayFailed,
    WalCorrupted,
    StateRootMismatch,
    CompactionFailed,
    InitializationFailed,
});

define_error!(NetworkError, NetworkErrorCode, ErrorClass::Retryable, {
    PortUnavailable,
    ConnectionRefused,
    HandshakeFailed,
    RateLimited,
    NatTraversalFailed,
    DnsResolutionFailed,
    BootstrapPeerUnreachable,
});

define_error!(GenesisError, GenesisErrorCode, ErrorClass::Fatal, {
    DownloadFailed,
    HashMismatch,
    StateRootInvalid,
    ValidatorListEmpty,
    ChainIdMismatch,
    ProtocolVersionIncompatible,
});

define_error!(SyncError, SyncErrorCode, ErrorClass::Retryable, {
    NoPeersAvailable,
    BlockVerificationFailed,
    StateVerificationFailed,
    TimeoutExceeded,
    DivergentState,
    SnapshotDownloadFailed,
});

define_error!(HealthError, HealthErrorCode, ErrorClass::Warning, {
    CpuThresholdExceeded,
    MemoryThresholdExceeded,
    DiskSpaceLow,
    TimeNotSynchronized,
    DatabaseUnhealthy,
    NetworkUnstable,
});

define_error!(ConsensusError, ConsensusErrorCode, ErrorClass::Retryable, {
    QuorumNotReached,
    RoundTimeout,
    DuplicateVote,
    InvalidProposal,
    NotActiveValidator,
    JoinRejected,
});

define_error!(RecoveryError, RecoveryErrorCode, ErrorClass::Recoverable, {
    CheckpointNotFound,
    WalCorrupted,
    StateMismatch,
    ReplayTimeout,
    SnapshotCorrupted,
});

define_error!(AuditError, AuditErrorCode, ErrorClass::Warning, {
    CriticalFinding,
    DomainFailed,
    ReportGenerationFailed,
    MainnetReadinessFailed,
});

define_error!(ConfigError, ConfigErrorCode, ErrorClass::Fatal, {
    MissingField,
    InvalidValue,
    FileNotFound,
    PermissionDenied,
    ParseFailed,
});

define_error!(EnrollmentError, EnrollmentErrorCode, ErrorClass::Retryable, {
    PhaseFailed,
    ChecklistNotMet,
    AuditFailed,
    PrerequisiteMissing,
    TimeoutExceeded,
    RollbackFailed,
    PolicyViolation,
});

define_error!(StateMachineError, StateMachineErrorCode, ErrorClass::Fatal, {
    IllegalTransition,
    PreconditionFailed,
    PostconditionFailed,
    TimeoutExceeded,
});
