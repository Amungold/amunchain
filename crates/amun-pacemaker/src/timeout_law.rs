// Merged from amun-timeout-law (Phase 48 Merge Strategy A)
// Timeout calculation laws for pacemaker

/// Calculate exponential backoff timeout
pub fn calculate_timeout(base_ms: u64, consecutive_timeouts: u64, max_ms: u64) -> u64 {
    let multiplier = 1.5f64.powi(consecutive_timeouts as i32);
    let ms = (base_ms as f64 * multiplier) as u64;
    ms.min(max_ms)
}

/// Check if node should halt due to excessive timeouts
pub fn should_halt(consecutive_timeouts: u64, max_allowed: u64) -> bool {
    consecutive_timeouts > max_allowed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff() {
        let t0 = calculate_timeout(1000, 0, 60000);
        assert_eq!(t0, 1000);
        let t1 = calculate_timeout(1000, 1, 60000);
        assert_eq!(t1, 1500);
        let t2 = calculate_timeout(1000, 2, 60000);
        assert_eq!(t2, 2250);
    }

    #[test]
    fn test_max_cap() {
        let result = calculate_timeout(1000, 100, 10000);
        assert_eq!(result, 10000);
    }

    #[test]
    fn test_halt_detection() {
        assert!(!should_halt(5, 10));
        assert!(should_halt(11, 10));
    }
}
