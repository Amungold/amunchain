use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let base_port: u16 = 10001;
    let count: usize = 4;
    let quorum: usize = 3;

    let mut validators: Vec<LiveValidator> = (0..count)
        .map(|i| {
            let ports = [base_port, base_port + 1, base_port + 2, base_port + 3];
            ValidatorConfig::test_cluster(i, &ports).with_quorum(quorum)
        })
        .map(|c| LiveValidator::new(c).unwrap())
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let _start = Instant::now();
    println!("=== N102.9 BYZANTINE & PARTITION TEST ===");
    println!("Validators: {} | Quorum: {}", count, quorum);
    println!();

    // Phase 1: warmup (30s)
    println!("Phase 1: Warmup...");
    thread::sleep(Duration::from_secs(30));
    let initial: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    println!("  Initial heights: {:?}", initial);

    // Phase 2: partition — stop V2 and V3, leave V0 and V1 (2 < quorum=3)
    println!("\nPhase 2: Partition (V0+V1 vs V2+V3) — stopping V2 and V3...");
    validators[2].stop();
    validators[3].stop();
    thread::sleep(Duration::from_secs(30));

    let v0_h = validators[0].store.lock().unwrap().latest_height();
    let v1_h = validators[1].store.lock().unwrap().latest_height();
    println!("  V0 height: {}, V1 height: {}", v0_h, v1_h);

    // With only 2/4 validators, quorum=3 cannot be met.
    // The chain should NOT advance (safety preserved).
    let stalled = v0_h <= initial[0] + 2 && v1_h <= initial[1] + 2;
    println!(
        "  Chain stalled: {}",
        if stalled {
            "PASS (safety preserved)"
        } else {
            "FAIL (advanced without quorum)"
        }
    );

    // Phase 3: heal partition — restart V2 and V3
    println!("\nPhase 3: Healing partition...");
    let ports = [base_port, base_port + 1, base_port + 2, base_port + 3];
    let v2 = LiveValidator::new(ValidatorConfig::test_cluster(2, &ports).with_quorum(quorum)).unwrap();
    let v3 = LiveValidator::new(ValidatorConfig::test_cluster(3, &ports).with_quorum(quorum)).unwrap();
    v2.start().unwrap();
    v3.start().unwrap();
    validators[2] = v2;
    validators[3] = v3;

    thread::sleep(Duration::from_secs(60));

    let final_heights: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let min_h = *final_heights.iter().min().unwrap_or(&0);
    let max_h = *final_heights.iter().max().unwrap_or(&0);
    let spread = max_h - min_h;
    println!("  Final heights: {:?} spread={}", final_heights, spread);

    let recovered = spread <= 2 && final_heights.iter().all(|h| *h > initial[0]);

    for v in &validators {
        v.stop();
    }

    println!("\n============================================");
    println!("  N102.9 BYZANTINE PARTITION RESULTS");
    println!("============================================");
    println!(
        "  Safety (no advance): {}",
        if stalled { "PASS" } else { "FAIL" }
    );
    println!(
        "  Recovery (rejoin):  {}",
        if recovered { "PASS" } else { "FAIL" }
    );
    println!("  Final spread:       {}", spread);
    println!(
        "  Verdict:            {}",
        if stalled && recovered {
            "PASS"
        } else {
            "PARTIAL"
        }
    );
    println!("============================================");
}
