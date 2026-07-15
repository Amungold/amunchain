#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning { reason: String },
    Degraded { reason: String },
    Recovering { since: u64 },
    Maintenance { reason: String },
    Offline { since: u64 },
    Failed { reason: String, since: u64 },
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }
    pub fn can_serve(&self) -> bool {
        matches!(self, HealthStatus::Healthy | HealthStatus::Warning { .. })
    }
}
