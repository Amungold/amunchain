use amun_persistent_node::persistent_store::PersistentValidatorStore;
use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_state_evolution <data_directory>");
        std::process::exit(1);
    }
    let path = Path::new(&args[1]);
    let mut store =
        PersistentValidatorStore::open(path.to_str().unwrap()).expect("Failed to open store");

    // 1. Get initial state root (should be zero)
    let root_before = store.state_root();
    println!("State Root (empty): {}", hex::encode(root_before));

    // 2. Add 3 resources
    for i in 0..3 {
        let id = ResourceId([i as u8; 32]);
        let meta = ResourceMetadata {
            resource_id: id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(id),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        };
        store
            .registry_mut()
            .register_genesis(meta)
            .expect("Failed to register");
    }

    // 3. Get state root after adding resources
    let root_after = store.state_root();
    println!(
        "State Root (after 3 resources): {}",
        hex::encode(root_after)
    );

    // 4. Verify the root changed
    if root_before == root_after {
        println!("FAIL: State root did not change after adding resources");
        std::process::exit(1);
    } else {
        println!("PASS: State root evolved correctly");
    }
}
