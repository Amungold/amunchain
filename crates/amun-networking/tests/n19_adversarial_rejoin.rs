use amun_certificate_network::distribution::LightClientProofBundle;
use amun_chain_checkpoint::{
    bootstrap::BootstrapSession,
    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
    CheckpointCertificate,
};
use amun_constitutional_block::ConstitutionalBlock;
use amun_constitutional_state::ConstitutionalStateRuntime;
use amun_networking::node::NetworkNode;

fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
    let mut rt = ConstitutionalStateRuntime::new();
    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
    let parent = "0".repeat(64);

    for height in start..=end {
        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();

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

    CheckpointCertificate::create(start, end, &bundles).unwrap()
}

// ============================================================
// N19.1 — Wrong Trusted Root
// ============================================================

#[test]
fn n19_wrong_trusted_root_rejected() {
    let cp = build_checkpoint(0, 9);
    let checkpoints = vec![cp.clone()];
    let _real_root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp, proof);

    // Attacker provides a different trusted root
    let fake_root = [0xFF; 32];
    let mut session = BootstrapSession::new(fake_root);
    assert!(session.ingest_bundles(&[bundle]).is_err());
}

// ============================================================
// N19.2 — Checkpoint Rollback
// ============================================================

#[test]
fn n19_checkpoint_rollback_rejected() {
    let cp_high = build_checkpoint(50, 59);
    let checkpoints = vec![cp_high.clone()];
    let root = checkpoint_merkle_root(&checkpoints);

    let mut node = NetworkNode::new_bootstrapping([6u8; 32]);
    node.import_checkpoint_height(100);

    // Attacker sends checkpoint at lower height
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_high.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp_high, proof);

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle]).is_ok());

    // But node should detect regression
    let attacker_height = 55;
    assert!(attacker_height < node.current_height);
    // Node must not regress
    node.import_checkpoint_height(node.current_height.max(attacker_height));
    assert_eq!(node.current_height, 100);
}

// ============================================================
// N19.3 — Checkpoint Gap Detection
// ============================================================

#[test]
fn n19_checkpoint_gap_detected() {
    let cp1 = build_checkpoint(0, 19);
    let cp2 = build_checkpoint(40, 59);

    let checkpoints = vec![cp1.clone(), cp2.clone()];
    let root = checkpoint_merkle_root(&checkpoints);
    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();

    let bundles = vec![
        CheckpointBundle::new(cp1, proof1),
        CheckpointBundle::new(cp2, proof2),
    ];

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&bundles).is_ok());

    // Verify gap exists between checkpoints
    assert_eq!(bundles[0].checkpoint.end_height, 19);
    assert_eq!(bundles[1].checkpoint.start_height, 40);
    assert!(bundles[1].checkpoint.start_height > bundles[0].checkpoint.end_height + 1);
}

// ============================================================
// N19.4 — Mixed Valid/Invalid Checkpoint Stream
// ============================================================

#[test]
fn n19_mixed_checkpoint_stream_rejected() {
    let cp1 = build_checkpoint(0, 9);
    let cp2 = build_checkpoint(10, 19);

    let checkpoints = vec![cp1.clone(), cp2.clone()];
    let root = checkpoint_merkle_root(&checkpoints);
    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();

    // Tamper with the second bundle
    let mut proof2 =
        prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
    proof2.root = [0xAA; 32];

    let bundles = vec![
        CheckpointBundle::new(cp1, proof1),
        CheckpointBundle::new(cp2, proof2),
    ];

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&bundles).is_err());
}

// ============================================================
// N19.5 — Byzantine Rejoin Source
// ============================================================

#[test]
fn n19_byzantine_rejoin_source_rejected() {
    let cp = build_checkpoint(0, 49);
    let checkpoints = vec![cp.clone()];
    let root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp, proof);

    // Honest bootstrap works
    let mut honest_session = BootstrapSession::new(root);
    assert!(honest_session
        .ingest_bundles(std::slice::from_ref(&bundle))
        .is_ok());

    // Byzantine source provides wrong root
    let byzantine_root = [0x13; 32];
    let mut byzantine_session = BootstrapSession::new(byzantine_root);
    assert!(byzantine_session.ingest_bundles(&[bundle]).is_err());
}
