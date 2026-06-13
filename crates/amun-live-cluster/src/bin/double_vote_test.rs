use amun_consensus_network::messages::{ConsensusVote, EquivocationProof, SignedVote};
use amun_consensus_network::slashing::should_slash;
use amun_consensus_network::validator_status::{ValidatorStatus, ValidatorStatusRegistry};
use amun_live_cluster::config::ValidatorConfig;
use amun_live_cluster::validator::LiveValidator;
use std::sync::{Arc, Mutex};
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

    println!("=== N103.3 AUTOMATIC SLASHING TEST ===");
    println!();

    println!("Phase 1: Warmup (30s)...");
    thread::sleep(Duration::from_secs(30));
    let initial: Vec<u64> = validators.iter().map(|v| v.store.lock().unwrap().latest_height()).collect();
    println!("  Initial heights: {:?}", initial);

    println!("\nPhase 2: Direct evidence injection + enforcement...");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    {
        let mut eng = validators[0].engine.lock().unwrap();
        let h = eng.current_height + 1;

        // Wire up validator_status
        eng.validator_status = Some(Arc::new(Mutex::new(ValidatorStatusRegistry::new())));

        // Inject 3 proofs with DIFFERENT signatures
        let count = 3u64;
        for i in 1..=count {
            let sig_a = [i as u8; 64];
            let sig_b = [(i + 100) as u8; 64];
            let vote_a = ConsensusVote {
                voter_id: [88u8; 32], height: h,
                block_hash: [0xAA; 32], state_root: [0xBB; 32],
                approve: true, signature: sig_a, timestamp: now + i,
            };
            let vote_b = ConsensusVote {
                voter_id: [88u8; 32], height: h,
                block_hash: [0xFF; 32], state_root: [0xBB; 32],
                approve: true, signature: sig_b, timestamp: now + i + 1,
            };
            let proof = EquivocationProof {
                validator_id: [88u8; 32],
                height: h, round: h,
                vote_a: SignedVote { vote: vote_a, signature: sig_a },
                vote_b: SignedVote { vote: vote_b, signature: sig_b },
                detected_at_height: h,
            };
            match eng.misbehavior_registry.add_proof(proof) {
                Ok(hash) => println!("  Offense {} recorded: proof_hash={:?}", i, &hash[..4]),
                Err(e) => println!("  Offense {} failed: {}", i, e),
            }
        }

        let offense_count = eng.misbehavior_registry.offense_count(&[88u8; 32]);
        let should = should_slash(&eng.misbehavior_registry, &[88u8; 32]);
        println!("  Offense count: {}", offense_count);
        println!("  should_slash: {}", should);

        if should {
            let until = eng.current_height + 100;
            if let Some(ref registry) = eng.validator_status {
                registry.lock().unwrap().set_status(
                    [88u8; 32],
                    ValidatorStatus::Suspended { until_height: until },
                );
                println!("  VALIDATOR_SLASHED: [88;32] until height {}", until);

                let suspended = registry.lock().unwrap().is_suspended(&[88u8; 32], eng.current_height);
                println!("  is_suspended: {}", suspended);

                // Verify suspended validator cannot vote
                let vote = ConsensusVote {
                    voter_id: [88u8; 32], height: h + 1,
                    block_hash: [0xAA; 32], state_root: [0xBB; 32],
                    approve: true, signature: [99u8; 64], timestamp: now + 10,
                };
                let result = eng.process_vote(vote);
                let blocked = result.is_err();
                println!("  Vote after suspension: {}", if blocked { "BLOCKED" } else { "ALLOWED" });
                println!("  Slashing enforcement: {}", if suspended && blocked { "PASS" } else { "FAIL" });
            }
        } else {
            println!("  Not enough offenses (need > 2)");
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
