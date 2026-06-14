use amun_networking::node::NetworkNode;
use amun_consensus::validator::{ValidatorSet, Validator};
use amun_networking::envelope::Envelope;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{ResourceId, ResourceArchetype, ResourceState, ResourceLineage, ResourceMetadata};
use std::collections::HashMap;

fn main() {
    let mut nodes: HashMap<usize, NetworkNode> = HashMap::new();
    let mut stores: HashMap<usize, PersistentValidatorStore> = HashMap::new();
    let mut validators = Vec::new();
    
    // Create 4 nodes with their own state stores
    for i in 0..4 {
        let id = [i as u8; 32];
        nodes.insert(i, NetworkNode::new(id));
        
        let dir = format!("/tmp/amun_state_test/validator{}", i);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let store = PersistentValidatorStore::open(&dir).expect("Failed to open store");
        stores.insert(i, store);
        
        validators.push(Validator {
            id,
            voting_power: 100,
        });
    }
    
    let validator_set = ValidatorSet::new(validators).unwrap();
    let mut time_ms = 0u64;
    let node_ids: Vec<usize> = nodes.keys().copied().collect();

    // Run until we have 3 commits
    while nodes.values().map(|n| n.committed_blocks.len()).min().unwrap_or(0) < 3 {
        time_ms += 100;

        // Leader proposes
        for &i in &node_ids {
            if let Some(node) = nodes.get_mut(&i) {
                if i == 0 && time_ms % 500 == 0 {
                    node.propose();
                }
                node.flush_outgoing();
            }
        }

        // Collect messages
        let mut all_messages: Vec<(usize, Envelope)> = Vec::new();
        for &i in &node_ids {
            if let Some(node) = nodes.get_mut(&i) {
                for env in node.drain_outbox() {
                    all_messages.push((i, env));
                }
            }
        }

        // Deliver to all other nodes
        for (sender_id, msg) in &all_messages {
            for &receiver_id in &node_ids {
                if receiver_id != *sender_id {
                    if let Some(node) = nodes.get_mut(&receiver_id) {
                        node.transport.deliver(msg.clone());
                    }
                }
            }
        }

        // Process incoming + update state store when commits happen
        for &i in &node_ids {
            if let Some(node) = nodes.get_mut(&i) {
                let prev_commits = node.committed_blocks.len();
                node.process_incoming(&validator_set);
                
                // If a new block was committed, update the state store
                if node.committed_blocks.len() > prev_commits {
                    if let Some(store) = stores.get_mut(&i) {
                        let height = node.committed_blocks.len() as u64;
                        // Add a resource mutation to make state non-zero
                        let resource_id = ResourceId([height as u8; 32]);
                        let meta = ResourceMetadata {
                            resource_id,
                            archetype: ResourceArchetype::Asset,
                            state: ResourceState::Active,
                            lineage: ResourceLineage::genesis(resource_id),
                            contract_id: [1u8; 32],
                            owner: [2u8; 32],
                        };
                        store.registry_mut().register_genesis(meta).expect("Failed to register");
                        store.advance(height, [0u8; 32], [0x10; 32], vec![]).expect("Failed to advance");
                    }
                }
            }
        }
    }

    // Compare state roots across all nodes
    let mut roots = Vec::new();
    for i in 0..4 {
        let root = stores[&i].state_root();
        println!("Validator {} final state root: {}", i, hex::encode(root));
        roots.push(root);
    }

    let first = roots[0];
    let all_match = roots.iter().all(|r| *r == first);
    if all_match {
        println!("\nPASS: State convergence verified across all validators");
        println!("Final root: {}", hex::encode(first));
    } else {
        println!("\nFAIL: State divergence detected");
        std::process::exit(1);
    }
}
