use amun_constitution_builder::*;

#[test]
fn test_manifest_determinism() {
    let gen = "genesis_hash".to_string();
    let spec = "spec_hash".to_string();
    let ts = "2026-05-28T00:00:00Z".to_string();

    let m1 = ConstitutionalManifest::new(gen.clone(), spec.clone(), ts.clone());
    let m2 = ConstitutionalManifest::new(gen, spec, ts);

    VerificationEngine::verify_replay(&m1, &m2).expect("Manifests must be identical");
    assert_eq!(m1.canonical_bytes(), m2.canonical_bytes());
}

#[test]
fn test_federation_determinism() {
    let ts = "2026-05-28T00:00:00Z".to_string();
    let f1 = FederationArtifact::new(
        "AmunChain".into(), "OtherChain".into(),
        "gen_a".into(), "gen_b".into(),
        "spec_a".into(), "spec_b".into(),
        vec!["treaty-1".into()],
        ts.clone(),
    );
    let f2 = FederationArtifact::new(
        "AmunChain".into(), "OtherChain".into(),
        "gen_a".into(), "gen_b".into(),
        "spec_a".into(), "spec_b".into(),
        vec!["treaty-1".into()],
        ts,
    );

    VerificationEngine::verify_replay(&f1, &f2).expect("Federation artifacts must be identical");
    assert_eq!(f1.canonical_bytes(), f2.canonical_bytes());
}

#[test]
fn test_treaty_determinism() {
    let ts = "2026-05-28T00:00:00Z".to_string();
    let t1 = TreatyArtifact::new(
        "treaty-1".into(),
        vec!["AmunChain".into(), "OtherChain".into()],
        ts.clone(),
    );
    let t2 = TreatyArtifact::new(
        "treaty-1".into(),
        vec!["AmunChain".into(), "OtherChain".into()],
        ts,
    );

    VerificationEngine::verify_replay(&t1, &t2).expect("Treaties must be identical");
    assert_eq!(t1.canonical_bytes(), t2.canonical_bytes());
}
