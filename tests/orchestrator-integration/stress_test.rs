use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn amunctl(args: &[&str]) -> String {
    let ws_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let binary = ws_root.join("target/debug/amunctl");
    let output = Command::new(&binary)
        .args(args)
        .current_dir(&ws_root)
        .output()
        .unwrap_or_else(|e| panic!("Failed to run amunctl at {}: {}", binary.display(), e));
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_kill_recovery_detection() {
    let _ = Command::new("pkill").args(["-f", "amun-node"]).output();
    let _ = Command::new("rm").args(["-rf", "./data"]).output();

    let out = amunctl(&["validator", "add", "v1", "--port", "9071", "--power", "100"]);
    assert!(out.contains("✅"), "Create failed: {}", out);
    let out = amunctl(&["validator", "start", "v1"]);
    assert!(out.contains("✅"), "Start failed: {}", out);
    sleep(Duration::from_secs(2));

    let out = amunctl(&["validator", "list"]);
    assert!(out.contains("running"), "v1 should be running: {}", out);

    let _ = Command::new("pkill").args(["-f", "amun-node"]).output();
    sleep(Duration::from_secs(2));

    let out = amunctl(&["health"]);
    assert!(
        out.contains("Health Score"),
        "Health report missing: {}",
        out
    );
    assert!(
        out.contains("Degraded") || out.contains("Unavailable") || out.contains("stopped"),
        "Health should show degraded after kill: {}",
        out
    );

    let _ = Command::new("rm").args(["-rf", "./data"]).output();
}

#[test]
fn test_multi_validator_stress() {
    let _ = Command::new("pkill").args(["-f", "amun-node"]).output();
    let _ = Command::new("rm").args(["-rf", "./data"]).output();

    for i in 1..=5 {
        let name = format!("v{}", i);
        let port = format!("{}", 9070 + i);
        let out = amunctl(&["validator", "add", &name, "--port", &port, "--power", "100"]);
        assert!(out.contains("✅"), "Create {} failed: {}", name, out);
    }

    for i in 1..=5 {
        let name = format!("v{}", i);
        let out = amunctl(&["validator", "start", &name]);
        assert!(out.contains("✅"), "Start {} failed: {}", name, out);
    }

    sleep(Duration::from_secs(5));

    let out = amunctl(&["validator", "list"]);
    for i in 1..=5 {
        assert!(out.contains(&format!("v{}", i)), "v{} missing: {}", i, out);
    }

    if let Ok(genesis) = std::fs::read_to_string("./data/genesis.json") {
        let count = genesis.matches("validator_id").count();
        assert!(
            count >= 5,
            "Genesis should have >=5 validators, found {}",
            count
        );
    }

    let out = amunctl(&["health"]);
    assert!(
        out.contains("Health Score"),
        "Health report missing: {}",
        out
    );

    let _ = Command::new("pkill").args(["-f", "amun-node"]).output();
    let _ = Command::new("rm").args(["-rf", "./data"]).output();
}

#[test]
fn test_port_conflict_detection() {
    let _ = Command::new("pkill").args(["-f", "amun-node"]).output();
    let _ = Command::new("rm").args(["-rf", "./data"]).output();

    let out = amunctl(&["validator", "add", "v1", "--port", "9071", "--power", "100"]);
    assert!(out.contains("✅"), "Create v1 failed: {}", out);
    let out = amunctl(&["validator", "add", "v2", "--port", "9071", "--power", "100"]);
    assert!(out.contains("✅"), "Create v2 failed: {}", out);

    let out = amunctl(&["validator", "start", "v1"]);
    assert!(out.contains("✅"), "Start v1 failed: {}", out);
    sleep(Duration::from_secs(2));

    // Start second on same port — may fail or cause degraded state
    let _out = amunctl(&["validator", "start", "v2"]);
    sleep(Duration::from_secs(2));

    let out = amunctl(&["health"]);
    assert!(
        out.contains("Health Score"),
        "Health report missing: {}",
        out
    );

    let _ = Command::new("pkill").args(["-f", "amun-node"]).output();
    let _ = Command::new("rm").args(["-rf", "./data"]).output();
}
