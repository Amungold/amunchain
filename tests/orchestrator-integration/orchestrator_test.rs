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
fn test_validator_lifecycle() {
    // Cleanup
    let _ = Command::new("pkill").args(["-f", "amun-node"]).output();
    let _ = Command::new("rm").args(["-rf", "./data"]).output();

    // 1. Create two validators
    let out = amunctl(&["validator", "add", "v1", "--port", "9071", "--power", "100"]);
    assert!(out.contains("✅"), "Create v1 failed: {}", out);

    let out = amunctl(&["validator", "add", "v2", "--port", "9072", "--power", "100"]);
    assert!(out.contains("✅"), "Create v2 failed: {}", out);

    // 2. Start both validators
    let out = amunctl(&["validator", "start", "v1"]);
    assert!(out.contains("✅"), "Start v1 failed: {}", out);

    let out = amunctl(&["validator", "start", "v2"]);
    assert!(out.contains("✅"), "Start v2 failed: {}", out);

    sleep(Duration::from_secs(3));

    // 3. Check list shows both validators
    let out = amunctl(&["validator", "list"]);
    assert!(out.contains("v1"), "List missing v1: {}", out);
    assert!(out.contains("v2"), "List missing v2: {}", out);

    // 4. Health check returns a score
    let out = amunctl(&["health"]);
    assert!(
        out.contains("Health Score"),
        "Health report missing: {}",
        out
    );
    assert!(
        out.contains("Operational State"),
        "Operational state missing: {}",
        out
    );

    // 5. Restart v1
    let out = amunctl(&["validator", "restart", "v1"]);
    assert!(out.contains("✅"), "Restart v1 failed: {}", out);
    sleep(Duration::from_secs(2));

    // 6. Verify v1 still exists in list
    let out = amunctl(&["validator", "list"]);
    assert!(out.contains("v1"), "v1 missing after restart: {}", out);

    // 7. Remove v2
    let out = amunctl(&["validator", "remove", "v2"]);
    assert!(out.contains("✅"), "Remove v2 failed: {}", out);

    // 8. Verify v2 is gone from list
    let out = amunctl(&["validator", "list"]);
    assert!(!out.contains("v2"), "v2 still present: {}", out);

    // Cleanup
    let _ = Command::new("pkill").args(["-f", "amun-node"]).output();
    let _ = Command::new("rm").args(["-rf", "./data"]).output();
}
