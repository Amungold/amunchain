use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::testing::{cleanup, free_ports, unique_test_dir};
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::Duration;

#[test]
fn n102_3_catchup_after_50_block_gap() {
    let ports = free_ports::<4>();

    // Clean up first
    let mut dirs = Vec::new();

    for i in 0..4 {
        let dir = unique_test_dir("n102", i);
        cleanup(&dir);
        dirs.push(dir);
    }
    // Start 4 validators
    let mut validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            let mut config = ValidatorConfig::test_cluster(i, &ports).with_quorum(3);
            config.data_dir = dirs[i].clone();
            LiveValidator::new(config).unwrap().unwrap()
        })
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    println!("=== Waiting for chain to reach height 100+ ... ===");
    // Wait until all validators reach height 100+
    for _ in 0..120 {
        thread::sleep(Duration::from_secs(1));
        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("Heights: {:?}", heights);
        if heights.iter().all(|&h| h >= 100) {
            break;
        }
    }

    let initial_heights: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    println!("Initial heights: {:?}", initial_heights);

    // Stop validator 3
    println!("=== Stopping validator 3 ===");
    let v3_height_before = validators[3].store.lock().unwrap().latest_height();
    validators[3].stop();
    println!("Validator 3 stopped at height {}", v3_height_before);

    // Let others advance 50+ blocks - wait longer
    println!("=== Waiting for others to advance 50+ blocks... ===");
    for i in 0..300 {
        thread::sleep(Duration::from_secs(1));
        let max_h = validators[0..3]
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .max()
            .unwrap_or(0);
        if i % 10 == 0 {
            println!(
                "Max height of active validators: {} (target: {})",
                max_h,
                v3_height_before + 50
            );
        }
        if max_h >= v3_height_before + 50 {
            println!(
                "Target reached! max_h={} >= {}",
                max_h,
                v3_height_before + 50
            );
            break;
        }
    }

    let max_height = validators[0..3]
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .max()
        .unwrap_or(0);
    println!("Max height reached: {}", max_height);

    // Restart validator 3
    println!("=== Restarting validator 3 ===");
    let mut config = ValidatorConfig::test_cluster(3, &ports).with_quorum(3);
    config.data_dir = dirs[3].clone();
    let v3_new = LiveValidator::new(config).unwrap().unwrap();
    v3_new.start().unwrap();
    validators[3] = v3_new;

    // Wait for catchup
    println!("=== Waiting for catchup... ===");
    let mut caught_up = false;
    for i in 0..60 {
        thread::sleep(Duration::from_secs(2));
        let h = validators[3].store.lock().unwrap().latest_height();
        let max_h = validators[0..3]
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .max()
            .unwrap_or(0);
        let spread = max_h.abs_diff(h);
        println!(
            "Validator 3 height: {}, max: {}, spread: {}",
            h, max_h, spread
        );
        if spread <= 2 {
            caught_up = true;
            println!("Catchup successful after {} seconds", (i + 1) * 2);
            break;
        }
    }

    // Final assertions
    let final_h = validators[3].store.lock().unwrap().latest_height();
    let all_heights: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let max_h = validators[0..3]
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .max()
        .unwrap_or(0);
    let final_spread = max_h.abs_diff(final_h);
    let consensus_pass = final_spread <= 2 && all_heights.iter().all(|h| *h > 0);

    println!("\n============================================");
    println!(" N102.3 CATCHUP AFTER 50 BLOCK GAP");
    println!("============================================");
    println!("  Final heights: {:?}", all_heights);
    println!("  Spread:        {}", final_spread);
    println!(
        "  Catch-up:      {}",
        if caught_up { "PASS" } else { "FAIL" }
    );
    println!(
        "  Consensus:     {}",
        if consensus_pass { "PASS" } else { "FAIL" }
    );
    println!(
        "  Verdict:       {}",
        if caught_up && consensus_pass {
            "PASS"
        } else {
            "FAIL"
        }
    );
    println!("============================================\n");

    // Clean up
    for v in &validators {
        v.stop();
    }
    for dir in &dirs {
        cleanup(dir);
    }
    assert!(caught_up, "Validator 3 failed to catch up within 2 minutes");
    assert!(consensus_pass, "Spread too large: {}", final_spread);
}
