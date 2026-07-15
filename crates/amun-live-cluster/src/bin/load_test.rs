use amun_consensus_network::messages::ConsensusVote;
use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use amun_network_transport::protocol::send_vote;
use std::net::TcpStream;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let levels: Vec<(&str, u64, u64)> = vec![
        ("10 TPS", 10, 60),
        ("25 TPS", 25, 60),
        ("50 TPS", 50, 60),
        ("75 TPS", 75, 60),
        ("100 TPS", 100, 60),
        ("150 TPS", 150, 60),
    ];

    let ports = [9100, 9101, 9102, 9103];
    let validators: Vec<LiveValidator> = (0..4)
        .map(|i| LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)).unwrap())
        .collect();

    for v in &validators {
        v.start().unwrap();
    }
    thread::sleep(Duration::from_secs(5));

    println!("=== N85 GRADUATED LOAD TEST ===");
    println!();

    let mut baseline_height = validators[0].store.lock().unwrap().latest_height();

    for (label, tps, duration_secs) in &levels {
        println!("--- {} for {}s ---", label, duration_secs);

        let interval_us = 1_000_000 / tps;
        let start = Instant::now();
        let mut tx_count: u64 = 0;

        while start.elapsed().as_secs() < *duration_secs {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64;

            let mut voter_id = [0u8; 32];
            voter_id[0..8].copy_from_slice(&(tx_count % 4 + 1).to_le_bytes());
            voter_id[8..16].copy_from_slice(&now.to_le_bytes());

            let mut block_hash = [0u8; 32];
            block_hash[0..8].copy_from_slice(&tx_count.to_le_bytes());

            let vote = ConsensusVote {
                voter_id,
                height: 0,
                block_hash,
                state_root: [0xBB; 32],
                approve: true,
                signature: [0u8; 64],
                timestamp: now / 1_000_000,
                commitment: None,
            };
            let data = postcard::to_stdvec(&vote).unwrap();

            let target_port = ports[(tx_count % 4) as usize];
            let addr = format!("127.0.0.1:{}", target_port);
            if let Ok(mut stream) =
                TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(50))
            {
                let _ = send_vote(&mut stream, &data);
            }

            tx_count += 1;
            if tx_count.is_multiple_of(100) {
                let h: Vec<u64> = validators
                    .iter()
                    .map(|v| v.store.lock().unwrap().latest_height())
                    .collect();
                let min = *h.iter().min().unwrap_or(&0);
                let max = *h.iter().max().unwrap_or(&0);
                println!("  tx={} heights={:?} spread={}", tx_count, h, max - min);
            }

            thread::sleep(Duration::from_micros(interval_us));
        }

        let final_h: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        let min_h = *final_h.iter().min().unwrap_or(&0);
        let max_h = *final_h.iter().max().unwrap_or(&0);
        let spread = max_h - min_h;
        let blocks_produced = max_h.saturating_sub(baseline_height);
        let elapsed = start.elapsed().as_secs();
        let actual_tps = if elapsed > 0 {
            blocks_produced as f64 / elapsed as f64
        } else {
            0.0
        };

        let verdict = if spread <= 1 {
            "PASS"
        } else if spread <= 3 {
            "DEGRADED"
        } else {
            "FAIL"
        };

        println!(
            "  Result: {} | spread={} | blocks={} | actual_tps={:.1} | verdict={}",
            label, spread, blocks_produced, actual_tps, verdict
        );

        baseline_height = max_h;

        if verdict == "FAIL" {
            println!("  Stopping test at saturation point.");
            break;
        }
    }

    for v in &validators {
        v.stop();
    }

    println!("\n============================================");
    println!("  N85 LOAD TEST COMPLETE");
    println!("============================================");
}
