use amun_kernel::hashing::domain_tags;
use amun_state_root::proofs::MerkleProof;
use sha2::{Digest, Sha256};

#[test]
fn test_merkle_proof_verify_valid() {
    let leaf = vec![1u8, 2, 3];
    let mut hasher = Sha256::new();
    hasher.update(domain_tags::MERKLE_LEAF);
    hasher.update(&leaf);
    let leaf_hash: [u8; 32] = hasher.finalize().into();
    let proof = MerkleProof { leaf, path: vec![] };
    assert!(proof.verify(&leaf_hash));
}

#[test]
fn test_merkle_proof_rejects_wrong_root() {
    let proof = MerkleProof {
        leaf: vec![1u8],
        path: vec![],
    };
    assert!(!proof.verify(&[0xFF; 32]));
}

#[test]
fn test_merkle_proof_rejects_sibling_swap() {
    let left = vec![1u8];
    let right = vec![2u8];

    let mut hasher = Sha256::new();
    hasher.update(domain_tags::MERKLE_LEAF);
    hasher.update(&left);
    let left_hash: [u8; 32] = hasher.finalize().into();
    let mut hasher = Sha256::new();
    hasher.update(domain_tags::MERKLE_LEAF);
    hasher.update(&right);
    let right_hash: [u8; 32] = hasher.finalize().into();

    let mut hasher = Sha256::new();
    hasher.update(domain_tags::MERKLE_NODE);
    hasher.update(left_hash);
    hasher.update(right_hash);
    let root: [u8; 32] = hasher.finalize().into();

    let proof = MerkleProof {
        leaf: left.clone(),
        path: vec![(true, right_hash)],
    };
    assert!(proof.verify(&root));

    let forged = MerkleProof {
        leaf: left,
        path: vec![(false, right_hash)],
    };
    assert!(!forged.verify(&root));
}

#[test]
fn test_merkle_proof_rejects_leaf_node_confusion() {
    let data = vec![1u8, 2, 3];
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let raw_hash: [u8; 32] = hasher.finalize().into();
    let proof = MerkleProof {
        leaf: data,
        path: vec![],
    };
    assert!(
        !proof.verify(&raw_hash),
        "Domain separation must prevent leaf/node confusion"
    );
}
