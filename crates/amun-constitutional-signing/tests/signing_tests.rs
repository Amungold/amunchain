use amun_constitution_builder::{
    digest::ArtifactDigest, ConstitutionalManifest, FederationArtifact, TreatyArtifact,
};
use amun_constitutional_signing::{ConstitutionalKeyPair, SignedArtifact};

#[test]
fn test_sign_and_verify_manifest() {
    let manifest = ConstitutionalManifest::new(
        "genesis_hash".into(),
        "spec_hash".into(),
        "2026-05-28T00:00:00Z".into(),
    );
    let keypair = ConstitutionalKeyPair::generate();
    let signed = SignedArtifact::sign(manifest, &keypair);
    assert!(signed.verify().is_ok());
}

#[test]
fn test_sign_and_verify_federation() {
    let fed = FederationArtifact::new(
        "AmunChain".into(),
        "OtherChain".into(),
        "gen_a".into(),
        "gen_b".into(),
        "spec_a".into(),
        "spec_b".into(),
        vec!["treaty-1".into()],
        "2026-05-28T00:00:00Z".into(),
    );
    let keypair = ConstitutionalKeyPair::generate();
    let signed = SignedArtifact::sign(fed, &keypair);
    assert!(signed.verify().is_ok());
}

#[test]
fn test_sign_and_verify_treaty() {
    let treaty = TreatyArtifact::new(
        "treaty-1".into(),
        vec!["AmunChain".into(), "OtherChain".into()],
        "2026-05-28T00:00:00Z".into(),
    );
    let keypair = ConstitutionalKeyPair::generate();
    let signed = SignedArtifact::sign(treaty, &keypair);
    assert!(signed.verify().is_ok());
}

#[test]
fn test_digest_determinism_with_domain_separation() {
    let m1 = ConstitutionalManifest::new("gen".into(), "spec".into(), "ts".into());
    let m2 = ConstitutionalManifest::new("gen".into(), "spec".into(), "ts".into());
    assert_eq!(m1.constitutional_digest(), m2.constitutional_digest());

    let t = TreatyArtifact::new("treaty-x".into(), vec!["AmunChain".into()], "ts".into());
    // Different domain separators should yield different digests.
    assert_ne!(m1.constitutional_digest(), t.constitutional_digest());
}

#[test]
fn test_signature_public_verifiability() {
    let manifest = ConstitutionalManifest::new("gen".into(), "spec".into(), "ts".into());
    let keypair = ConstitutionalKeyPair::generate();
    let signed = SignedArtifact::sign(manifest, &keypair);

    // Verification uses only public information.
    assert!(signed.verify().is_ok());

    // Tampering with the artifact must invalidate the signature.
    let mut tampered = signed.artifact.clone();
    tampered.genesis_hash = "tampered".into();
    let tampered_signed = SignedArtifact {
        artifact: tampered,
        signature: signed.signature.clone(),
    };
    assert!(tampered_signed.verify().is_err());
}
