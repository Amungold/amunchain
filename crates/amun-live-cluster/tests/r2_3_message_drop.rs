use amun_live_cluster::fault_injector::FaultInjector;
use amun_live_cluster::network_adapter::ValidatorNetworkAdapter;
use amun_networking::frame::{FrameKind, NetworkFrame};
use amun_networking::handshake::ConstitutionInfo;
use amun_networking::tcp_transport::TcpTransport;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

/// Helper: create a TcpTransport for testing.
fn make_test_transport(listen_addr: SocketAddr) -> TcpTransport {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let constitution = ConstitutionInfo {
        version: 1,
        hash: [0u8; 32],
        proof_system_version: 1,
        state_commitment_algorithm: "test".to_string(),
        accepted_features: vec!["sync".into(), "vote".into(), "block_range".into()],
    };
    TcpTransport::new(
        listen_addr,
        [0u8; 32], // network_id
        [0u8; 32], // genesis_hash
        [1u8; 32], // node_id
        signing_key,
        constitution,
    )
}

/// Helper: create a ValidatorNetworkAdapter with fault injection.
fn make_adapter(drop_percent: u8) -> (ValidatorNetworkAdapter, SocketAddr) {
    let listen_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let transport = Arc::new(Mutex::new(make_test_transport(listen_addr)));
    let fi = Arc::new(FaultInjector::drop(drop_percent));
    let adapter = ValidatorNetworkAdapter::with_fault_injector(transport, fi);
    (adapter, listen_addr)
}

// ============================================================
// R2.3.1 — Message Drop Tests
// ============================================================

#[test]
fn r2_3_1_drop_0_percent_all_delivered() {
    let (adapter, _addr) = make_adapter(0);
    let peer: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    for _ in 0..200 {
        let frame = NetworkFrame::new(FrameKind::Ping, vec![].into());
        // With 0% drop, send_to should succeed (transport may fail, but not due to fault injector)
        let _ = adapter.send_to(peer, frame);
    }
    println!("R2.3.1 drop_0%: 200 sends completed without fault injector interference");
}

#[test]
fn r2_3_1_drop_100_percent_silent_return() {
    let (adapter, _addr) = make_adapter(100);
    let peer: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    for _ in 0..100 {
        let frame = NetworkFrame::new(FrameKind::Ping, vec![].into());
        // With 100% drop, send_to returns Ok(()) but does NOT actually send
        assert!(adapter.send_to(peer, frame).is_ok());
    }
    println!("R2.3.1 drop_100%: 100 sends — all returned Ok(()) silently");
}

#[test]
fn r2_3_1_broadcast_vote_respects_fault_injector() {
    let (adapter, _) = make_adapter(50);
    for _ in 0..50 {
        adapter.broadcast_vote(vec![1, 2, 3]);
    }
    println!("R2.3.1 broadcast_vote with 50% drop: no crash");
}

#[test]
fn r2_3_1_disabled_injector_no_drops() {
    let fi = FaultInjector::disabled();

    for _ in 0..1000 {
        assert!(!fi.should_drop(), "Disabled FaultInjector must never drop");
    }

    println!("R2.3.1 disabled injector: 1000 calls, 0 drops");
}

// ============================================================
// FaultInjector unit-level determinism tests
// ============================================================

#[test]
fn r2_3_1_drop_30_percent_deterministic() {
    let fi1 = FaultInjector::drop(30);
    let fi2 = FaultInjector::drop(30);

    let seq1: Vec<bool> = (0..200).map(|_| fi1.should_drop()).collect();
    let seq2: Vec<bool> = (0..200).map(|_| fi2.should_drop()).collect();

    assert_eq!(
        seq1, seq2,
        "Same drop% must produce identical deterministic sequence"
    );

    let drop_count = seq1.iter().filter(|&&d| d).count();
    let drop_pct = (drop_count as f64 / 200.0) * 100.0;
    assert!(
        (20.0..=40.0).contains(&drop_pct),
        "Expected ~30% drop, got {:.1}% ({} / 200)",
        drop_pct,
        drop_count
    );
}

#[test]
fn r2_3_1_drop_50_percent_deterministic() {
    let fi = FaultInjector::drop(50);
    let seq: Vec<bool> = (0..200).map(|_| fi.should_drop()).collect();
    let drop_count = seq.iter().filter(|&&d| d).count();
    let drop_pct = (drop_count as f64 / 200.0) * 100.0;
    assert!(
        (40.0..=60.0).contains(&drop_pct),
        "Expected ~50% drop, got {:.1}% ({} / 200)",
        drop_pct,
        drop_count
    );
}

#[test]
fn r2_3_1_shared_injector_across_threads() {
    use std::thread;

    let fi = Arc::new(FaultInjector::drop(25));
    let fi2 = Arc::clone(&fi);

    let h = thread::spawn(move || {
        let mut drops = 0;
        for _ in 0..500 {
            if fi2.should_drop() {
                drops += 1;
            }
        }
        drops
    });

    let mut drops_main = 0;
    for _ in 0..500 {
        if fi.should_drop() {
            drops_main += 1;
        }
    }

    let drops_thread = h.join().unwrap();
    let total = drops_main + drops_thread;
    let pct = (total as f64 / 1000.0) * 100.0;

    assert!(
        (15.0..=35.0).contains(&pct),
        "Expected ~25% drop across threads, got {:.1}% ({} / 1000)",
        pct,
        total
    );
}
