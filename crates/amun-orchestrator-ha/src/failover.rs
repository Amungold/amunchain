/// Failover configuration and logic.
#[derive(Debug, Clone)]
pub struct FailoverConfig {
    /// Seconds without heartbeat before declaring leader dead.
    pub leader_timeout_secs: u64,
    /// Seconds between failover checks.
    pub check_interval_secs: u64,
    /// Maximum number of failovers before alerting.
    pub max_failovers: u32,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            leader_timeout_secs: 30,
            check_interval_secs: 5,
            max_failovers: 3,
        }
    }
}

/// Tracks failover events.
#[derive(Debug, Clone, Default)]
pub struct FailoverTracker {
    pub failover_count: u32,
    pub last_failover: Option<chrono::DateTime<chrono::Utc>>,
    pub failed_leader_id: Option<String>,
}

impl FailoverTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a failover event.
    pub fn record(&mut self, failed_leader_id: String) {
        self.failover_count += 1;
        self.last_failover = Some(chrono::Utc::now());
        self.failed_leader_id = Some(failed_leader_id);
    }

    /// Check if failover threshold exceeded.
    pub fn threshold_exceeded(&self, max: u32) -> bool {
        self.failover_count >= max
    }

    /// Reset failover count.
    pub fn reset(&mut self) {
        self.failover_count = 0;
        self.last_failover = None;
        self.failed_leader_id = None;
    }
}
