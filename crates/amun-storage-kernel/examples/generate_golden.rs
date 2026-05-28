use amun_storage_kernel::{Key256, SparseMerkleTree};
use std::fs;

fn main() {
    let key = Key256([1u8; 32]);
    let value = [42u8; 32];
    let tree = SparseMerkleTree::empty().insert(&key, &value, 0);
    let root = tree.root();
    let proof = tree.generate_inclusion_proof(&key).unwrap();

    fs::write("fixtures/root_v1.bin", root.0).unwrap();
    fs::write("fixtures/proof_v1.bin", proof.encode()).unwrap();
    println!("Golden vectors generated");
}
