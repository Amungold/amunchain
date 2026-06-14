use amun_networking::node::NetworkNode;
use amun_consensus::validator::{ValidatorSet, Validator};
use amun_networking::envelope::Envelope;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{ResourceId, ResourceArchetype, ResourceState, ResourceLineage, ResourceMetadata};
use amun_validator_networking::rejoin_protocol::{RejoinProtocol, RejoinResult};
use amun_validator_networking::sync_transport::SyncTransport;
use std::collections::HashMap;

fn main() {
    let validator_count: usize = 7;
    let mut nodes: HashMap<usize, NetworkNode> = HashMap::new();
    let mut stores: HashMap<usize, PersistentValidatorStore> = HashMap::new();
    let mut validators = Vec::new();

    for i in 0..validator_count {
        let dir = format!("/tmp/amun_post_rejoin_test/validator{}", i);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let store = PersistentValidatorStore::open(&dir).expect("Failed to open store");
        stores.insert(i, store);
    }

    for i in 0..validator_count {
        nodes.insert(i, NetworkNode::new([i as u8; 32]));
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
                                resource_id: rid, archetype: ResourceArchetype::Asset,
                                state: ResourceState::Active, lineage: ResourceLineage::genesis(rid),
                                contract_id: [1u8; 32], owner: [2u8; 32],
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

    // Phase 1: Run to height 5
    println!("Phase 1: Running 7-validator network to height 5...");
    assert!(run_consensus(&mut nodes, &mut stores, &validator_set, 5, 2000));

    // Phase 2: Crash 2 and 4
    println!("Phase 2: Crashing validators 2 and 4...");
    nodes.remove(&2); nodes.remove(&4);

    let mut live_validators = Vec::new();
    for &i in &[0, 1, 3, 5, 6] {
        live_validators.push(Validator { id: [i as u8; 32], voting_power: 100 });
    }
    let live_set = ValidatorSet::new(live_validators).unwrap();

    // Phase 3: Continue to height 10
    println!("Phase 3: Continuing with 5 validators to height 10...");
    assert!(run_consensus(&mut nodes, &mut stores, &live_set, 10, 3000));
    let live_root = stores[&0].state_root();
    println!("Live validators root at height 10: {}", hex::encode(live_root));

    // Phase 4: Rejoin 2 and 4 with RejoinProtocol
    println!("Phase 4: Rejoining validators 2 and 4...");
    let peer_registry = stores[&0].registry().clone();
    let current_height = 10u64;
    let block_hash = [current_height as u8; 32];
    let trusted_root = [0x10; 32];

    for &v in &[2, 4] {
        let result = RejoinProtocol::rejoin(
            &peer_registry, current_height, block_hash, trusted_root, trusted_root,
        );
        match result {
            RejoinResult::Rejoined { height, resources_imported } => {
                // Apply the recovered state to the validator's store
                let recovered_store = stores.get_mut(&v).unwrap();
                let package = SyncTransport::export_snapshot(
                    &peer_registry, current_height, block_hash, trusted_root, "rejoin".into(),
                );
                let imported = SyncTransport::import_snapshot(&package, trusted_root).unwrap();
                *recovered_store.registry_mut() = imported;
                recovered_store.advance(current_height, [0u8; 32], [0x10; 32], vec![]).unwrap();
                
                let root = recovered_store.state_root();
                println!("Validator {} rejoined: height={}, resources={}, root={}",
                         v, height, resources_imported, hex::encode(root));
            }
            RejoinResult::Failed { reason } => {
                println!("Validator {} rejoin failed: {}", v, reason);
            }
        }
    }

    // Verify convergence immediately after rejoin
    println!("\nPhase 5: Verifying state convergence after rejoin...");
    let _post_rejoin_roots: Vec<_> = (0..7).map(|i| {
        if stores.contains_key(&i) {
            Some(stores[&i].state_root())
        } else {
            None
        }
    }).collect();
    
    let live_roots: Vec<_> = [0,1,3,5,6].iter().map(|&i| stores[&i].state_root()).collect();
    let recovered_roots: Vec<_> = [2,4].iter().map(|&i| stores[&i].state_root()).collect();
    
    println!("Live validators: {:?}", live_roots.iter().map(hex::encode).collect::<Vec<_>>());
    println!("Recovered validators: {:?}", recovered_roots.iter().map(hex::encode).collect::<Vec<_>>());
    
    let all_match = live_roots.iter().chain(recovered_roots.iter()).all(|r| *r == live_roots[0]);
    if all_match {
        println!("PASS: All validators converged after rejoin");
    } else {
        println!("FAIL: State divergence after rejoin");
    }
}
