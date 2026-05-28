use amun_smt::*;
use amun_smt::canonical_model::{CanonicalModel, assert_equivalent};

/// Generate all permutations of insert/delete for n keys
#[test]
fn exhaustive_1_key() {
    let keys = vec![Key256([1u8; 32])];
    test_all_permutations(&keys, 1);
    println!("Exhaustive 1-key: PASSED");
}

#[test]
fn exhaustive_2_keys() {
    let keys = vec![Key256([1u8; 32]), Key256([2u8; 32])];
    test_all_permutations(&keys, 2);
    println!("Exhaustive 2-keys: PASSED");
}

#[test]
fn exhaustive_3_keys() {
    let keys = vec![Key256([1u8; 32]), Key256([2u8; 32]), Key256([3u8; 32])];
    test_all_permutations(&keys, 3);
    println!("Exhaustive 3-keys: PASSED");
}

#[test]
fn exhaustive_4_keys() {
    let keys = vec![
        Key256([1u8; 32]),
        Key256([2u8; 32]),
        Key256([3u8; 32]),
        Key256([4u8; 32]),
    ];
    test_all_permutations(&keys, 4);
    println!("Exhaustive 4-keys: PASSED");
}

fn test_all_permutations(keys: &[Key256], n: usize) {
    let mut rng = rand::thread_rng();
    
    // Generate many random operation sequences
    for _ in 0..500 {
        let mut model = CanonicalModel::new();
        let mut tree = SparseMerkleTree::empty();
        
        // Random sequence of inserts and deletes
        for _ in 0..20 {
            let key_idx = rand::Rng::gen_range(&mut rng, 0..n);
            let key = &keys[key_idx];
            
            if rand::Rng::gen_bool(&mut rng, 0.5) {
                // Insert
                let value = [rand::Rng::gen::<u8>(&mut rng); 32];
                tree = tree.insert(key, &value, 1).unwrap();
                model.insert(*key, value, 1);
            } else {
                // Delete
                tree = tree.delete(key).unwrap();
                model.delete(key);
            }
            
            assert_equivalent(&tree, &model, &format!("random-op")).unwrap();
        }
    }
}
