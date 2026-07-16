use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::{LiveValidator, RuntimeSummary};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let ports = [9300, 9301, 9302, 9303];
    let mut validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)).unwrap()
        })
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let start = Instant::now();

    println!("=== Phase 1: All 4 validators (20s) ===");
    for _ in 0..10 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("t={:3}s heights={:?}", t, h);
    }

    println!("=== Phase 2: Partition — stop validators 2 & 3 (20s) ===");
    validators[2].stop();
    validators[3].stop();

    for _ in 0..10 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h0 = validators[0].store.lock().unwrap().latest_height();
        let h1 = validators[1].store.lock().unwrap().latest_height();
        println!("t={:3}s heights=[{}, {}, --, --]", t, h0, h1);
    }

    println!("=== Phase 3: Heal partition — restart validators 2 & 3 ===");
    let v2 = LiveValidator::new(ValidatorConfig::test_cluster(2, &ports).with_quorum(4)).unwrap();
    let v3 = LiveValidator::new(ValidatorConfig::test_cluster(3, &ports).with_quorum(4)).unwrap();
    v2.start().unwrap();
    v3.start().unwrap();
    validators[2] = v2;
    validators[3] = v3;

    // Give restarted validators time to open their listening ports
    println!("=== Warmup: waiting 5s for restarted validators to be ready ===");
    thread::sleep(Duration::from_secs(5));

    // R2.2.1-3: Catch-up loop with 60s timeout
    let heal_deadline = Instant::now() + Duration::from_secs(60);
    println!("=== Phase 4: Waiting for partition healing (timeout: 60s) ===");
    loop {
        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        let max_h = heights.iter().max().copied().unwrap_or(0);
        let min_h = heights.iter().min().copied().unwrap_or(0);
        let spread = max_h - min_h;
        println!("Heights: {:?} (spread={})", heights, spread);

        if spread == 0 {
            println!(
                "Partition healed: all validators converged at height {}",
                max_h
            );
            break;
        }
        if Instant::now() >= heal_deadline {
            eprintln!(
                "R2.2 FAIL: Partition healing timeout after 60s. Heights: {:?} (spread={})",
                heights, spread
            );
            std::process::exit(1);
        }
        thread::sleep(Duration::from_secs(2));
    }

    // R2.2.4: Verify history_root convergence across all validators
    let summaries: Vec<RuntimeSummary> = validators.iter().map(|v| v.runtime_summary()).collect();
    let reference_root = summaries[0].history_root;
    let roots_match = summaries.iter().all(|s| s.history_root == reference_root);

    for v in &validators {
        v.stop();
    }

    let final_h: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let spread = final_h.iter().max().unwrap_or(&0) - final_h.iter().min().unwrap_or(&0);
    let all_equal = spread == 0;

    println!();
    println!("============================================");
    println!("  R2.2 PARTITION RECOVERY RESULTS");
    println!("============================================");
    println!("  Final heights:       {:?}", final_h);
    println!("  Height spread:       {}", spread);
    println!("  All equal:           {}", all_equal);
    println!("  History roots match:  {}", roots_match);
    println!(
        "  Recovery verdict:    {}",
        if all_equal && roots_match {
            "PASS"
        } else {
            "FAIL"
        }
    );
    println!("============================================");

    if !all_equal {
        eprintln!("R2.2 FAIL: height spread != 0");
        std::process::exit(1);
    }
    if !roots_match {
        eprintln!("R2.2 FAIL: history_root mismatch after partition recovery");
        std::process::exit(1);
    }
}
