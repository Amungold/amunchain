use amun_resource_core::ResourceId;
use amun_validator_runtime::validator_node::ValidatorNode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: test_validator_cluster_determinism <temp_dir_prefix>");
        std::process::exit(1);
    }
    let prefix = &args[1];
    const BLOCKS: u64 = 3;
    let mut all_roots = Vec::new();

    for v in 0..4 {
        let dir = format!("{}/validator{}", prefix, v);
        std::fs::create_dir_all(&dir).expect("Failed to create dir");
        let node_id = ResourceId([v as u8; 32]);
        let mut node = ValidatorNode::new(node_id, &dir).expect("Failed to create node");

        let mut heights_roots = Vec::new();
        for h in 1..=BLOCKS {
            node.propose_block(h).expect("Failed to propose block");
            let root = node.store.state_root();
            heights_roots.push((h, root));
            println!("Validator {} | Height {} root: {}", v, h, hex::encode(root));
        }
        all_roots.push(heights_roots);
    }

    // Verify all validators have identical state roots at each height
    let first_validator = &all_roots[0];
    for (h_idx, (height, expected_root)) in first_validator.iter().enumerate() {
        for v in 1..4 {
            let (_, validator_root) = all_roots[v][h_idx];
            if validator_root != *expected_root {
                println!(
                    "FAIL: Validator {} height {} root diverges",
                    v, height
                );
                std::process::exit(1);
            }
        }
    }

    let final_root = first_validator.last().unwrap().1;
    println!("\nPASS: Validator cluster determinism verified");
    println!("Final root after {} blocks: {}", BLOCKS, hex::encode(final_root));
}
