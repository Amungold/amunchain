use amun_consensus_network::validator_status::{ValidatorStatus, ValidatorStatusRegistry};
use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let base_port: u16 = 10001;
    let count: usize = 4;
    let quorum: usize = 3;

    let validators: Vec<LiveValidator> = (0..count)
        .map(|i| {
            let ports = [base_port, base_port + 1, base_port + 2, base_port + 3];
            ValidatorConfig::test_cluster(i, &ports).with_quorum(quorum)
        })
        .map(LiveValidator::new)
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    println!("=== N103.5 QUORUM EXCLUSION TEST ===");
    println!();

    println!("Phase 1: Warmup (30s)...");
    thread::sleep(Duration::from_secs(30));
    let initial: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    println!("  Initial heights: {:?}", initial);

    println!("\nPhase 2: Initialize validator_status, set IDs, suspend validator 2...");
    {
        let mut eng = validators[0].engine.lock().unwrap();

        // Initialize validator_status
        eng.validator_status = Some(Arc::new(Mutex::new(ValidatorStatusRegistry::new())));

        // Set real validator IDs
        eng.validator_ids = vec![[1u8; 32], [2u8; 32], [3u8; 32], [4u8; 32]];

        let active_before = eng.active_validator_count();
        println!("  Active before: {}", active_before);

        // Suspend validator with ID [2u8;32]
        let target = [2u8; 32];
        eng.validator_status
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .set_status(
                target,
                ValidatorStatus::Suspended {
                    until_height: eng.current_height + 200,
                },
            );

        let active_after = eng.active_validator_count();
        println!("  Active after:  {}", active_after);
        println!(
            "  Exclusion:     {}",
            if active_before == 4 && active_after == 3 {
                "PASS"
            } else {
                "FAIL"
            }
        );
    }

    println!("\nPhase 3: Verify network continues...");
    thread::sleep(Duration::from_secs(30));

    let final_heights: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let min_h = *final_heights.iter().min().unwrap_or(&0);
    let max_h = *final_heights.iter().max().unwrap_or(&0);
    let spread = max_h - min_h;
    let final_active = validators[0]
        .engine
        .lock()
        .unwrap()
        .active_validator_count();

    for v in &validators {
        v.stop();
    }

    println!("\n============================================");
    println!("  N103.5 QUORUM EXCLUSION RESULTS");
    println!("============================================");
    println!("  Final active:    {}", final_active);
    println!("  Final heights:   {:?} spread={}", final_heights, spread);
    println!(
        "  Network:         {}",
        if spread <= 2 { "PASS" } else { "FAIL" }
    );
    println!(
        "  Quorum Exclusion: {}",
        if final_active == 3 { "PASS" } else { "PARTIAL" }
    );
    println!("============================================");
}
