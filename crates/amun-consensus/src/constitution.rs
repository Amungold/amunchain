#![forbid(unsafe_code)]

//! Consensus Constitution - Frozen constants and rules

/// Consensus constitution version
pub const CONSTITUTION_VERSION: &str = "1.0.0";

/// Arithmetic scale factor (6 decimal digits)
pub const SCALE: i64 = 1_000_000;

/// Maximum fixed value (constitutional bound)
pub const MAX_FIXED: i64 = 10_000_000_000 * SCALE;

/// Event type priorities for canonical ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventPriority {
    Mint = 1,
    Reward = 2,
    Transfer = 3,
    Delegate = 4,
    Undelegate = 5,
    Slash = 6,
    Burn = 7,
}

impl EventPriority {
    pub fn from_event_type(event_type: u8) -> Option<Self> {
        match event_type {
            1 => Some(Self::Mint),
            2 => Some(Self::Reward),
            3 => Some(Self::Transfer),
            4 => Some(Self::Delegate),
            5 => Some(Self::Undelegate),
            6 => Some(Self::Slash),
            7 => Some(Self::Burn),
            _ => None,
        }
    }
    
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Byzantine fault tolerance quorum size
/// Returns 2f + 1 where f = floor((N - 1) / 3)
pub const fn quorum_size(total_validators: u32) -> u32 {
    if total_validators == 0 {
        return 0;
    }
    let f = (total_validators - 1) / 3;
    (2 * f) + 1
}

/// Minimum number of validators required for BFT safety
pub const fn min_validators() -> u32 {
    4  // 2f+1 with f=1 requires N>=4
}

/// Check if a validator set size is constitutional
pub const fn is_valid_validator_count(count: u32) -> bool {
    count >= min_validators()
}

/// Canonical hash function identifier
pub const HASH_FUNCTION: &str = "SHA-256";

/// Canonical byte order
pub const BYTE_ORDER: &str = "big-endian";

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quorum_size() {
        assert_eq!(quorum_size(1), 1);
        assert_eq!(quorum_size(2), 1);
        assert_eq!(quorum_size(3), 1);
        assert_eq!(quorum_size(4), 3);
        assert_eq!(quorum_size(5), 3);
        assert_eq!(quorum_size(6), 3);
        assert_eq!(quorum_size(7), 5);
        assert_eq!(quorum_size(10), 7);
    }
    
    #[test]
    fn test_min_validators() {
        assert!(is_valid_validator_count(4));
        assert!(!is_valid_validator_count(3));
        assert!(!is_valid_validator_count(2));
        assert!(!is_valid_validator_count(1));
    }
}
