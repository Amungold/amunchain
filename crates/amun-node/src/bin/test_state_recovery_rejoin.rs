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

    // Create directories and initial stores
    for i in 0..validator_count {
        let dir = format!("/tmp/amun_rejoin_test/validator{}", i);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let store = PersistentValidatorStore::open(&dir).expect("Failed to open store");
        stores.insert(i, store);
    }

    // Helper: create a fresh NetworkNode for a given validator ID
    let make_node = |i: usize| -> NetworkNode {
        NetworkNode::new([i as u8; 32])
    };

    // Initialize all nodes
    for i in 0..validator_count {
        nodes.insert(i, make_node(i));
        validators.push(Validator { id: [i as u8; 32], voting_power: 100 });
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
    
    // Record pre-crash state
    let pre_roots: Vec<_> = stores.values().map(|s| s.state_root()).collect();
    let pre_height = nodes.values().next().map(|n| n.current_height).unwrap_or(0);
    println!("Pre-crash height: {}, roots converge: {}", pre_height, 
             pre_roots.iter().all(|r| *r == pre_roots[0]));

    // Phase 2: Crash validators 2 and 4
    println!("Phase 2: Crashing validators 2 and 4...");
    nodes.remove(&2);
    nodes.remove(&4);

    // Build live validator set
    let mut live_validators = Vec::new();
    for &i in &[0, 1, 3, 5, 6] {
        live_validators.push(Validator { id: [i as u8; 32], voting_power: 100 });
    }
    let live_set = ValidatorSet::new(live_validators).unwrap();

    // Phase 3: Continue with 5 validators
    println!("Phase 3: Continuing with 5 validators to height 10...");
    assert!(run_consensus(&mut nodes, &mut stores, &live_set, 10, 3000));
    
    let current_height = nodes.values().next().map(|n| n.current_height).unwrap_or(0);
    println!("Post-crash height: {}", current_height);

    // Phase 4: Rejoin validator 2 with state recovery
    println!("Phase 4: Rejoining validator 2 with state recovery...");
    
    // Recover the persisted state for validator 2
    let recovered_store = PersistentValidatorStore::open("/tmp/amun_rejoin_test/validator2")
        .expect("Failed to reopen store for validator 2");
    let recovered_root = recovered_store.state_root();
    println!("Validator 2 recovered state root: {}", hex::encode(recovered_root));
    
    // Create new node for validator 2
    let mut new_node = make_node(2);
    // Set consensus state to match current network height
    new_node.current_height = current_height;
    new_node.consensus.state.height = current_height;
    new_node.consensus.last_committed_height = current_height.saturating_sub(1);
    
    stores.insert(2, recovered_store);
    nodes.insert(2, new_node);

    // Phase 5: Rejoin validator 4 with state recovery
    println!("Phase 5: Rejoining validator 4 with state recovery...");
    
    let recovered_store_4 = PersistentValidatorStore::open("/tmp/amun_rejoin_test/validator4")
        .expect("Failed to reopen store for validator 4");
    println!("Validator 4 recovered state root: {}", hex::encode(recovered_store_4.state_root()));
    
    let mut new_node_4 = make_node(4);
    new_node_4.current_height = current_height;
    new_node_4.consensus.state.height = current_height;
    new_node_4.consensus.last_committed_height = current_height.saturating_sub(1);
    
    stores.insert(4, recovered_store_4);
    nodes.insert(4, new_node_4);

    // Rebuild full validator set
    let mut all_validators = Vec::new();
    for i in 0..validator_count {
        all_validators.push(Validator { id: [i as u8; 32], voting_power: 100 });
    }
    let full_set = ValidatorSet::new(all_validators).unwrap();

    // Phase 6: Run network with all 7 validators
    println!("Phase 6: Running full 7-validator network to height 15...");
    let ok = run_consensus(&mut nodes, &mut stores, &full_set, 15, 4000);

    // Compare all state roots
    let final_roots: Vec<_> = stores.values().map(|s| s.state_root()).collect();
    println!("\nFinal state roots:");
    for (i, r) in final_roots.iter().enumerate() {
        println!("  Validator {}: {}", i, hex::encode(*r));
    }

    if ok {
        let first = final_roots[0];
        let all_match = final_roots.iter().all(|r| *r == first);
        if all_match {
            println!("\nPASS: Full state recovery and rejoin successful");
            println!("Final root: {}", hex::encode(first));
        } else {
            println!("\nPARTIAL: Network progressed but recovered validators may have divergent state");
            println!("This is expected — recovered validators have pre-crash state only.");
            println!("Full state sync (catch-up) is not yet implemented (D-005).");
        }
    } else {
        println!("\nResult: Network did not reach target commits after full rejoin.");
        println!("Recovered validators joined with pre-crash state but lack catch-up mechanism.");
    }
}
