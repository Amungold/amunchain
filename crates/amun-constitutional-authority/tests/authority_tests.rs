use amun_constitution_builder::digest::ArtifactDigest;
use amun_constitutional_signing::{ConstitutionalKeyPair, SignedArtifact};
use amun_constitutional_authority::{
    ConstitutionalCertificate, CertificateChain, RevocationRegistry,
    KeyRotationLaw, TrustAnchor,
};

fn build_root(key: &ConstitutionalKeyPair) -> ConstitutionalCertificate {
    ConstitutionalCertificate::new_root(
        key.verifying_key_hex(),
        "2026-01-01T00:00:00Z".into(),
        "2036-01-01T00:00:00Z".into(),
        "ConstitutionalRoot".into(),
        "2026-05-28T00:00:00Z".into(),
    )
}

fn build_child(parent: &ConstitutionalCertificate, key: &ConstitutionalKeyPair) -> ConstitutionalCertificate {
    ConstitutionalCertificate::new_child(
        parent.certificate_id.clone(),
        key.verifying_key_hex(),
        parent.certificate_id.clone(),
        "2026-06-01T00:00:00Z".into(),
        "2035-12-31T23:59:59Z".into(),
        "OperationalScope".into(),
        "2026-05-28T12:00:00Z".into(),
    )
}

#[test]
fn stable_certificate_id() {
    let key = ConstitutionalKeyPair::generate();
    let a = build_root(&key);
    let b = build_root(&key);
    assert_eq!(a.certificate_id, b.certificate_id);
    assert_eq!(a.certificate_id, a.digest_hex());
}

#[test]
fn valid_chain_passes_validation() {
    let root_key = ConstitutionalKeyPair::generate();
    let child_key = ConstitutionalKeyPair::generate();
    let root = build_root(&root_key);
    let child = build_child(&root, &child_key);

    let mut chain = CertificateChain::new(SignedArtifact::sign(root, &root_key));
    chain.append(SignedArtifact::sign(child, &root_key)).unwrap();
    assert!(chain.validate(&RevocationRegistry::new()).is_ok());
}

#[test]
fn wrong_issuer_key_rejected() {
    let root_key = ConstitutionalKeyPair::generate();
    let child_key = ConstitutionalKeyPair::generate();
    let fake = ConstitutionalKeyPair::generate();
    let root = build_root(&root_key);
    let child = build_child(&root, &child_key);

    let mut chain = CertificateChain::new(SignedArtifact::sign(root, &root_key));
    chain.append(SignedArtifact::sign(child, &fake)).unwrap();
    assert!(chain.validate(&RevocationRegistry::new()).is_err());
}

#[test]
fn broken_lineage_rejected() {
    let root_key = ConstitutionalKeyPair::generate();
    let child_key = ConstitutionalKeyPair::generate();
    let root = build_root(&root_key);
    let mut child = build_child(&root, &child_key);
    child.lineage_parent_hash = Some("deadbeef".into());

    let mut chain = CertificateChain::new(SignedArtifact::sign(root, &root_key));
    assert!(chain.append(SignedArtifact::sign(child, &root_key)).is_err());
}

#[test]
fn revoked_certificate_rejected() {
    let root_key = ConstitutionalKeyPair::generate();
    let child_key = ConstitutionalKeyPair::generate();
    let root = build_root(&root_key);
    let child = build_child(&root, &child_key);

    let signed_child = SignedArtifact::sign(child, &root_key);
    let mut chain = CertificateChain::new(SignedArtifact::sign(root, &root_key));
    chain.append(signed_child.clone()).unwrap();

    let mut reg = RevocationRegistry::new();
    reg.revoke(signed_child.artifact.certificate_id.clone());
    assert!(chain.validate(&reg).is_err());
}

#[test]
fn rotation_with_proof_accepted() {
    let old = ConstitutionalKeyPair::generate();
    let new = ConstitutionalKeyPair::generate();
    let cert = ConstitutionalCertificate::new_child(
        "parent".into(), new.verifying_key_hex(), "parent".into(),
        "2026-07-01T00:00:00Z".into(), "2035-12-31T23:59:59Z".into(),
        "RotatedScope".into(), "2026-05-28T13:00:00Z".into(),
    );
    let signed = SignedArtifact::sign(cert, &old);
    assert!(KeyRotationLaw::validate_rotation(&old.verifying_key_hex(), &signed, &old).is_ok());
}

#[test]
fn trust_anchor_is_self_signed() {
    let key = ConstitutionalKeyPair::generate();
    let root = build_root(&key);
    let signed = SignedArtifact::sign(root, &key);
    let anchor = TrustAnchor::new(signed, "genesis".into(), "root-scope".into());
    assert!(anchor.is_self_signed());
    assert!(anchor.verify().is_ok());
}

#[test]
fn revocation_registry_is_deterministic() {
    let mut reg = RevocationRegistry::new();
    reg.revoke("b".into());
    reg.revoke("a".into());
    let json = serde_json::to_string(&reg).unwrap();
    assert!(json.contains("\"a\",\"b\""));
}
