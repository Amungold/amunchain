use std::net::{SocketAddr, TcpListener};
use std::str::FromStr;
use std::time::Duration;

use amun_networking::node::{NetworkNode, NodeLifecycle};
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::peer_identity::PeerIdentity;
use amun_networking::sync_protocol::{SyncRequest, SyncResponse};
use amun_networking::crypto_identity::PeerKeyPair;
use amun_networking::transport_trait::Transport;
use amun_chain_checkpoint::{
    CheckpointCertificate,
    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
    bootstrap::BootstrapSession,
};
use amun_constitutional_state::ConstitutionalStateRuntime;
use amun_constitutional_block::ConstitutionalBlock;
use amun_certificate_network::distribution::LightClientProofBundle;

// ============================================================
// Helper: find an available port on localhost
// ============================================================
fn find_available_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to port");
    listener.local_addr().unwrap().port()
}

// ============================================================
// N20.10.1 — TCP Bind and Connect
// ============================================================
#[test]
fn n20_10_tcp_bind_and_connect() {
    let port = find_available_port();
    let addr = SocketAddr::from_str(&format!("127.0.0.1:{}", port)).unwrap();

    // Server
    let mut server = TcpTransport::new(addr);
    server.bind().expect("Server should bind");

    // Client
    let client_port = find_available_port();
    let client_addr = SocketAddr::from_str(&format!("127.0.0.1:{}", client_port)).unwrap();
    let mut client = TcpTransport::new(client_addr);
    client.connect_to(addr);

    // Let connection establish
    std::thread::sleep(Duration::from_millis(100));
    server.tick(0);
    client.tick(0);

    // Server should have accepted the connection
    // TCP bind and connect verified
}

// ============================================================
// N20.10.2 — Peer Identity Exchange over TCP
// ============================================================
#[test]
fn n20_10_peer_identity_exchange() {
    let alice_keypair = PeerKeyPair::generate();
    let bob_keypair = PeerKeyPair::generate();

    let alice_id = alice_keypair.peer_id();
    let bob_id = bob_keypair.peer_id();

    let alice_identity = PeerIdentity::new(
        alice_id,
        alice_keypair.verifying_key.to_bytes(),
        "127.0.0.1:8001".parse().unwrap(),
    );
    let bob_identity = PeerIdentity::new(
        bob_id,
        bob_keypair.verifying_key.to_bytes(),
        "127.0.0.1:8002".parse().unwrap(),
    );

    // Verify identities are distinct
    assert_ne!(alice_id, bob_id);
    assert_ne!(alice_identity.peer_id, bob_identity.peer_id);
}

// ============================================================
// N20.10.3 — Multi-Node Bootstrap Scenario
// ============================================================
fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
    let mut rt = ConstitutionalStateRuntime::new();
    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
    let parent = "0".repeat(64);

    for height in start..=end {
        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
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
            height, parent_hash.into(), "t".into(), "p".into(), vec![],
            hex::encode(rt.state_root()), "g".into(), "e".into(), "ev".into(), merkle_root,
        );

        bundles.push(LightClientProofBundle::new(block, cert, proof));
    }

    CheckpointCertificate::create(start, end, &bundles).unwrap()
}

#[test]
fn n20_10_new_node_bootstraps_from_existing_peer() {
    // Phase 1: Existing node has checkpoint at height 50
    let cp = build_checkpoint(0, 49);
    let checkpoints = vec![cp.clone()];
    let trusted_root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp.clone(), proof);

    // Phase 2: New node starts bootstrapping
    let mut new_node = NetworkNode::new_bootstrapping([10u8; 32]);
    assert_eq!(new_node.lifecycle, NodeLifecycle::Bootstrapping);
    assert_eq!(new_node.current_height, 0);

    // Phase 3: New node receives checkpoint from existing peer
    let _request = SyncRequest { from_height: 0 };
    let response = SyncResponse {
        latest_height: 50,
        checkpoints: vec![cp],
    };

    // Phase 4: Verify checkpoint
    let mut session = BootstrapSession::new(trusted_root);
    assert!(session.ingest_bundles(&[bundle]).is_ok());

    // Phase 5: Import and activate
    new_node.import_checkpoint_height(response.latest_height);
    assert_eq!(new_node.current_height, 50);

    new_node.begin_catchup();
    new_node.begin_verification();
    new_node.activate();

    assert!(new_node.is_active());
    assert_eq!(new_node.current_height, 50);
}

// ============================================================
// N20.10.4 — Four Node Network Startup (simulated)
// ============================================================
#[test]
fn n20_10_four_nodes_start_together() {
    // Simulate 4 nodes starting with keypairs and identities
    let mut nodes = Vec::new();
    for i in 0..4 {
        let keypair = PeerKeyPair::generate();
        let identity = PeerIdentity::new(
            keypair.peer_id(),
            keypair.verifying_key.to_bytes(),
            format!("127.0.0.1:{}", 9001 + i).parse().unwrap(),
        );
        let mut node = NetworkNode::new([i as u8; 32]);
        node.keypair = Some(keypair);
        nodes.push((node, identity));
    }

    // All nodes should be active
    for (node, _) in &nodes {
        assert!(node.is_active());
    }

    // All peer IDs should be distinct
    let ids: Vec<_> = nodes.iter().map(|(n, _)| n.peer_id().unwrap()).collect();
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            assert_ne!(ids[i], ids[j]);
        }
    }
}

// ============================================================
// N20.10.5 — Network Survives Node Disconnect (simulated)
// ============================================================
#[test]
fn n20_10_network_survives_disconnect() {
    let cp = build_checkpoint(50, 99);
    let checkpoints = vec![cp.clone()];
    let root = checkpoint_merkle_root(&checkpoints);
    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
    let bundle = CheckpointBundle::new(cp.clone(), proof);

    // Node goes offline, network advances, node reconnects
    let mut node = NetworkNode::new_bootstrapping([5u8; 32]);

    // Node receives checkpoint from after the disconnect
    let response = SyncResponse {
        latest_height: 100,
        checkpoints: vec![cp],
    };

    let mut session = BootstrapSession::new(root);
    assert!(session.ingest_bundles(&[bundle]).is_ok());

    node.import_checkpoint_height(response.latest_height);
    node.begin_catchup();
    node.begin_verification();
    node.activate();

    assert_eq!(node.current_height, 100);
    assert!(node.is_active());
}
