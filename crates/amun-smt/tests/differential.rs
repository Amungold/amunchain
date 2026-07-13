use amun_smt::*;
use amun_smt::canonical_model::{CanonicalModel, assert_equivalent};

#[test]
fn differential_insert_delete_chain() {
    let mut model = CanonicalModel::new();
    let mut tree = SparseMerkleTree::empty();
    let mut trace = String::new();

    let a = Key256([1u8; 32]);
    let b = Key256([2u8; 32]);
    let c = Key256([3u8; 32]);

    // Insert A
    tree = tree.insert(&a, &[10u8; 32], 1).unwrap();
    model.insert(a, [10u8; 32], 1);
    trace.push_str("insert(A) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    // Insert B
    tree = tree.insert(&b, &[20u8; 32], 1).unwrap();
    model.insert(b, [20u8; 32], 1);
    trace.push_str("insert(B) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    // Insert C
    tree = tree.insert(&c, &[30u8; 32], 1).unwrap();
    model.insert(c, [30u8; 32], 1);
    trace.push_str("insert(C) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    // Delete B
    tree = tree.delete(&b).unwrap();
    model.delete(&b);
    trace.push_str("delete(B) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    // Delete C
    tree = tree.delete(&c).unwrap();
    model.delete(&c);
    trace.push_str("delete(C) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    // Now only A remains
    assert_eq!(model.leaf_count(), 1);
    println!("SUCCESS: Insert A,B,C -> Delete B,C -> Only A remains, roots match");
}

#[test]
fn differential_delete_all_restores_empty() {
    let mut model = CanonicalModel::new();
    let mut tree = SparseMerkleTree::empty();
    let mut trace = String::new();

    let a = Key256([1u8; 32]);
    let b = Key256([2u8; 32]);

    tree = tree.insert(&a, &[10u8; 32], 1).unwrap();
    model.insert(a, [10u8; 32], 1);
    trace.push_str("insert(A) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    tree = tree.insert(&b, &[20u8; 32], 1).unwrap();
    model.insert(b, [20u8; 32], 1);
    trace.push_str("insert(B) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    tree = tree.delete(&a).unwrap();
    model.delete(&a);
    trace.push_str("delete(A) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    tree = tree.delete(&b).unwrap();
    model.delete(&b);
    trace.push_str("delete(B) ");
    assert_equivalent(&tree, &model, &trace).unwrap();

    assert_eq!(model.leaf_count(), 0);
    println!("SUCCESS: Delete all keys restores empty tree");
}
