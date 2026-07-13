use amun_resource_core::ResourceId;
use amun_validator_runtime::validator_node::ValidatorNode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_live_state <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    let mut roots = Vec::new();

    for i in 0..4 {
        let dir = format!("{}/validator{}", prefix, i);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let node_id = ResourceId([i as u8; 32]);
        let mut node = ValidatorNode::new(node_id, &dir).expect("Failed to create node");
        node.propose_block(1).expect("Failed to propose block");
        let root = node.store.state_root();
        println!("Validator {}: {}", i, hex::encode(root));
        roots.push(root);
    }

    let first = roots[0];
    let all_match = roots.iter().all(|r| *r == first);
    if all_match {
        println!(
            "PASS: All validators have state_root = {}",
            hex::encode(first)
        );
    } else {
        println!("FAIL: Mismatch detected");
        std::process::exit(1);
    }
}
