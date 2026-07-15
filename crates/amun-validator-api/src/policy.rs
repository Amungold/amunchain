use crate::types::capabilities::{
    ArchiveCapability, MetricsCapability, SnapshotCapability, SyncCapability, ValidatorCapabilities,
};

#[derive(Debug, Clone)]
pub struct ValidatorPolicy {
    pub minimum_protocol_version: u32,
    pub minimum_identity_version: u32,
    pub minimum_storage_version: u32,
    pub minimum_consensus_version: u32,
    pub minimum_ram_bytes: u64,
    pub minimum_disk_bytes: u64,
    pub minimum_cpu_cores: u32,
    pub required_capabilities: ValidatorCapabilities,
    pub max_slash_count: u32,
    pub max_downtime_seconds: u64,
    pub require_time_sync: bool,
    pub require_hsm: bool,
    pub require_snapshot_verification: bool,
    pub require_genesis_verification: bool,
}

impl Default for ValidatorPolicy {
    fn default() -> Self {
        ValidatorPolicy {
            minimum_protocol_version: 1,
            minimum_identity_version: 1,
            minimum_storage_version: 1,
            minimum_consensus_version: 1,
            minimum_ram_bytes: 8_589_934_592,
            minimum_disk_bytes: 100_000_000_000,
            minimum_cpu_cores: 4,
            required_capabilities: ValidatorCapabilities::default(),
            max_slash_count: 5,
            max_downtime_seconds: 3600,
            require_time_sync: true,
            require_hsm: false,
            require_snapshot_verification: true,
            require_genesis_verification: true,
        }
    }
}

impl ValidatorPolicy {
    pub fn validate_capabilities(
        &self,
        capabilities: &ValidatorCapabilities,
    ) -> Result<(), String> {
        let required = &self.required_capabilities;
        if !Self::archive_meets(&capabilities.archive, &required.archive) {
            return Err(format!(
                "Archive capability insufficient: {:?} < {:?}",
                capabilities.archive, required.archive
            ));
        }
        if !Self::snapshot_meets(&capabilities.snapshot, &required.snapshot) {
            return Err(format!(
                "Snapshot capability insufficient: {:?} < {:?}",
                capabilities.snapshot, required.snapshot
            ));
        }
        if !Self::sync_meets(&capabilities.sync, &required.sync) {
            return Err(format!(
                "Sync capability insufficient: {:?} < {:?}",
                capabilities.sync, required.sync
            ));
        }
        if !Self::metrics_meets(&capabilities.metrics, &required.metrics) {
            return Err(format!(
                "Metrics capability insufficient: {:?} < {:?}",
                capabilities.metrics, required.metrics
            ));
        }
        if required.evidence_service && !capabilities.evidence_service {
            return Err("Evidence service required by policy".into());
        }
        if required.hsm_support && !capabilities.hsm_support {
            return Err("HSM support required by policy".into());
        }
        if capabilities.max_connections < required.max_connections {
            return Err(format!(
                "Max connections {} below required {}",
                capabilities.max_connections, required.max_connections
            ));
        }
        Ok(())
    }

    fn archive_meets(actual: &ArchiveCapability, required: &ArchiveCapability) -> bool {
        matches!(
            (actual, required),
            (ArchiveCapability::None, ArchiveCapability::None)
                | (ArchiveCapability::V2, _)
                | (ArchiveCapability::V1, ArchiveCapability::V1)
                | (ArchiveCapability::V1, ArchiveCapability::None)
                | (ArchiveCapability::V1, ArchiveCapability::Light)
                | (ArchiveCapability::Full, ArchiveCapability::Full)
                | (ArchiveCapability::Full, ArchiveCapability::None)
                | (ArchiveCapability::Full, ArchiveCapability::Light)
                | (ArchiveCapability::Light, ArchiveCapability::Light)
                | (ArchiveCapability::Light, ArchiveCapability::None)
        )
    }

    fn snapshot_meets(actual: &SnapshotCapability, required: &SnapshotCapability) -> bool {
        matches!(
            (actual, required),
            (SnapshotCapability::None, SnapshotCapability::None)
                | (SnapshotCapability::CreateAndServe, _)
                | (SnapshotCapability::Serve, SnapshotCapability::Serve)
                | (SnapshotCapability::Serve, SnapshotCapability::None)
                | (SnapshotCapability::Create, SnapshotCapability::Create)
                | (SnapshotCapability::Create, SnapshotCapability::None)
        )
    }

    fn sync_meets(actual: &SyncCapability, required: &SyncCapability) -> bool {
        matches!(
            (actual, required),
            (SyncCapability::Full, _)
                | (SyncCapability::Historical, SyncCapability::Historical)
                | (SyncCapability::Historical, SyncCapability::Fast)
                | (SyncCapability::Historical, SyncCapability::LightClient)
                | (SyncCapability::Fast, SyncCapability::Fast)
                | (SyncCapability::Fast, SyncCapability::LightClient)
                | (SyncCapability::LightClient, SyncCapability::LightClient)
        )
    }

    fn metrics_meets(actual: &MetricsCapability, required: &MetricsCapability) -> bool {
        matches!(
            (actual, required),
            (MetricsCapability::None, MetricsCapability::None)
                | (MetricsCapability::OpenTelemetry, _)
                | (MetricsCapability::Prometheus, MetricsCapability::Prometheus)
                | (MetricsCapability::Prometheus, MetricsCapability::Advanced)
                | (MetricsCapability::Prometheus, MetricsCapability::Basic)
                | (MetricsCapability::Prometheus, MetricsCapability::None)
                | (MetricsCapability::Advanced, MetricsCapability::Advanced)
                | (MetricsCapability::Advanced, MetricsCapability::Basic)
                | (MetricsCapability::Advanced, MetricsCapability::None)
                | (MetricsCapability::Basic, MetricsCapability::Basic)
                | (MetricsCapability::Basic, MetricsCapability::None)
        )
    }

    pub fn validate_resources(
        &self,
        ram_bytes: u64,
        disk_bytes: u64,
        cpu_cores: u32,
    ) -> Result<(), String> {
        if ram_bytes < self.minimum_ram_bytes {
            return Err(format!(
                "Insufficient RAM: {} < {}",
                ram_bytes, self.minimum_ram_bytes
            ));
        }
        if disk_bytes < self.minimum_disk_bytes {
            return Err(format!(
                "Insufficient disk: {} < {}",
                disk_bytes, self.minimum_disk_bytes
            ));
        }
        if cpu_cores < self.minimum_cpu_cores {
            return Err(format!(
                "Insufficient CPU cores: {} < {}",
                cpu_cores, self.minimum_cpu_cores
            ));
        }
        Ok(())
    }
}
