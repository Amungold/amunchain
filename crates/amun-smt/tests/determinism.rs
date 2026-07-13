use amun_smt::*;
use rand::Rng;
use rand::seq::SliceRandom;

fn random_key(rng: &mut impl Rng) -> Key256 {
    let mut a = [0u8; 32];
    rng.fill(&mut a);
    Key256(a)
}

#[test]
fn insertion_order_independence() {
    let n = 40;
    let mut rng = rand::thread_rng();
    let keys: Vec<_> = (0..n).map(|_| random_key(&mut rng)).collect();
    let vals: Vec<_> = (0..n).map(|i| [i as u8; 32]).collect();

    // Random order
    let mut order: Vec<usize> = (0..n).collect();
    order.shuffle(&mut rng);

    let mut t1 = SparseMerkleTree::empty();
    for &i in &order {
        t1 = t1.insert(&keys[i], &vals[i], 0).unwrap();
    }
    let root1 = t1.root();

    // Sorted order
    let mut sorted_keys = keys.clone();
    sorted_keys.sort_by_key(|k| k.0);
    let mut t2 = SparseMerkleTree::empty();
    for k in &sorted_keys {
        let idx = keys.iter().position(|x| x == k).unwrap();
        t2 = t2.insert(k, &vals[idx], 0).unwrap();
    }
    let root2 = t2.root();

    assert_eq!(root1.0, root2.0, "Roots must be equal regardless of insertion order");
}

#[test]
fn delete_stability() {
    let mut rng = rand::thread_rng();
    let k = random_key(&mut rng);
    let v = [42u8; 32];

    let t1 = SparseMerkleTree::empty()
        .insert(&k, &v, 1)
        .unwrap();
    let t2 = t1.delete(&k).unwrap();

    assert_eq!(
        t2.root().0,
        SparseMerkleTree::empty().root().0,
        "Delete must restore empty root"
    );
}
