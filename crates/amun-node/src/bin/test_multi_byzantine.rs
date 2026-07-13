use amun_consensus::validator::{Validator, ValidatorSet};
use amun_networking::envelope::Envelope;
use amun_networking::node::NetworkNode;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use std::collections::HashMap;

fn main() {
    let validator_count: usize = 7;
    let byzantine_ids: Vec<usize> = vec![2, 3];
    let mut nodes: HashMap<usize, NetworkNode> = HashMap::new();
    let mut stores: HashMap<usize, PersistentValidatorStore> = HashMap::new();
    let mut validators = Vec::new();

    for i in 0..validator_count {
        let dir = format!("/tmp/amun_multi_byzantine_test/validator{}", i);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let store = PersistentValidatorStore::open(&dir).expect("Failed to open store");
        stores.insert(i, store);
    }

    for i in 0..validator_count {
        nodes.insert(i, NetworkNode::new([i as u8; 32]));
        validators.push(Validator {
            id: [i as u8; 32],
            voting_power: 100,
        });
    }

    let validator_set = ValidatorSet::new(validators).unwrap();

    let run_consensus = |nodes: &mut HashMap<usize, NetworkNode>,
                         stores: &mut HashMap<usize, PersistentValidatorStore>,
                         validator_set: &ValidatorSet,
                         target_commits: usize,
                         max_ticks: usize,
                         byzantine: &[usize]|
     -> bool {
        let mut time_ms = 0u64;
        let node_ids: Vec<usize> = nodes.keys().copied().collect();
        let mut byzantine_acted = false;
        while nodes
            .values()
            .map(|n| n.committed_blocks.len())
            .min()
            .unwrap_or(0)
            < target_commits
        {
            if time_ms as usize > max_ticks * 100 {
                return false;
            }
            time_ms += 100;
            for &i in &node_ids {
                if let Some(node) = nodes.get_mut(&i) {
                    if i == 0 && time_ms.is_multiple_of(500) {
                        node.propose();
                    }
                    node.flush_outgoing();
                }
            }
            let mut all_messages: Vec<(usize, Envelope)> = Vec::new();
            for &i in &node_ids {
                if let Some(node) = nodes.get_mut(&i) {
                    for env in node.drain_outbox() {
                        all_messages.push((i, env));
                    }
                }
            }
            // Byzantine nodes send conflicting proposals
            if time_ms >= 1000 && !byzantine_acted {
                for &b_id in byzantine {
                    for &receiver_id in &node_ids {
                        if receiver_id != b_id {
                            let env = Envelope {
                                sender: hex::encode([b_id as u8; 32]),
                                recipient: hex::encode([receiver_id as u8; 32]),
                                sequence: time_ms + b_id as u64,
                                timestamp: time_ms,
                                message_type: "proposal".into(),
                                payload: vec![0xFF; 32].into(),
                            };
                            all_messages.push((b_id, env));
                        }
                    }
                }
                byzantine_acted = true;
                println!("Byzantine nodes {:?} sent conflicting proposals", byzantine);
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
                            store
                                .registry_mut()
                                .register_genesis(meta)
                                .expect("Failed to register");
                            store
                                .advance(h, [0u8; 32], [0x10; 32], vec![])
                                .expect("Failed to advance");
                        }
                    }
                }
            }
        }
        true
    };

    println!(
        "Testing with {} Byzantine validators out of {}...",
        byzantine_ids.len(),
        validator_count
    );
    let ok = run_consensus(
        &mut nodes,
        &mut stores,
        &validator_set,
        5,
        3000,
        &byzantine_ids,
    );

    // Honest validators are all except the Byzantine ones
    let honest_ids: Vec<usize> = (0..validator_count)
        .filter(|i| !byzantine_ids.contains(i))
        .collect();
    let honest_roots: Vec<_> = honest_ids
        .iter()
        .map(|&i| stores[&i].state_root())
        .collect();
    let first = honest_roots[0];
    let all_match = honest_roots.iter().all(|r| *r == first);

    println!("Consensus reached: {}", ok);
    println!("Honest validators: {:?}", honest_ids);
    println!(
        "Honest roots: {:?}",
        honest_roots.iter().map(hex::encode).collect::<Vec<_>>()
    );
    println!("All honest converge: {}", all_match);

    if ok && all_match {
        println!(
            "\nPASS: Network tolerates {} Byzantine validators",
            byzantine_ids.len()
        );
        println!("Honest validators maintained state convergence");
    } else if all_match {
        println!("\nPARTIAL: Honest validators converge but commit rate affected");
    } else {
        println!("\nFAIL: State divergence among honest validators");
    }
}
