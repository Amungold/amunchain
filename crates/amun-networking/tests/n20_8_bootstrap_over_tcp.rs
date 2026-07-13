use amun_certificate_network::distribution::LightClientProofBundle;
use amun_chain_checkpoint::{
    bootstrap::BootstrapSession,
    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
    CheckpointCertificate,
};
use amun_constitutional_block::ConstitutionalBlock;
use amun_constitutional_state::ConstitutionalStateRuntime;
use amun_networking::node::{NetworkNode, NodeLifecycle};
use amun_networking::sync_protocol::{SyncRequest, SyncResponse};

/// Build a checkpoint covering blocks [start, end].
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
// N20.8 — Bootstrap Request/Response
// ============================================================

#[test]
fn n20_8_bootstrap_request_roundtrip() {
    let request = SyncRequest { from_height: 0 };
    let json = serde_json::to_string(&request).unwrap();
    let decoded: SyncRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.from_height, 0);
}

#[test]
fn n20_8_bootstrap_response_roundtrip() {
    let cp = build_checkpoint(0, 9);
    let response = SyncResponse {
        latest_height: 10,
        checkpoints: vec![cp],
    };
    let json = serde_json::to_string(&response).unwrap();
    let decoded: SyncResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.latest_height, 10);
    assert_eq!(decoded.checkpoints.len(), 1);
}

// ============================================================
// N20.8 — Bootstrap over TCP (simulated)
// ============================================================

#[test]
fn n20_8_node_bootstraps_over_tcp() {
    // Node A: active, at height 50
    let cp = build_checkpoint(0, 49);
    let checkpoints = vec![cp.clone()];
    let trusted_root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp.clone(), proof);

    // Node B: fresh, bootstrapping
    let mut node_b = NetworkNode::new_bootstrapping([20u8; 32]);
    assert_eq!(node_b.lifecycle, NodeLifecycle::Bootstrapping);
    assert_eq!(node_b.current_height, 0);

    // Node B requests sync from Node A
    let _request = SyncRequest { from_height: 0 };

    // Node A responds with checkpoint
    let response = SyncResponse {
        latest_height: 50,
        checkpoints: vec![cp],
    };

    // Node B verifies against trusted root
    let mut session = BootstrapSession::new(trusted_root);
    assert!(session.ingest_bundles(&[bundle]).is_ok());

    // Node B imports the checkpoint height
    node_b.import_checkpoint_height(response.latest_height);
    assert_eq!(node_b.current_height, 50);

    // Node B transitions to active
    node_b.begin_catchup();
    node_b.begin_verification();
    node_b.activate();
    assert!(node_b.is_active());
}

#[test]
fn n20_8_wrong_trusted_root_rejected_over_tcp() {
    let cp = build_checkpoint(0, 9);
    let checkpoints = vec![cp.clone()];
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp, proof);

    // Attacker provides wrong trusted root
    let fake_root = [0xFF; 32];
    let mut session = BootstrapSession::new(fake_root);
    assert!(session.ingest_bundles(&[bundle]).is_err());
}

#[test]
fn n20_8_empty_bootstrap_rejected() {
    let response = SyncResponse {
        latest_height: 0,
        checkpoints: Vec::new(),
    };
    assert!(response.checkpoints.is_empty());

    // Empty checkpoint list must not allow bootstrap
    let root = [0u8; 32];
    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[]).is_err());
}

#[test]
fn n20_8_activation_after_successful_bootstrap() {
    let cp = build_checkpoint(0, 49);
    let checkpoints = vec![cp.clone()];
    let root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp, proof);

    let mut node = NetworkNode::new_bootstrapping([30u8; 32]);
    assert!(!node.is_active());

    // Verify and import
    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle]).is_ok());

    node.import_checkpoint_height(50);
    node.begin_catchup();
    node.begin_verification();
    node.activate();

    assert!(node.is_active());
    node.propose();
    assert!(!node.consensus.pending_actions.is_empty());
}
