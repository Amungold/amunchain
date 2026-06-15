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
        .map(LiveValidator::new)
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let start = Instant::now();
    println!("=== N102.7 MULTI-VALIDATOR TEST ===");
    println!("Validators: {} | Quorum: {}", count, quorum);
    println!();

    // Phase 1: consensus convergence (60s)
    println!("Phase 1: Consensus convergence...");
    for _ in 0..60 {
        thread::sleep(Duration::from_secs(1));
        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        let min_h = *heights.iter().min().unwrap_or(&0);
        let max_h = *heights.iter().max().unwrap_or(&0);
        if heights.iter().all(|h| *h > 0) && max_h - min_h <= 1 {
            println!(
                "  Converged at t={}s: heights={:?} spread={}",
                start.elapsed().as_secs(),
                heights,
                max_h - min_h
            );
            break;
        }
    }

    let pre_kill_heights: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    println!("  Pre-kill heights: {:?}", pre_kill_heights);

    // Phase 2: kill validator 2, verify quorum continues (30s)
    println!("\nPhase 2: Killing validator 2...");
    validators[2].stop();
    thread::sleep(Duration::from_secs(30));

    let after_kill: Vec<u64> = validators
        .iter()
        .enumerate()
        .map(|(i, v)| {
            if i == 2 {
                0
            } else {
                v.store.lock().unwrap().latest_height()
            }
        })
        .collect();
    println!("  After kill heights: {:?}", after_kill);
    let progress = after_kill[0] > pre_kill_heights[0]
        && after_kill[1] > pre_kill_heights[1]
        && after_kill[3] > pre_kill_heights[3];
    println!(
        "  Progress made: {}",
        if progress { "PASS" } else { "FAIL" }
    );

    // Phase 3: restart validator 2, verify catch-up (60s)
    println!("\nPhase 3: Restarting validator 2...");
    let ports = [base_port, base_port + 1, base_port + 2, base_port + 3];
    let v2_config = ValidatorConfig::test_cluster(2, &ports).with_quorum(quorum);
    let v2 = LiveValidator::new(v2_config);
    v2.start().unwrap();
    validators[2] = v2;

    thread::sleep(Duration::from_secs(60));

    let final_heights: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let final_min = *final_heights.iter().min().unwrap_or(&0);
    let final_max = *final_heights.iter().max().unwrap_or(&0);
    let final_spread = final_max - final_min;
    println!(
        "  Final heights: {:?} spread={}",
        final_heights, final_spread
    );

    for v in &validators {
        v.stop();
    }

    let recovered = final_spread <= 2 && final_heights[2] > 0;

    println!("\n============================================");
    println!("  N102.7 MULTI-VALIDATOR TEST RESULTS");
    println!("============================================");
    println!("  Consensus:    {}", if progress { "PASS" } else { "FAIL" });
    println!(
        "  Crash-rejoin: {}",
        if recovered { "PASS" } else { "FAIL" }
    );
    println!("  Final spread: {}", final_spread);
    println!(
        "  Verdict:      {}",
        if progress && recovered {
            "PASS"
        } else {
            "PARTIAL"
        }
    );
    println!("============================================");
}
