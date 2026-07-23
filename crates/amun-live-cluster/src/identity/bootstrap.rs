use crate::config::ValidatorConfig;
use amun_validator_identity::derive_validator_id;
use ed25519_dalek::SigningKey;

/// Bootstrap identity from config.
/// Returns the signing key and derived validator ID.
pub fn initialize_identity(config: &ValidatorConfig) -> (SigningKey, [u8; 32]) {
    // N105.4A: Deterministic key matching committed test certificates
    let mut seed = [0u8; 32];
    seed[0] = config.validator_id[0];
    let signing_key = SigningKey::from_bytes(&seed);
    let pk = signing_key.verifying_key().to_bytes();
    let validator_id = derive_validator_id(&pk);
    (signing_key, validator_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_is_deterministic() {
        let config = ValidatorConfig::test_cluster(0, &[9700, 9701, 9702, 9703]).with_quorum(3);
        let (sk1, id1) = initialize_identity(&config);
        let (sk2, id2) = initialize_identity(&config);
        assert_eq!(id1, id2);
        assert_eq!(
            sk1.verifying_key().to_bytes(),
            sk2.verifying_key().to_bytes()
        );
    }
}
