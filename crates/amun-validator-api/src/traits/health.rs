use crate::error::PlatformResult;
use crate::types::audit::AuditFinding;
use crate::types::health::HealthStatus;

pub trait HealthProvider: Send + Sync {
    fn status(&self) -> PlatformResult<HealthStatus>;
    fn cpu_usage(&self) -> PlatformResult<f64>;
    fn memory_usage(&self) -> PlatformResult<f64>;
    fn disk_free(&self) -> PlatformResult<u64>;
    fn is_time_synced(&self) -> PlatformResult<bool>;
    fn run_checklist(&self) -> PlatformResult<Vec<AuditFinding>>;
}
