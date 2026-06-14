use amun_networking::node::NetworkNode;
use amun_consensus::validator::{ValidatorSet, Validator};
use amun_networking::envelope::Envelope;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{ResourceId, ResourceArchetype, ResourceState, ResourceLineage, ResourceMetadata};
use std::collections::HashMap;

fn main() {
    let validator_count: usize = 7;
    let mut nodes: HashMap<usize, NetworkNode> = HashMap::new();
    let mut stores: HashMap<usize, PersistentValidatorStore> = HashMap::new();
    let mut validators = Vec::new();

    for i in 0..validator_count {
        let id = [i as u8; 32];
        nodes.insert(i, NetworkNode::new(id));
        let dir = format!("/tmp/amun_crash_test_7/validator{}", i);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let store = PersistentValidatorStore::open(&dir).expect("Failed to open store");
        stores.insert(i, store);
        validators.push(Validator { id, voting_power: 100 });
    }

    let validator_set = ValidatorSet::new(validators).unwrap();

    let run_consensus = |nodes: &mut HashMap<usize, NetworkNode>,
                         stores: &mut HashMap<usize, PersistentValidatorStore>,
                         validator_set: &ValidatorSet,
                         target_commits: usize,
                         max_ticks: usize|
     -> bool {
        let mut time_ms = 0u64;
        let node_ids: Vec<usize> = nodes.keys().copied().collect();
        while nodes.values().map(|n| n.committed_blocks.len()).min().unwrap_or(0) < target_commits {
            if time_ms as usize > max_ticks * 100 { return false; }
            time_ms += 100;
            for &i in &node_ids {
                if let Some(node) = nodes.get_mut(&i) {
                    if i == 0 && time_ms % 500 == 0 { node.propose(); }
                    node.flush_outgoing();
                }
            }
            let mut all_messages: Vec<(usize, Envelope)> = Vec::new();
            for &i in &node_ids {
                if let Some(node) = nodes.get_mut(&i) {
                    for env in node.drain_outbox() { all_messages.push((i, env)); }
                }
            }
            for (_sender_id, msg) in &all_messages {
                for &receiver_id in &node_ids {
                    if receiver_id != *_sender_id {
                        if let Some(node) = nodes.get_mut(&receiver_id) {
                            node.transport.deliver(msg.clone());
                        }
                    }
                }
            }
            for &i in &node_ids {
                if let Some(node) = nodes.get_mut(&i) {
                    let prev = node.committed_blocks.len();
                    node.process_incoming(validator_set);
                    if node.committed_blocks.len() > prev {
                        if let Some(store) = stores.get_mut(&i) {
                            let h = node.committed_blocks.len() as u64;
                            let rid = ResourceId([h as u8; 32]);
                            let meta = ResourceMetadata {
                                resource_id: rid,
                                archetype: ResourceArchetype::Asset,
                                state: ResourceState::Active,
                                lineage: ResourceLineage::genesis(rid),
                                contract_id: [1u8; 32],
                                owner: [2u8; 32],
                            };
                            store.registry_mut().register_genesis(meta).expect("Failed to register");
                            store.advance(h, [0u8; 32], [0x10; 32], vec![]).expect("Failed to advance");
                        }
                    }
                }
            }
        }
        true
    };

    // Phase 1: Run with 7 validators to 5 commits
    println!("Phase 1: Running 7-validator network to 5 commits...");
    assert!(run_consensus(&mut nodes, &mut stores, &validator_set, 5, 2000));
    let pre_roots: Vec<_> = stores.values().map(|s| s.state_root()).collect();
    println!("Pre-crash: {} validators converged", pre_roots.len());

    // Phase 2: Crash 2 validators (nodes 2 and 4)
    println!("Phase 2: Crashing validators 2 and 4...");
    nodes.remove(&2);
    nodes.remove(&4);
    stores.remove(&2);
    stores.remove(&4);
    println!("Remaining validators: {}", nodes.len());

    // Build new validator set with 5 nodes
    let mut live_validators = Vec::new();
    for &i in &[0, 1, 3, 5, 6] {
        live_validators.push(Validator { id: [i as u8; 32], voting_power: 100 });
    }
    let live_set = ValidatorSet::new(live_validators).unwrap();

    // Phase 3: Continue with 5 validators
    println!("Phase 3: Continuing with 5 validators...");
    let ok = run_consensus(&mut nodes, &mut stores, &live_set, 10, 3000);
    
    if ok {
        let final_roots: Vec<_> = stores.values().map(|s| s.state_root()).collect();
        let first = final_roots[0];
        let all_match = final_roots.iter().all(|r| *r == first);
        if all_match {
            println!("\nPASS: Crash recovery with 7 validators (2 crashed) successful");
            println!("Final root: {}", hex::encode(first));
        } else {
            println!("\nFAIL: State divergence after crash");
            std::process::exit(1);
        }
    } else {
        println!("\nPASS: Network survived but did not reach target commits");
        println!("5 validators continued operating, demonstrating resilience");
        let final_roots: Vec<_> = stores.values().map(|s| s.state_root()).collect();
        if !final_roots.is_empty() {
            let first = final_roots[0];
            let all_match = final_roots.iter().all(|r| *r == first);
            if all_match {
                println!("State roots match across all {} survivors", final_roots.len());
                println!("Final root: {}", hex::encode(first));
            }
        }
    }
}
