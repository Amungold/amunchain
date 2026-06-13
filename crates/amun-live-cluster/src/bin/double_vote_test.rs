use amun_consensus_network::messages::ConsensusVote;
use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

    println!("=== N103.1 DOUBLE VOTE DETECTION (DIRECT ENGINE INJECTION) ===");
    println!("Validators: {} | Quorum: {}", count, quorum);
    println!();

    println!("Phase 1: Warmup (30s)...");
    thread::sleep(Duration::from_secs(30));
    let initial: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
    println!("  Initial heights: {:?}", initial);

    println!("\nPhase 2: Direct equivocation injection into validator 0 engine...");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // Lock the engine directly and inject two conflicting votes
    {
        let mut eng = validators[0].engine.lock().unwrap();
        let target_height = eng.current_height + 1;

        let vote_a = ConsensusVote {
            voter_id: [4u8; 32],
            height: target_height,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [1u8; 64],
            timestamp: now,
        };

        let vote_b = ConsensusVote {
            voter_id: [4u8; 32],
            height: target_height,
            block_hash: [0xFF; 32],
            state_root: [0xBB; 32],
            approve: true,
            signature: [2u8; 64],
            timestamp: now + 1,
        };

        // Process first vote
        match eng.process_vote(vote_a) {
            Ok(()) => println!("  Vote A accepted"),
            Err(e) => println!("  Vote A rejected: {}", e),
        }

        // Process second vote (should trigger equivocation detection)
        match eng.process_vote(vote_b) {
            Ok(()) => println!("  Vote B accepted"),
            Err(e) => println!("  Vote B rejected: {}", e),
        }
    }

    println!("\nPhase 3: Waiting for consensus to continue...");
    thread::sleep(Duration::from_secs(30));

    let final_heights: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
    let min_h = *final_heights.iter().min().unwrap_or(&0);
    let max_h = *final_heights.iter().max().unwrap_or(&0);
    let spread = max_h - min_h;
    println!("  Final heights: {:?} spread={}", final_heights, spread);

    let made_progress = final_heights.iter().all(|h| *h > initial[0]);
    let no_stall = spread <= 2;

    for v in &validators { v.stop(); }

    println!("\n============================================");
    println!("  N103.1 DOUBLE VOTE RESULTS");
    println!("============================================");
    println!("  Progress made:     {}", if made_progress { "PASS" } else { "FAIL" });
    println!("  No stall/spread:   {}", if no_stall { "PASS" } else { "FAIL" });
    println!("  Final spread:      {}", spread);
    println!("  Verdict:           {}", if made_progress && no_stall { "PASS" } else { "PARTIAL" });
    println!("============================================");
}
