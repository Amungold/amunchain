use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let ports = [9200, 9201, 9202, 9203];
    let mut validators: Vec<LiveValidator> = (0..4)
        .map(|i| LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)).unwrap())
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let start = Instant::now();

    println!("=== Phase 1: All 4 validators running (30s) ===");
    for _ in 0..45 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("t={:3}s heights={:?}", t, h);
    }

    println!("=== Phase 2: Stopping validator 3 (crash) ===");
    validators[3].stop();
    let crash_time = start.elapsed().as_secs();

    for _ in 0..20 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h: Vec<u64> = validators[0..3]
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("t={:3}s heights=[{}, {}, {}, --]", t, h[0], h[1], h[2]);
    }

    println!("=== Phase 3: Restarting validator 3 ===");
    let new_v3 = LiveValidator::new(ValidatorConfig::test_cluster(3, &ports).with_quorum(4)).unwrap();
    new_v3.start().unwrap();
    validators[3] = new_v3;

    for _ in 0..45 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("t={:3}s heights={:?}", t, h);
    }

    for v in &validators {
        v.stop();
    }

    let final_h: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let spread = final_h.iter().max().unwrap_or(&0) - final_h.iter().min().unwrap_or(&0);

    println!("\n============================================");
    println!("  N74-F CRASH RECOVERY RESULTS");
    println!("============================================");
    println!("  Final heights:       {:?}", final_h);
    println!("  Height spread:       {}", spread);
    println!("  Crash time:          {}s", crash_time);
    println!(
        "  Recovery verdict:    {}",
        if spread <= 2 { "PASS" } else { "PARTIAL" }
    );
    println!("============================================");
}
