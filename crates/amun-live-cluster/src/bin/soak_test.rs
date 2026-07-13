use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let ports = [9900, 9901, 9902, 9903];
    let validators: Vec<LiveValidator> = (0..4)
        .map(|i| LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)))
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let start = Instant::now();
    let duration = 3600;
    let interval = 60;
    let mut prev_max: u64 = 0;
    let crash_time = Duration::from_secs(120);
    let restart_time = Duration::from_secs(150);
    let mut crashed = false;
    let mut restarted = false;

    println!("=== N84 SOAK TEST: {} minutes ===", duration / 60);

    for _min in 1..=(duration / interval) {
        thread::sleep(Duration::from_secs(interval));
        if !crashed && start.elapsed() >= crash_time {
            validators[3].stop();
            println!("=== CRASH: Validator 3 stopped ===");
            crashed = true;
        }
        if crashed && !restarted && start.elapsed() >= restart_time {
            validators[3].start().unwrap();
            println!("=== RECOVERY: Validator 3 restarted ===");
            restarted = true;
        }
        let elapsed = start.elapsed().as_secs();
        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        let min_h = *heights.iter().min().unwrap_or(&0);
        let max_h = *heights.iter().max().unwrap_or(&0);
        let spread = max_h - min_h;
        let blocks = max_h - prev_max;
        let tps = blocks as f64 / interval as f64;
        prev_max = max_h;

        println!(
            "t={:4}min  min={}  max={}  spread={}  blocks={}  tps={:.1}",
            elapsed / 60,
            min_h,
            max_h,
            spread,
            blocks,
            tps
        );

        if spread > 2 {
            eprintln!(
                "WARN: spread={} exceeds threshold at t={}min",
                spread,
                elapsed / 60
            );
        }
        if blocks == 0 {
            eprintln!("STALL: no blocks produced in last {}s", interval);
            break;
        }
    }

    for v in &validators {
        v.stop();
    }

    let final_h: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let spread = final_h.iter().max().unwrap_or(&0) - final_h.iter().min().unwrap_or(&0);
    let total_time = start.elapsed().as_secs() as f64 / 60.0;
    let total_blocks = *final_h.iter().max().unwrap_or(&0);

    println!("\n============================================");
    println!("  N84 SOAK TEST RESULTS");
    println!("============================================");
    println!("  Duration:       {:.0} min", total_time);
    println!("  Total blocks:   {}", total_blocks);
    println!("  Final heights:  {:?}", final_h);
    println!("  Spread:         {}", spread);
    println!(
        "  Avg TPS:        {:.2}",
        total_blocks as f64 / (total_time * 60.0)
    );
    println!(
        "  Verdict:        {}",
        if spread <= 2 { "PASS" } else { "DEGRADED" }
    );
    println!("============================================");
}
