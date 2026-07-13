use amun_smt::*;
use amun_smt::canonical_model::{CanonicalModel, assert_equivalent};

/// Law 1: InsertDeleteIdentity
/// T.insert(k,v).delete(k) == T
#[test]
fn law_insert_delete_identity() {
    let k = Key256([42u8; 32]);
    let v = [99u8; 32];
    
    let t = SparseMerkleTree::empty();
    let t1 = t.insert(&k, &v, 1).unwrap();
    let t2 = t1.delete(&k).unwrap();
    
    assert_eq!(t2.root(), SparseMerkleTree::empty().root());
    assert_eq!(t2.internal_root(), SparseMerkleTree::empty().internal_root());
    
    println!("InsertDeleteIdentity: PASSED");
}

/// Law 2: DeleteInsertCanonicality
/// delete(k) + insert(k,v) == canonical rebuild with {k,v}
#[test]
fn law_delete_insert_canonicality() {
    let k = Key256([42u8; 32]);
    let v = [99u8; 32];
    
    // Start with tree containing k
    let mut model = CanonicalModel::new();
    let mut tree = SparseMerkleTree::empty();
    
    tree = tree.insert(&k, &v, 1).unwrap();
    model.insert(k, v, 1);
    
    // Delete then re-insert
    tree = tree.delete(&k).unwrap();
    model.delete(&k);
    tree = tree.insert(&k, &v, 1).unwrap();
    model.insert(k, v, 1);
    
    // Should match canonical rebuild
    assert_equivalent(&tree, &model, "delete-insert").unwrap();
    
    println!("DeleteInsertCanonicality: PASSED");
}

/// Law 3: StructuralDeterminism
/// Same leaf set => same root (already tested in determinism tests)
#[test]
fn law_structural_determinism() {
    let k1 = Key256([1u8; 32]);
    let k2 = Key256([2u8; 32]);
    
    let t1 = SparseMerkleTree::empty()
        .insert(&k1, &[10u8; 32], 1).unwrap()
        .insert(&k2, &[20u8; 32], 1).unwrap();
    
    let t2 = SparseMerkleTree::empty()
        .insert(&k2, &[20u8; 32], 1).unwrap()
        .insert(&k1, &[10u8; 32], 1).unwrap();
    
    assert_eq!(t1.root(), t2.root());
    
    println!("StructuralDeterminism: PASSED");
}
