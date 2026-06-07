use amun_chain_checkpoint::{
    CheckpointCertificate,
    inclusion::{
        checkpoint_merkle_root,
        prove_checkpoint_inclusion,
        CheckpointBundle,
    },
    bootstrap::BootstrapSession,
};
use amun_constitutional_state::ConstitutionalStateRuntime;
use amun_constitutional_block::ConstitutionalBlock;
use amun_certificate_network::distribution::LightClientProofBundle;

fn build_checkpoint_bundle(
    start: u64,
    end: u64,
) -> (CheckpointCertificate, CheckpointBundle, [u8; 32]) {
    let mut rt = ConstitutionalStateRuntime::new();
    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
    let parent = "0".repeat(64);

    for height in start..=end {
        rt.apply_transition(&[height as u8; 32], &[0xCC; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(
            ConstitutionalStateRuntime::certificate_merkle_root(&certs)
        );
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(
            &certs, &hash
        ).unwrap();

        let parent_hash = if height == start {
            &parent
        } else {
            &bundles.last().unwrap().block_header.block_hash
        };

        let block = ConstitutionalBlock::new(
            height,
            parent_hash.into(),
            "t".into(),
            "p".into(),
            vec![],
            hex::encode(rt.state_root()),
            "g".into(),
            "e".into(),
            "ev".into(),
            merkle_root,
        );

        bundles.push(LightClientProofBundle::new(block, cert, proof));
    }

    let cp = CheckpointCertificate::create(start, end, &bundles).unwrap();
    let checkpoints = vec![cp.clone()];
    let root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(
        &checkpoints, &cp.checkpoint_hash_bytes()
    ).unwrap();
    let bundle = CheckpointBundle::new(cp.clone(), proof);

    (cp, bundle, root)
}

fn build_bundle(
    start: u64,
    end: u64,
) -> (CheckpointBundle, [u8; 32]) {
    let (_, bundle, root) = build_checkpoint_bundle(start, end);
    (bundle, root)
}

#[test]
fn n16_forged_checkpoint_rejected() {
    let (cp1, _, root) = build_checkpoint_bundle(0, 2);
    let (bundle2, _) = build_bundle(3, 5);

    let checkpoints = vec![cp1.clone()];
    let proof = prove_checkpoint_inclusion(
        &checkpoints, &cp1.checkpoint_hash_bytes()
    ).unwrap();
    let bundle = CheckpointBundle::new(cp1, proof);

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle, bundle2]).is_err());
}

#[test]
fn n16_tampered_bundle_rejected() {
    let (_, mut bundle, root) = build_checkpoint_bundle(0, 4);
    bundle.inclusion_proof.checkpoint_hash = [0xFF; 32];

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle]).is_err());
}

#[test]
fn n16_chain_gap_detected() {
    let (_, bundle1, root) = build_checkpoint_bundle(0, 2);
    let (bundle2, _) = build_bundle(4, 6);

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle1, bundle2]).is_err());
}

#[test]
fn n16_wrong_trusted_root_rejected() {
    let (bundle, _) = build_bundle(0, 4);

    let mut session = BootstrapSession::new([0x77; 32]);
    assert!(session.ingest_bundles(&[bundle]).is_err());
}

#[test]
fn n16_byzantine_source_mixed_valid_invalid() {
    let (_, bundle1, root) = build_checkpoint_bundle(0, 2);

    let (_, mut bundle2, _) = build_checkpoint_bundle(3, 5);
    bundle2.inclusion_proof.root = [0x99; 32];

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle1, bundle2]).is_err());
}

#[test]
fn n16_empty_bundle_list_rejected() {
    let root = [0xAB; 32];
    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[]).is_err());
}

#[test]
fn n16_duplicate_checkpoint_accepted() {
    let (cp, bundle, root) = build_checkpoint_bundle(0, 4);

    let checkpoints = vec![cp.clone()];
    let proof = prove_checkpoint_inclusion(
        &checkpoints, &cp.checkpoint_hash_bytes()
    ).unwrap();
    let bundle_copy = CheckpointBundle::new(cp, proof);

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle, bundle_copy]).is_ok());
}
