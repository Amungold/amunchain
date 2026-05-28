use amun_smt::*;
use rand::Rng;

#[test]
fn validate_empty_tree() {
    let t = SparseMerkleTree::empty();
    validate_tree(&t.internal_root(), t.context()).unwrap();
}

#[test]
fn validate_after_inserts() {
    let mut rng = rand::thread_rng();
    let mut t = SparseMerkleTree::empty();
    for i in 0..30u8 {
        let key = Key256([i; 32]);
        t = t.insert(&key, &[i; 32], i as u64).unwrap();
    }
    validate_tree(&t.internal_root(), t.context()).unwrap();
}

#[test]
fn validate_after_delete() {
    let k = Key256([42u8; 32]);
    let t = SparseMerkleTree::empty()
        .insert(&k, &[1u8; 32], 0)
        .unwrap()
        .delete(&k)
        .unwrap();
    validate_tree(&t.internal_root(), t.context()).unwrap();
}
