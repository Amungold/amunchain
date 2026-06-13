use amun_consensus_network::messages::{ConsensusVote, EquivocationProof, SignedVote};
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

    for v in &validators { v.start().unwrap(); }

    println!("=== N103.2 EVIDENCE PIPELINE TEST ===");
    println!();

    println!("Phase 1: Warmup (30s)...");
    thread::sleep(Duration::from_secs(30));
    let initial: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
    println!("  Initial heights: {:?}", initial);

    println!("\nPhase 2: Evidence pipeline...");
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    {
        let mut eng = validators[0].engine.lock().unwrap();
        let h = eng.current_height + 1;

        let vote_a = ConsensusVote {
            voter_id: [88u8; 32], height: h,
            block_hash: [0xAA; 32], state_root: [0xBB; 32],
            approve: true, signature: [1u8; 64], timestamp: now,
        };
        let vote_b = ConsensusVote {
            voter_id: [88u8; 32], height: h,
            block_hash: [0xFF; 32], state_root: [0xBB; 32],
            approve: true, signature: [2u8; 64], timestamp: now + 1,
        };

        let proof = EquivocationProof {
            validator_id: [88u8; 32],
            height: h,
            round: h,
            vote_a: SignedVote { vote: vote_a, signature: [1u8; 64] },
            vote_b: SignedVote { vote: vote_b, signature: [2u8; 64] },
            detected_at_height: h,
        };

        let pre = eng.misbehavior_registry.all_proofs().len();
        match eng.misbehavior_registry.add_proof(proof) {
            Ok(hash) => {
                let post = eng.misbehavior_registry.all_proofs().len();
                println!("  Evidence recorded: hash={:?}", &hash[..8]);
                println!("  Registry: {} -> {}", pre, post);
                println!("  Evidence pipeline: PASS");
            }
            Err(e) => println!("  Evidence recording failed: {}", e),
        }
    }

    thread::sleep(Duration::from_secs(10));
    let final_heights: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
    let max_h = *final_heights.iter().max().unwrap_or(&0);
    let min_h = *final_heights.iter().min().unwrap_or(&0);

    for v in &validators { v.stop(); }

    println!("\n============================================");
    println!("  Final heights: {:?} spread={}", final_heights, max_h - min_h);
    println!("  Network: {}", if max_h - min_h <= 2 { "PASS" } else { "FAIL" });
    println!("============================================");
}
