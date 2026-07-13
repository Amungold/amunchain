use amun_consensus::validator::{Validator, ValidatorSet};
use amun_networking::envelope::Envelope;
use amun_networking::node::NetworkNode;
use amun_persistence::PersistentState;
use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use amun_snapshot_engine_unified::SnapshotManager;
use amun_validator_networking::rejoin_protocol::{RejoinProtocol, RejoinResult};
use amun_validator_networking::sync_transport::SyncTransport;
use std::collections::HashMap;

fn main() {
    let validator_count: usize = 7;
    let mut nodes: HashMap<usize, NetworkNode> = HashMap::new();
    let mut stores: HashMap<usize, PersistentValidatorStore> = HashMap::new();
    let mut validators = Vec::new();

    for i in 0..validator_count {
        let dir = format!("/tmp/amun_checkpoint_test/validator{}", i);
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
                         max_ticks: usize|
     -> bool {
        let mut time_ms = 0u64;
        let node_ids: Vec<usize> = nodes.keys().copied().collect();
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

    // Phase 1: Run to height 5, create snapshots
    println!("Phase 1: Running network to height 5...");
    assert!(run_consensus(
        &mut nodes,
        &mut stores,
        &validator_set,
        5,
        2000
    ));

    for i in 0..validator_count {
        let snapshot_path = format!("/tmp/amun_checkpoint_test/snapshot_v{}.json", i);
        let manager = SnapshotManager::new(&snapshot_path);
        let root = stores[&i].state_root();
        let state = PersistentState {
            height: 5,
            state_root: hex::encode(root),
            evidence_root: hex::encode([0u8; 32]),
            block_hash: hex::encode([5u8; 32]),
            last_commit_hash: hex::encode([0u8; 32]),
        };
        manager.create(&state).expect("Failed to create snapshot");
    }
    println!("Snapshots created at height 5");

    // Phase 2: Crash 2 validators
    println!("Phase 2: Crashing validators 2 and 4...");
    nodes.remove(&2);
    nodes.remove(&4);
    let mut live_validators = Vec::new();
    for &i in &[0, 1, 3, 5, 6] {
        live_validators.push(Validator {
            id: [i as u8; 32],
            voting_power: 100,
        });
    }
    let live_set = ValidatorSet::new(live_validators).unwrap();

    // Phase 3: Continue to height 10
    println!("Phase 3: Continuing to height 10...");
    assert!(run_consensus(&mut nodes, &mut stores, &live_set, 10, 3000));

    // Phase 4: Recover via snapshots + RejoinProtocol
    println!("Phase 4: Recovering validators 2 and 4...");
    for &v in &[2, 4] {
        let snapshot_path = format!("/tmp/amun_checkpoint_test/snapshot_v{}.json", v);
        let manager = SnapshotManager::new(&snapshot_path);
        let _snapshot = manager.load().expect("Failed to load snapshot");

        let peer_registry = stores[&0].registry().clone();
        let result = RejoinProtocol::rejoin(&peer_registry, 10, [10u8; 32], [0x10; 32], [0x10; 32]);
        match result {
            RejoinResult::Rejoined {
                height,
                resources_imported,
            } => {
                let package = SyncTransport::export_snapshot(
                    &peer_registry,
                    height,
                    [10u8; 32],
                    [0x10; 32],
                    "rejoin".into(),
                );
                let imported = SyncTransport::import_snapshot(&package, [0x10; 32]).unwrap();
                let store = stores.get_mut(&v).unwrap();
                *store.registry_mut() = imported;
                store
                    .advance(height, [0u8; 32], [0x10; 32], vec![])
                    .unwrap();
                let root = store.state_root();
                println!(
                    "Validator {} recovered: height={}, resources={}, root={}",
                    v,
                    height,
                    resources_imported,
                    hex::encode(root)
                );
            }
            RejoinResult::Failed { reason } => {
                println!("Validator {} rejoin failed: {}", v, reason);
            }
        }
    }

    // Verify convergence
    let all_roots: Vec<_> = (0..7).map(|i| stores[&i].state_root()).collect();
    let first = all_roots[0];
    let all_match = all_roots.iter().all(|r| *r == first);

    println!("\nAll validator roots:");
    for (i, r) in all_roots.iter().enumerate() {
        println!("  Validator {}: {}", i, hex::encode(*r));
    }

    if all_match {
        println!("\nPASS: Checkpoint-based recovery and rejoin successful");
        println!("Final root: {}", hex::encode(first));
    } else {
        println!("\nFAIL: State divergence after checkpoint recovery");
    }
}
