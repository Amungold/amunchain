use amun_constitutional_signing::ConstitutionalKeyPair;
use amun_peer_identity::{ConstitutionalPeerId, IdentityVerifier, PeerCertificate, PeerRegistry};

const GENESIS: &str = "423ba28b1134ec6c2c0c87d67786fa13f792b36e7d2d561c1c6fff8e57f3a732";

#[test]
fn test_peer_id_determinism() {
    let a = ConstitutionalPeerId::new("key_a".into(), GENESIS.into());
    let b = ConstitutionalPeerId::new("key_a".into(), GENESIS.into());
    assert_eq!(a.peer_id, b.peer_id);
}

#[test]
fn test_different_genesis_produces_different_id() {
    let a = ConstitutionalPeerId::new("key".into(), GENESIS.into());
    let b = ConstitutionalPeerId::new("key".into(), "other_genesis".into());
    assert_ne!(a.peer_id, b.peer_id);
}

#[test]
fn test_self_signed_certificate_verification() {
    let keypair = ConstitutionalKeyPair::generate();
    let peer_id = ConstitutionalPeerId::new(keypair.verifying_key_hex(), GENESIS.into());
    let cert = PeerCertificate::self_sign(peer_id, &keypair);
    assert!(IdentityVerifier::verify(&cert, GENESIS).is_ok());
}

#[test]
fn test_certificate_rejected_for_wrong_civilisation() {
    let keypair = ConstitutionalKeyPair::generate();
    let peer_id = ConstitutionalPeerId::new(keypair.verifying_key_hex(), "other_genesis".into());
    let cert = PeerCertificate::self_sign(peer_id, &keypair);
    assert!(IdentityVerifier::verify(&cert, GENESIS).is_err());
}

#[test]
fn test_registry_determinism() {
    let keypair = ConstitutionalKeyPair::generate();
    let peer_id = ConstitutionalPeerId::new(keypair.verifying_key_hex(), GENESIS.into());
    let cert = PeerCertificate::self_sign(peer_id, &keypair);

    let mut reg = PeerRegistry::new();
    reg.register(cert);
    assert_eq!(reg.len(), 1);

    let serialized = serde_json::to_string(&reg).unwrap();
    let deserialized: PeerRegistry = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.len(), 1);
}
