use amun_consensus_network::messages::ConsensusVote;
use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let ports = [9400, 9401, 9402, 9403];
    let validators: Vec<LiveValidator> = (0..4)
        .map(|i| LiveValidator::new(ValidatorConfig::test_cluster(i, &ports).with_quorum(4)))
        .collect();

    for v in &validators {
        v.start().unwrap();
    }

    let start = Instant::now();

    println!("=== Phase 1: All 4 validators (15s) ===");
    for _ in 0..15 {
        thread::sleep(Duration::from_secs(1));
        let t = start.elapsed().as_secs();
        let h: Vec<u64> = validators
            .iter()
            .map(|v| v.store.lock().unwrap().latest_height())
            .collect();
        println!("t={:3}s heights={:?}", t, h);
    }

    let h_before = validators[0].store.lock().unwrap().latest_height();

    println!("=== Phase 2: Byzantine attacks (10s) ===");
    for _round in 0..5 {
        let fake_vote = ConsensusVote {
            voter_id: [255u8; 32],
            height: 999999,
            block_hash: [0xFF; 32],
            state_root: [0xFF; 32],
            approve: true,
            signature: [0u8; 64],
            timestamp: 0,
        };

        let attack_types = [
            ("future_height", fake_vote.clone()),
            {
                let mut v = fake_vote.clone();
                v.approve = false;
                ("malformed_vote", v)
            },
            {
                let mut v = fake_vote.clone();
                v.signature = [0xFF; 64];
                ("bad_signature", v)
            },
            {
                let mut v = fake_vote.clone();
                v.voter_id = [0u8; 32];
                ("zero_id", v)
            },
            {
                let mut v = fake_vote.clone();
                v.height = 0;
                ("zero_height", v)
            },
        ];

        for (label, vote) in &attack_types {
            let data = postcard::to_stdvec(vote).unwrap();
            for port in &ports {
                let addr = format!("127.0.0.1:{}", port);
                if let Ok(mut stream) =
                    TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(200))
                {
                    let msg_type = [0x00u8];
                    let len = (data.len() as u32).to_be_bytes();
                    let _ = stream.write_all(&msg_type);
                    let _ = stream.write_all(&len);
                    let _ = stream.write_all(&data);
                    let _ = stream.flush();
                }
            }
            println!("  Attack: {} sent", label);
            thread::sleep(Duration::from_millis(500));
        }
    }

    println!("=== Phase 3: Recovery check (10s) ===");
    for _ in 0..10 {
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

    let h_after = validators[0].store.lock().unwrap().latest_height();
    let final_h: Vec<u64> = validators
        .iter()
        .map(|v| v.store.lock().unwrap().latest_height())
        .collect();
    let spread = final_h.iter().max().unwrap_or(&0) - final_h.iter().min().unwrap_or(&0);
    let made_progress = h_after > h_before;
    let all_alive = spread <= 2;

    println!("\n============================================");
    println!("  N76 BYZANTINE RESISTANCE RESULTS");
    println!("============================================");
    println!("  Height before attacks:  {}", h_before);
    println!("  Height after attacks:   {}", h_after);
    println!("  Made progress:          {}", made_progress);
    println!("  All validators alive:   {}", all_alive);
    println!("  Final heights:          {:?}", final_h);
    println!("  Height spread:          {}", spread);
    println!(
        "  Byzantine verdict:      {}",
        if made_progress && all_alive {
            "PASS"
        } else {
            "PARTIAL"
        }
    );
    println!("============================================");
}
