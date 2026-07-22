use ed25519_dalek::SigningKey;
use amun_authority_registry::AuthorityRegistry;
use amun_consensus_network::engine::ConsensusEngine;
use crate::config::ValidatorConfig;

use super::bootstrap;
use super::certificates;
use super::registry;

/// IdentityService encapsulates all identity-related initialization.
/// ADR-023 Phase 3: Extracted from LiveValidator::new().
pub struct IdentityService;

impl IdentityService {
    /// Initialize identity: create keys, build registry, issue/verify certificates,
    /// register self and all peers in the consensus engine.
    pub fn initialize(
        config: &ValidatorConfig,
        engine: &mut ConsensusEngine,
    ) -> (SigningKey, [u8; 32], AuthorityRegistry) {
        // 1. Create signing key and derive validator ID
        let (signing_key, validator_id) = bootstrap::initialize_identity(config);
        let pk = signing_key.verifying_key().to_bytes();

        // 2. Build authority registry
        let registry = registry::build_authority_registry();

        // 3. Issue and verify self certificate
        let self_cert = certificates::issue_self_certificate(pk, &registry);
        certificates::verify_self_certificate(&self_cert, &registry);

        // 4. Register self in engine
        engine.register_validator_identity(
            self_cert.validator_id.0,
            validator_id,
            pk,
            100,
        );
        engine.validator_id = validator_id;

        // 5. Load and verify peer certificates, register them
        let peers = certificates::load_and_verify_peer_certificates(config, &registry);
        for (cert_id, peer_id, peer_pk) in peers {
            engine.register_validator_identity(cert_id.0, peer_id, peer_pk, 100);
        }

        (signing_key, validator_id, registry)
    }
}
