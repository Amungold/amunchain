use super::types::Genesis;

/// Validate genesis structure.
pub fn validate(genesis: &Genesis) -> bool {
    // Chain ID must not be empty
    if genesis.chain_id.is_empty() {
        tracing::warn!("Genesis validation failed: empty chain_id");
        return false;
    }

    // Must have at least 1 validator (for production; dev can have 0)
    // Allow 0 validators for bootstrap phase
    if genesis.validator_count() == 0 {
        tracing::info!("Genesis has 0 validators — bootstrap phase");
    }

    // Check for duplicate validator IDs
    let mut ids: Vec<_> = genesis.validators.iter().map(|v| v.validator_id).collect();
    let len_before = ids.len();
    ids.sort();
    ids.dedup();
    if ids.len() != len_before {
        tracing::warn!("Genesis validation failed: duplicate validator IDs");
        return false;
    }

    // Genesis hash must not be empty (should have been computed)
    if genesis.genesis_hash.is_empty() {
        tracing::warn!("Genesis validation failed: empty genesis_hash");
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::create_genesis;
    use amun_orchestrator_core::types::{PublicKey, ValidatorId};

    #[test]
    fn test_validate_valid_genesis() {
        let validators = vec![
            (ValidatorId([1u8; 32]), PublicKey([10u8; 32]), 100),
            (ValidatorId([2u8; 32]), PublicKey([20u8; 32]), 100),
        ];
        let genesis = create_genesis("test-chain", &validators);
        assert!(validate(&genesis));
    }

    #[test]
    fn test_validate_empty_chain_id_fails() {
        let mut genesis = create_genesis("test-chain", &[]);
        genesis.chain_id = String::new();
        genesis.genesis_hash = "dummy".into();
        assert!(!validate(&genesis));
    }

    #[test]
    fn test_validate_zero_validators_allowed() {
        let genesis = create_genesis("test-chain", &[]);
        assert!(validate(&genesis));
    }
}
