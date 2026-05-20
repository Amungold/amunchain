/// Canonical validator ordering law.
/// All QCs MUST have validators in strictly increasing order by ID.
pub fn canonical_validator_order(ids: &[u64]) -> bool {
    if ids.is_empty() {
        return true;
    }
    for i in 1..ids.len() {
        if ids[i] <= ids[i - 1] {
            return false;
        }
    }
    true
}

/// Validate that all validator IDs are unique and sorted.
pub fn validate_ordering(ids: &[u64]) -> Result<(), &'static str> {
    if !canonical_validator_order(ids) {
        return Err("validators not in canonical order");
    }
    // Check uniqueness via ordering
    for i in 1..ids.len() {
        if ids[i] == ids[i - 1] {
            return Err("duplicate validator in ordering");
        }
    }
    Ok(())
}
