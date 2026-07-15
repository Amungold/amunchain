#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveCapability {
    None,
    Light,
    Full,
    V1,
    V2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotCapability {
    None,
    Create,
    Serve,
    CreateAndServe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncCapability {
    Full,
    Fast,
    Historical,
    LightClient,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricsCapability {
    None,
    Basic,
    Advanced,
    Prometheus,
    OpenTelemetry,
}

#[derive(Debug, Clone)]
pub struct ValidatorCapabilities {
    pub archive: ArchiveCapability,
    pub snapshot: SnapshotCapability,
    pub sync: SyncCapability,
    pub metrics: MetricsCapability,
    pub evidence_service: bool,
    pub hsm_support: bool,
    pub auto_upgrade: bool,
    pub geo_region: Option<String>,
    pub max_connections: u32,
}

impl Default for ValidatorCapabilities {
    fn default() -> Self {
        ValidatorCapabilities {
            archive: ArchiveCapability::None,
            snapshot: SnapshotCapability::None,
            sync: SyncCapability::Fast,
            metrics: MetricsCapability::Basic,
            evidence_service: false,
            hsm_support: false,
            auto_upgrade: false,
            geo_region: None,
            max_connections: 50,
        }
    }
}
