use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let ports = [9300, 9301, 9302, 9303];
    let mut validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            LiveValidator::new(
                ValidatorConfig::test_cluster(i, &ports).with_quorum(4),
            )
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
        let h: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
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

    println!("=== Phase 3: Heal partition — restart validators 2 & 3 (30s) ===");
    let v2 = LiveValidator::new(
        ValidatorConfig::test_cluster(2, &ports).with_quorum(4),
    );
    let v3 = LiveValidator::new(
        ValidatorConfig::test_cluster(3, &ports).with_quorum(4),
    );
    v2.start().unwrap();
    v3.start().unwrap();
    validators[2] = v2;
    validators[3] = v3;

    for _ in 0..20 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
        println!("t={:3}s heights={:?}", t, h);
    }

    println!("=== Cooldown 10s ===");
    thread::sleep(Duration::from_secs(10));
    let h_cooldown: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
    println!("After cooldown: {:?}", h_cooldown);

    for v in &validators {
        v.stop();
    }

    let final_h: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
    let spread = final_h.iter().max().unwrap_or(&0) - final_h.iter().min().unwrap_or(&0);
    let all_equal = spread == 0;

    println!("\n============================================");
    println!("  N75 PARTITION RECOVERY RESULTS");
    println!("============================================");
    println!("  Final heights:       {:?}", final_h);
    println!("  Height spread:       {}", spread);
    println!("  All equal:           {}", all_equal);
    println!("  Recovery verdict:    {}", if all_equal { "PASS" } else { "PARTIAL" });
    println!("============================================");
}
