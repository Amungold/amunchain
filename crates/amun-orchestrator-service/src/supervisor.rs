use std::collections::HashMap;
use std::time::Instant;

/// Tracks failures and implements backoff for auto-recovery.
pub struct Supervisor {
    failures: HashMap<String, Vec<Instant>>,
    max_restart_attempts: u32,
    cooldown_secs: u64,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            failures: HashMap::new(),
            max_restart_attempts: 3,
            cooldown_secs: 30,
        }
    }

    /// Record a failure for a service.
    pub fn record_failure(&mut self, service_name: &str) {
        let entry = self.failures.entry(service_name.to_string()).or_default();
        entry.push(Instant::now());

        // Prune old failures outside the cooldown window
        let cutoff = Instant::now() - std::time::Duration::from_secs(self.cooldown_secs);
        entry.retain(|t| *t > cutoff);
    }

    /// Check if a service should be restarted.
    pub fn should_restart(&self, service_name: &str) -> bool {
        if let Some(entries) = self.failures.get(service_name) {
            if entries.len() >= self.max_restart_attempts as usize {
                tracing::warn!(
                    service = %service_name,
                    failures = entries.len(),
                    "Max restart attempts reached — cooling down"
                );
                return false;
            }
        }
        true
    }

    /// Reset failure count for a service after successful recovery.
    pub fn reset(&mut self, service_name: &str) {
        self.failures.remove(service_name);
    }

    /// Get failure count for a service.
    pub fn failure_count(&self, service_name: &str) -> usize {
        self.failures
            .get(service_name)
            .map(|v| v.len())
            .unwrap_or(0)
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_restart_under_limit() {
        let mut sup = Supervisor::new();
        sup.record_failure("test");
        sup.record_failure("test");
        assert!(sup.should_restart("test"));
    }

    #[test]
    fn test_should_not_restart_over_limit() {
        let mut sup = Supervisor::new();
        for _ in 0..5 {
            sup.record_failure("test");
        }
        assert!(!sup.should_restart("test"));
    }

    #[test]
    fn test_reset_clears_failures() {
        let mut sup = Supervisor::new();
        sup.record_failure("test");
        sup.reset("test");
        assert_eq!(sup.failure_count("test"), 0);
    }
}
