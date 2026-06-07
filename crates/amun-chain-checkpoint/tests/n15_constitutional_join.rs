use amun_chain_checkpoint::{
    CheckpointCertificate, CheckpointStore,
    inclusion::{
        checkpoint_merkle_root,
        prove_checkpoint_inclusion,
        CheckpointBundle,
    },
    chain::RecursiveCheckpointProof,
    bootstrap::BootstrapSession,
};
use amun_constitutional_state::ConstitutionalStateRuntime;
use amun_constitutional_block::ConstitutionalBlock;
use amun_certificate_network::distribution::LightClientProofBundle;

#[test]
fn n15_node_b_full_bootstrap_from_node_a_checkpoints() {
    let mut rt_a = ConstitutionalStateRuntime::new();
    let mut bundles_a: Vec<LightClientProofBundle> = Vec::new();
    let parent = "0".repeat(64);

    for height in 0..10 {
        rt_a.apply_transition(&[height as u8; 32], &[0xAA; 32]);
        let cert = rt_a.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(
            ConstitutionalStateRuntime::certificate_merkle_root(&certs)
        );
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(
            &certs, &hash
        ).unwrap();

        let parent_hash = if height == 0 {
            &parent
        } else {
            &bundles_a.last().unwrap().block_header.block_hash
        };

        let block = ConstitutionalBlock::new(
            height,
            parent_hash.into(),
            "t".into(),
            "p".into(),
            vec![],
            hex::encode(rt_a.state_root()),
            "g".into(),
            "e".into(),
            "ev".into(),
            merkle_root,
        );

        bundles_a.push(LightClientProofBundle::new(block, cert, proof));
    }

    let final_state_root = hex::encode(rt_a.state_root());

    let cp1 = CheckpointCertificate::create(0, 4, &bundles_a[0..5]).unwrap();
    let cp2 = CheckpointCertificate::create(5, 9, &bundles_a[5..10]).unwrap();

    let checkpoints = vec![cp1.clone(), cp2.clone()];
    let checkpoint_root = checkpoint_merkle_root(&checkpoints);

    let proof1 = prove_checkpoint_inclusion(
        &checkpoints, &cp1.checkpoint_hash_bytes()
    ).unwrap();
    let proof2 = prove_checkpoint_inclusion(
        &checkpoints, &cp2.checkpoint_hash_bytes()
    ).unwrap();

    let checkpoint_bundles = vec![
        CheckpointBundle::new(cp1, proof1),
        CheckpointBundle::new(cp2, proof2),
    ];

    let mut node_b = BootstrapSession::new(checkpoint_root);

    assert!(node_b.ingest_bundles(&checkpoint_bundles).is_ok());

    assert_eq!(node_b.trusted_root(), checkpoint_root);

    let mut rt_b = ConstitutionalStateRuntime::new();
    for height in 0..10 {
        rt_b.apply_transition(&[height as u8; 32], &[0xAA; 32]);
    }
    let expected_root = hex::encode(rt_b.state_root());
    assert_eq!(expected_root, final_state_root);

    let recursive = RecursiveCheckpointProof::from_checkpoints(&checkpoints).unwrap();
    assert!(recursive.verify().is_ok());

    let mut store = CheckpointStore::new();
    store.store(checkpoints[0].clone());
    store.store(checkpoints[1].clone());
    assert_eq!(store.count(), 2);
    assert!(store.get(4).is_some());
    assert!(store.get(9).is_some());
    assert_eq!(store.latest().unwrap().end_height, 9);
}

#[test]
fn n15_bootstrap_preserves_chain_continuity() {
    let mut rt = ConstitutionalStateRuntime::new();
    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
    let parent = "0".repeat(64);

    for height in 0..5 {
        rt.apply_transition(&[height as u8; 32], &[0xBB; 32]);
        let cert = rt.create_certificate(height, [0u8; 32]);
        let certs = vec![cert.clone()];
        let merkle_root = hex::encode(
            ConstitutionalStateRuntime::certificate_merkle_root(&certs)
        );
        let hash = cert.certificate_hash();
        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(
            &certs, &hash
        ).unwrap();

        let parent_hash = if height == 0 {
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

    let cp = CheckpointCertificate::create(0, 4, &bundles).unwrap();
    let checkpoints = vec![cp.clone()];
    let root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(
        &checkpoints, &cp.checkpoint_hash_bytes()
    ).unwrap();
    let bundle = CheckpointBundle::new(cp, proof);

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle]).is_ok());

    let mut rt_check = ConstitutionalStateRuntime::new();
    for height in 0..5 {
        rt_check.apply_transition(&[height as u8; 32], &[0xBB; 32]);
    }

    let mut rt_verify = ConstitutionalStateRuntime::new();
    for height in 0..5 {
        rt_verify.apply_transition(&[height as u8; 32], &[0xBB; 32]);
    }
    assert_eq!(rt_check.state_root(), rt_verify.state_root());
}
