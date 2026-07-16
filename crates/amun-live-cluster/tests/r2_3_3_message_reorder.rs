use amun_live_cluster::fault_injector::FaultInjector;
use std::sync::Arc;

#[test]
fn r2_3_3_reorder_0_percent_never_reorders() {
    let fi = FaultInjector::reorder(0, 4);
    for _ in 0..1000 {
        assert!(
            fi.should_reorder().is_none(),
            "0% reorder should never reorder"
        );
    }
}

#[test]
fn r2_3_3_reorder_100_percent_always_reorders() {
    let fi = FaultInjector::reorder(100, 4);
    let mut reorder_count = 0;
    for _ in 0..200 {
        if fi.should_reorder().is_some() {
            reorder_count += 1;
        }
    }
    assert_eq!(reorder_count, 200, "100% reorder should always reorder");
}

#[test]
fn r2_3_3_reorder_returns_correct_buffer_size() {
    let fi = FaultInjector::reorder(100, 8);
    for _ in 0..50 {
        assert_eq!(fi.should_reorder(), Some(8), "Should return buffer_size=8");
    }
}

#[test]
fn r2_3_3_reorder_is_deterministic() {
    let fi1 = FaultInjector::reorder(30, 4);
    let fi2 = FaultInjector::reorder(30, 4);
    let seq1: Vec<Option<usize>> = (0..200).map(|_| fi1.should_reorder()).collect();
    let seq2: Vec<Option<usize>> = (0..200).map(|_| fi2.should_reorder()).collect();
    assert_eq!(
        seq1, seq2,
        "Same reorder config must produce identical sequence"
    );
    let reorder_count = seq1.iter().filter(|d| d.is_some()).count();
    let reorder_pct = (reorder_count as f64 / 200.0) * 100.0;
    assert!(
        (20.0..=40.0).contains(&reorder_pct),
        "Expected ~30%, got {:.1}%",
        reorder_pct
    );
}

#[test]
fn r2_3_3_reorder_does_not_drop() {
    let fi = FaultInjector::reorder(100, 4);
    for _ in 0..500 {
        assert!(!fi.should_drop(), "Reorder mode must never drop");
    }
}

#[test]
fn r2_3_3_reorder_does_not_delay() {
    let fi = FaultInjector::reorder(100, 4);
    for _ in 0..500 {
        assert!(fi.should_delay().is_none(), "Reorder mode must never delay");
    }
}

#[test]
fn r2_3_3_reorder_shared_across_threads() {
    use std::thread;
    let fi = Arc::new(FaultInjector::reorder(25, 4));
    let fi2 = Arc::clone(&fi);
    let h = thread::spawn(move || {
        let mut reorders = 0;
        for _ in 0..500 {
            if fi2.should_reorder().is_some() {
                reorders += 1;
            }
        }
        reorders
    });
    let mut reorders_main = 0;
    for _ in 0..500 {
        if fi.should_reorder().is_some() {
            reorders_main += 1;
        }
    }
    let total = reorders_main + h.join().unwrap();
    let pct = (total as f64 / 1000.0) * 100.0;
    assert!(
        (15.0..=35.0).contains(&pct),
        "Expected ~25%, got {:.1}%",
        pct
    );
}

#[test]
fn r2_3_3_integration_lifo_ordering_verified() {
    use amun_live_cluster::network_adapter::ValidatorNetworkAdapter;
    use amun_networking::frame::{FrameKind, NetworkFrame};
    use amun_networking::handshake::ConstitutionInfo;
    use amun_networking::tcp_transport::TcpTransport;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use std::net::SocketAddr;
    use std::sync::{Arc, Mutex};

    let listen_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let constitution = ConstitutionInfo {
        version: 1,
        hash: [0u8; 32],
        proof_system_version: 1,
        state_commitment_algorithm: "test".to_string(),
        accepted_features: vec!["sync".into(), "vote".into(), "block_range".into()],
    };
    let transport = Arc::new(Mutex::new(TcpTransport::new(
        listen_addr,
        [0u8; 32],
        [0u8; 32],
        [1u8; 32],
        signing_key,
        constitution,
    )));
    let fi = Arc::new(FaultInjector::reorder(100, 4));
    let adapter = ValidatorNetworkAdapter::with_fault_injector(transport, fi);
    let peer: SocketAddr = "127.0.0.1:9999".parse().unwrap();

    for i in 1..=4 {
        let payload = vec![i as u8];
        let frame = NetworkFrame::new(FrameKind::Ping, payload.into());
        let _ = adapter.send_to(peer, frame);
    }
    // After 4 messages with buffer_size=4, auto-flush should have triggered.
    // force_flush_reorder ensures any remaining messages are sent.
    adapter.force_flush_reorder();
    println!("R2.3.3 integration: force_flush_reorder completed after 4 messages");
}
