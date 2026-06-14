use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_replay_determinism <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    const HEIGHTS: u64 = 3;
    let mut mutations_log: Vec<Vec<ResourceMetadata>> = Vec::new();

    // Phase 1: Build reference state and log mutations
    let ref_dir = format!("{}/reference", prefix);
    std::fs::create_dir_all(&ref_dir).expect("Failed to create dir");
    let mut ref_store = PersistentValidatorStore::open(&ref_dir).expect("Failed to open store");

    for h in 1..=HEIGHTS {
        let mut height_mutations = Vec::new();
        for j in 0..h {
            let id = ResourceId([h as u8, j as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            let meta = ResourceMetadata {
                resource_id: id,
                archetype: ResourceArchetype::Asset,
                state: ResourceState::Active,
                lineage: ResourceLineage::genesis(id),
                contract_id: [1u8; 32],
                owner: [2u8; 32],
            };
            ref_store.registry_mut().register_genesis(meta.clone()).expect("Failed to register");
            height_mutations.push(meta);
        }
        ref_store.advance(h, [0u8; 32], [0x10; 32], vec![]).expect("Failed to advance");
        mutations_log.push(height_mutations);
    }
    let ref_root = ref_store.state_root();
    println!("Reference root: {}", hex::encode(ref_root));
    drop(ref_store);

    // Phase 2: Replay on 4 independent stores
    let mut replay_roots = Vec::new();
    for v in 0..4 {
        let replay_dir = format!("{}/replay{}", prefix, v);
        std::fs::create_dir_all(&replay_dir).expect("Failed to create dir");
        let mut replay_store = PersistentValidatorStore::open(&replay_dir).expect("Failed to open store");

        for (h, height_mutations) in mutations_log.iter().enumerate() {
            for meta in height_mutations {
                replay_store.registry_mut().register_genesis(meta.clone()).expect("Failed to register");
            }
            replay_store.advance((h + 1) as u64, [0u8; 32], [0x10; 32], vec![]).expect("Failed to advance");
        }
        let replay_root = replay_store.state_root();
        println!("Replay {} root: {}", v, hex::encode(replay_root));
        replay_roots.push(replay_root);
    }

    // Phase 3: Verify all replay roots match reference
    let all_match = replay_roots.iter().all(|r| *r == ref_root);
    if all_match {
        println!("\nPASS: Replay determinism verified across all validators");
        println!("Reference root: {}", hex::encode(ref_root));
    } else {
        println!("\nFAIL: Replay determinism violation");
        for (i, r) in replay_roots.iter().enumerate() {
            println!("Replay {} root: {} (match: {})", i, hex::encode(*r), *r == ref_root);
        }
        std::process::exit(1);
    }
}
