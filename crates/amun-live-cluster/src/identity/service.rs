use crate::config::ValidatorConfig;
use amun_authority_registry::AuthorityRegistry;
use amun_consensus_network::engine::ConsensusEngine;
use ed25519_dalek::SigningKey;

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

        // Save self-certificate so peers can discover it
        let cert_path = std::path::PathBuf::from(&config.data_dir).join("certificate.json");
        if let Ok(json) = serde_json::to_string(&self_cert) {
            let _ = std::fs::write(&cert_path, json);
            eprintln!("[PHASE3] Saved cert to {}", cert_path.display());
        }

        // 4. Register self in engine
        engine.register_validator_identity(self_cert.validator_id.0, validator_id, pk, 100);
        engine.validator_id = validator_id;
        eprintln!(
            "[PHASE3] SELF: cert.validator_id={:?} derived_id={:?} pk_first4={:?}",
            &self_cert.validator_id.0[..4],
            &validator_id[..4],
            &pk[..4]
        );

        // 5. Load and verify peer certificates, register them
        let peer_count = config.other_peers().len();
        eprintln!("[PHASE3] other_peers count = {}", peer_count);
        let peers = certificates::load_and_verify_peer_certificates(config, &registry);
        eprintln!("[PHASE3] loaded peers count = {}", peers.len());
        for (cert_id, peer_id, peer_pk) in peers {
            eprintln!(
                "[PHASE3] PEER: cert.validator_id={:?} derived_id={:?} pk_first4={:?}",
                &cert_id.0[..4],
                &peer_id[..4],
                &peer_pk[..4]
            );
            // PHASE3 FIX: Register with peer_id (derived) which matches vote.voter_id
            engine.register_validator_identity(peer_id, peer_id, peer_pk, 100);
        }

        (signing_key, validator_id, registry)
    }
}
