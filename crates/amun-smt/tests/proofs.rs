use amun_smt::*;
use rand::Rng;

fn random_key(rng: &mut impl Rng) -> Key256 {
    let mut a = [0u8; 32];
    rng.fill(&mut a);
    Key256(a)
}

#[test]
fn inclusion_proof_roundtrip() {
    let mut rng = rand::thread_rng();
    let k = random_key(&mut rng);
    let v = [99u8; 32];
    let t = SparseMerkleTree::empty()
        .insert(&k, &v, 2)
        .unwrap();
    let root = t.root();

    let proof = t
        .generate_inclusion_proof(&k)
        .unwrap()
        .expect("Proof must exist");
    assert!(proof.verify(&root.0).unwrap(), "Proof must verify");
}

#[test]
fn absence_proof_roundtrip() {
    let mut rng = rand::thread_rng();
    let k1 = random_key(&mut rng);
    let k2 = random_key(&mut rng);
    let t = SparseMerkleTree::empty()
        .insert(&k1, &[1u8; 32], 0)
        .unwrap();
    let root = t.root();

    let proof = t
        .generate_absence_proof(&k2)
        .unwrap()
        .expect("Proof must exist");
    assert!(proof.verify(&root.0).unwrap(), "Absence proof must verify");
}

#[test]
fn empty_tree_absence() {
    let t = SparseMerkleTree::empty();
    let root = t.root();
    let key = Key256([7u8; 32]);

    let proof = t
        .generate_absence_proof(&key)
        .unwrap()
        .expect("Proof must exist");
    assert!(
        proof.verify(&root.0).unwrap(),
        "Empty tree absence must verify"
    );
}
