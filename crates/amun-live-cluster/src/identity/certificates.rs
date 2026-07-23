use crate::config::ValidatorConfig;
use amun_authority_registry::AuthorityRegistry;
use amun_networking::peer_identity::PeerId;
use amun_networking::validator_certificate::ValidatorCertificate;

/// Issue a self-signed certificate using the genesis authority.
pub fn issue_self_certificate(pk: [u8; 32], registry: &AuthorityRegistry) -> ValidatorCertificate {
    let active_authority = registry.active().expect("No active authority");
    let my_peer_id = PeerId::from_bytes(pk);
    let genesis_authority_kp = amun_networking::crypto_identity::PeerKeyPair::from_seed([0x42; 32]);
    ValidatorCertificate::issue_v2(
        my_peer_id,
        pk,
        active_authority.authority_version,
        active_authority.authority_id,
        &genesis_authority_kp,
        0,
        0,
    )
}

/// Verify a self-signed certificate against the registry at height 0.
pub fn verify_self_certificate(cert: &ValidatorCertificate, registry: &AuthorityRegistry) {
    if !registry.verify_certificate_at(cert, 0) {
        panic!("Self certificate verification failed");
    }
}

/// Load a peer certificate from disk.
pub fn load_peer_certificate(path: &str) -> ValidatorCertificate {
    let cert_json = std::fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read certificate {}", path));
    serde_json::from_str(&cert_json)
        .unwrap_or_else(|_| panic!("Invalid certificate JSON in {}", path))
}

/// Verify a peer certificate against the registry at height 0.
pub fn verify_peer_certificate(
    cert: &ValidatorCertificate,
    registry: &AuthorityRegistry,
    path: &str,
) {
    if !registry.verify_certificate_at(cert, 0) {
        panic!("Peer certificate verification failed for {}", path);
    }
}

/// Load all peer certificates from config and verify them.
pub fn load_and_verify_peer_certificates(
    config: &ValidatorConfig,
    registry: &AuthorityRegistry,
) -> Vec<(amun_networking::peer_identity::PeerId, [u8; 32], [u8; 32])> {
    let mut result = Vec::new();
    for peer in config.other_peers() {
        let cert_path = peer
            .certificate_path
            .as_ref()
            .expect("Peer certificate_path not set");
        let peer_cert = load_peer_certificate(cert_path);
        verify_peer_certificate(&peer_cert, registry, cert_path);
        let peer_pk = peer_cert.public_key;
        let peer_id = amun_validator_identity::derive_validator_id(&peer_pk);
        result.push((peer_cert.validator_id, peer_id, peer_pk));
    }
    result
}
