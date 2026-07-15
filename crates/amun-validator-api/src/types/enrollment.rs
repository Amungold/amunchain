use crate::types::audit::AuditReport;
use crate::types::capabilities::ValidatorCapabilities;
use crate::types::id::ValidatorId;
use crate::types::manifest::ValidatorManifest;
use crate::types::version::PlatformVersion;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum OperatingMode {
    Validator,
    Archive,
    Observer,
    Bootstrap,
    Seed,
    Developer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum EnrollmentPhase {
    MachineAudit,
    IdentitySetup,
    KeyGeneration,
    CertificateIssuance,
    DirectoryCreation,
    ConfigGeneration,
    GenesisDownload,
    GenesisVerification,
    StorageInitialization,
    NetworkInitialization,
    PeerDiscovery,
    HistoricalSync,
    StateVerification,
    HealthCheck,
    ConsensusJoin,
    Completed,
}

impl EnrollmentPhase {
    pub fn all() -> Vec<EnrollmentPhase> {
        vec![
            EnrollmentPhase::MachineAudit,
            EnrollmentPhase::IdentitySetup,
            EnrollmentPhase::KeyGeneration,
            EnrollmentPhase::CertificateIssuance,
            EnrollmentPhase::DirectoryCreation,
            EnrollmentPhase::ConfigGeneration,
            EnrollmentPhase::GenesisDownload,
            EnrollmentPhase::GenesisVerification,
            EnrollmentPhase::StorageInitialization,
            EnrollmentPhase::NetworkInitialization,
            EnrollmentPhase::PeerDiscovery,
            EnrollmentPhase::HistoricalSync,
            EnrollmentPhase::StateVerification,
            EnrollmentPhase::HealthCheck,
            EnrollmentPhase::ConsensusJoin,
            EnrollmentPhase::Completed,
        ]
    }

    pub fn description(&self) -> &str {
        match self {
            EnrollmentPhase::MachineAudit => "Auditing machine resources",
            EnrollmentPhase::IdentitySetup => "Setting up validator identity",
            EnrollmentPhase::KeyGeneration => "Generating cryptographic keys",
            EnrollmentPhase::CertificateIssuance => "Issuing constitutional certificate",
            EnrollmentPhase::DirectoryCreation => "Creating validator directory structure",
            EnrollmentPhase::ConfigGeneration => "Generating configuration files",
            EnrollmentPhase::GenesisDownload => "Downloading genesis file",
            EnrollmentPhase::GenesisVerification => "Verifying genesis integrity",
            EnrollmentPhase::StorageInitialization => "Initializing storage engine",
            EnrollmentPhase::NetworkInitialization => "Initializing network stack",
            EnrollmentPhase::PeerDiscovery => "Discovering network peers",
            EnrollmentPhase::HistoricalSync => "Syncing historical blocks",
            EnrollmentPhase::StateVerification => "Verifying state integrity",
            EnrollmentPhase::HealthCheck => "Running health checks",
            EnrollmentPhase::ConsensusJoin => "Joining consensus",
            EnrollmentPhase::Completed => "Enrollment complete",
        }
    }

    pub fn rollback_target(&self) -> Option<EnrollmentPhase> {
        match self {
            EnrollmentPhase::MachineAudit => None,
            EnrollmentPhase::IdentitySetup => Some(EnrollmentPhase::MachineAudit),
            EnrollmentPhase::KeyGeneration => Some(EnrollmentPhase::IdentitySetup),
            EnrollmentPhase::CertificateIssuance => Some(EnrollmentPhase::KeyGeneration),
            EnrollmentPhase::DirectoryCreation => Some(EnrollmentPhase::CertificateIssuance),
            EnrollmentPhase::ConfigGeneration => Some(EnrollmentPhase::DirectoryCreation),
            EnrollmentPhase::GenesisDownload => Some(EnrollmentPhase::ConfigGeneration),
            EnrollmentPhase::GenesisVerification => Some(EnrollmentPhase::GenesisDownload),
            EnrollmentPhase::StorageInitialization => Some(EnrollmentPhase::GenesisVerification),
            EnrollmentPhase::NetworkInitialization => Some(EnrollmentPhase::StorageInitialization),
            EnrollmentPhase::PeerDiscovery => Some(EnrollmentPhase::NetworkInitialization),
            EnrollmentPhase::HistoricalSync => Some(EnrollmentPhase::PeerDiscovery),
            EnrollmentPhase::StateVerification => Some(EnrollmentPhase::HistoricalSync),
            EnrollmentPhase::HealthCheck => Some(EnrollmentPhase::StateVerification),
            EnrollmentPhase::ConsensusJoin => Some(EnrollmentPhase::HealthCheck),
            EnrollmentPhase::Completed => Some(EnrollmentPhase::ConsensusJoin),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnrollmentConfig {
    pub validator_name: String,
    pub chain_id: String,
    pub operating_mode: OperatingMode,
    pub authority_key: [u8; 32],
    pub bootstrap_peers: Vec<String>,
    pub data_dir: PathBuf,
    pub capabilities: ValidatorCapabilities,
    pub platform_version: PlatformVersion,
}

#[derive(Debug, Clone)]
pub struct EnrollmentContext {
    pub config: EnrollmentConfig,
    pub manifest: Option<ValidatorManifest>,
    pub validator_id: Option<ValidatorId>,
    pub current_phase: EnrollmentPhase,
    pub completed_phases: Vec<EnrollmentPhase>,
    pub failed_phases: Vec<(EnrollmentPhase, String)>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl EnrollmentContext {
    pub fn new(config: EnrollmentConfig) -> Self {
        EnrollmentContext {
            config,
            manifest: None,
            validator_id: None,
            current_phase: EnrollmentPhase::MachineAudit,
            completed_phases: vec![],
            failed_phases: vec![],
            errors: vec![],
            warnings: vec![],
        }
    }

    pub fn advance_phase(&mut self, next: EnrollmentPhase) {
        self.completed_phases.push(self.current_phase);
        self.current_phase = next;
    }

    pub fn record_failure(&mut self, phase: EnrollmentPhase, error: String) {
        self.failed_phases.push((phase, error));
    }

    pub fn rollback_target(&self) -> Option<EnrollmentPhase> {
        self.current_phase.rollback_target()
    }
}

#[derive(Debug, Clone)]
pub struct EnrollmentResult {
    pub validator_id: ValidatorId,
    pub success: bool,
    pub manifest: ValidatorManifest,
    pub phases_completed: Vec<EnrollmentPhase>,
    pub phases_failed: Vec<(EnrollmentPhase, String)>,
    pub audit_report: AuditReport,
    pub duration_secs: u64,
    pub data_dir: PathBuf,
}
