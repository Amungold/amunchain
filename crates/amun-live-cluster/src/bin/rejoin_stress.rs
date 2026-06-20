use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use amun_sync::catch_up::{append_missing_records, download_missing_records};
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

fn sync_to_tip(v: &LiveValidator, peer_addr: SocketAddr) {
    let peers = vec![peer_addr];
    let local_h = v.store.lock().unwrap().latest_height();
    eprintln!("  sync_to_tip: local_h={}, peer={}", local_h, peer_addr);

    match download_missing_records(local_h, &peers) {
        Ok(records) => {
            eprintln!("  sync_to_tip: downloaded {} records", records.len());
            if records.is_empty() {
                eprintln!(
                    "  sync_to_tip: WARNING - no records downloaded, peer may be at same height"
                );
            }
            if !records.is_empty() {
                let new_h = {
                    let mut store_g = v.store.lock().unwrap();
                    append_missing_records(&mut store_g, local_h, records).unwrap_or(local_h)
                };
                let mut eng = v.engine.lock().unwrap();
                if new_h > eng.current_height {
                    eng.current_height = new_h;
                    eng.rounds.clear();
                    // N102.6: Also update history_root from store to match the chain
                    let store = v.store.lock().unwrap();
                    if let Some(tip) = store.load_tip() {
                        eng.history_root = tip.history_root;
                        eprintln!(
                            "  sync_to_tip: history_root updated to {:?}",
                            &tip.history_root[..4]
                        );
                    }
                    eprintln!("  sync_to_tip: engine advanced to {}", new_h);
                } else {
                    eprintln!(
                        "  sync_to_tip: no advance (new_h={} <= current={})",
                        new_h, eng.current_height
                    );
                }
            }
        }
        Err(e) => eprintln!("  sync_to_tip error: {}", e),
    }
}

fn main() {
    let base_port: u16 = 9900;
    let cycles: u64 = 3;
    let crash_interval: u64 = 90;
    let mut passed: u64 = 0;
    let mut failed: u64 = 0;

    let data_dirs: Vec<String> = (0..4)
        .map(|i| format!("/tmp/amun-test-validator-{}", i))
        .collect();

    let mut validators: Vec<LiveValidator> = (0..4)
        .map(|i| {
            let mut config = ValidatorConfig::test_cluster(
                i,
                &[base_port, base_port + 1, base_port + 2, base_port + 3],
            )
            .with_quorum(4);
            config.data_dir = data_dirs[i].clone();
            LiveValidator::new(config)
        })
        .collect();

    for v in &validators {
        v.start().unwrap();
    }
    println!("=== N100.4 REJOIN STRESS (FIXED FINAL) ===");
    println!("Cycles: {} | Interval: {}s", cycles, crash_interval);
    thread::sleep(Duration::from_secs(crash_interval));

    for cycle in 1..=cycles {
        let target = (cycle % 4) as usize;
        let peer_idx = (target + 1) % 4;
        let peer_addr: SocketAddr = format!("127.0.0.1:{}", base_port + peer_idx as u16)
            .parse()
            .unwrap();

        println!("Cycle {}/{} | Killing validator {}", cycle, cycles, target);
        let pre_height = validators[target].store.lock().unwrap().latest_height();
        validators[target].stop();
        thread::sleep(Duration::from_secs(10));

        println!("  Restarting validator {}...", target);
        let mut config = ValidatorConfig::test_cluster(
            target,
            &[base_port, base_port + 1, base_port + 2, base_port + 3],
        )
        .with_quorum(4);
        config.data_dir = data_dirs[target].clone();
        let new_v = LiveValidator::new(config);
        new_v.start().unwrap();
        validators[target] = new_v;

        // Wait for listen thread to be ready, then sync
        thread::sleep(Duration::from_secs(5));
        sync_to_tip(&validators[target], peer_addr);

        // Wait for convergence
        for t in [10, 20, 30, 45, 60] {
            thread::sleep(Duration::from_secs(t));
            let h = validators[target].store.lock().unwrap().latest_height();
            eprintln!("  t+{}s: height={}", t, h);
            if h > pre_height + 3 {
                break;
            }
        }

        let heights: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        let min_h = *heights.iter().min().unwrap_or(&0);
        let max_h = *heights.iter().max().unwrap_or(&0);
        let spread = max_h - min_h;
        let post_height = heights[target];

        if spread <= 5 && post_height > pre_height {
            passed += 1;
            println!(
                "  Result: PASS | pre_h={} post_h={} spread={}",
                pre_height, post_height, spread
            );
        } else {
            failed += 1;
            println!(
                "  Result: FAIL | pre_h={} post_h={} spread={}",
                pre_height, post_height, spread
            );
        }
    }

    for v in &validators {
        v.stop();
    }
    let final_h: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let final_spread = final_h.iter().max().unwrap_or(&0) - final_h.iter().min().unwrap_or(&0);
    println!("\n============================================");
    println!("  Passed: {}  Failed: {}", passed, failed);
    println!("  Final spread: {}", final_spread);
    println!(
        "  Verdict: {}",
        if failed == 0 && final_spread <= 5 {
            "PASS"
        } else {
            "PARTIAL"
        }
    );
    println!("============================================");
}
