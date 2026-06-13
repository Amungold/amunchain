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

    println!("\nPhase 2: Injecting offenses + verifying enforcement...");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let threshold = 2u64;

    {
        let mut eng = validators[0].engine.lock().unwrap();
        let h = eng.current_height + 1;

        // Wire up validator_status registry (LiveValidator starts with None)
        eng.validator_status = Some(Arc::new(Mutex::new(ValidatorStatusRegistry::new())));

        // Inject threshold+1 offenses via process_vote to trigger enforcement
        for i in 1..=threshold + 1 {
            let sig_a = [i as u8; 64];
            let sig_b = [(i + 100) as u8; 64];

            // First vote: accepted
            let vote_a = ConsensusVote {
                voter_id: [88u8; 32], height: h,
                block_hash: [0xAA; 32], state_root: [0xBB; 32],
                approve: true, signature: sig_a, timestamp: now + i,
            };
            // Second vote: triggers equivocation → evidence + slashing
            let vote_b = ConsensusVote {
                voter_id: [88u8; 32], height: h,
                block_hash: [0xFF; 32], state_root: [0xBB; 32],
                approve: true, signature: sig_b, timestamp: now + i + 1,
            };

            let _ = eng.process_vote(vote_a);
            let result = eng.process_vote(vote_b);
            println!("  Round {}: vote_b result = {:?}", i, result.as_ref().err());
        }

        let count = eng.misbehavior_registry.offense_count(&[88u8; 32]);
        let should = should_slash(&eng.misbehavior_registry, &[88u8; 32]);
        let suspended = eng
            .validator_status
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .is_suspended(&[88u8; 32], eng.current_height);

        println!("\n  Offense count: {}", count);
        println!("  should_slash:  {}", should);
        println!("  is_suspended:  {}", suspended);
        println!("  Slashing enforcement: {}", if should && suspended { "PASS" } else { "FAIL" });
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
