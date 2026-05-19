#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::proof::Direction;
    use crate::*;

    #[test]
    fn test_empty_root_freeze() {
        let root = MerkleTree::empty_root();
        assert_eq!(root.as_bytes().len(), 32);
        assert_eq!(root, MerkleTree::empty_root());
    }

    #[test]
    fn test_leaf_hash_freeze() {
        let h1 = MerkleTree::leaf_hash(b"test");
        let h2 = MerkleTree::leaf_hash(b"test");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_internal_hash_freeze() {
        let l = MerkleTree::leaf_hash(b"a");
        let r = MerkleTree::leaf_hash(b"b");
        let h1 = MerkleTree::internal_hash(&l, &r);
        let h2 = MerkleTree::internal_hash(&l, &r);
        assert_eq!(h1, h2);
        assert_ne!(h1, l);
        assert_ne!(h1, r);
    }

    #[test]
    fn test_single_leaf_root() {
        let leaf = MerkleTree::leaf_hash(b"hello");
        let root = MerkleTree::compute_root(&[leaf]);
        assert_eq!(root, leaf);
    }

    #[test]
    fn test_two_leaf_root_deterministic() {
        let l1 = MerkleTree::leaf_hash(b"a");
        let l2 = MerkleTree::leaf_hash(b"b");
        assert_eq!(
            MerkleTree::compute_root(&[l1, l2]),
            MerkleTree::compute_root(&[l1, l2])
        );
    }

    #[test]
    fn test_three_leaf_root_deterministic() {
        let l1 = MerkleTree::leaf_hash(b"a");
        let l2 = MerkleTree::leaf_hash(b"b");
        let l3 = MerkleTree::leaf_hash(b"c");
        assert_eq!(
            MerkleTree::compute_root(&[l1, l2, l3]),
            MerkleTree::compute_root(&[l1, l2, l3])
        );
    }

    #[test]
    fn test_odd_leaf_duplication_rule() {
        let l1 = MerkleTree::leaf_hash(b"a");
        let l2 = MerkleTree::leaf_hash(b"b");
        let l3 = MerkleTree::leaf_hash(b"c");
        let root3 = MerkleTree::compute_root(&[l1, l2, l3]);
        let root4 = MerkleTree::compute_root(&[l1, l2, l3, l3]);
        assert_eq!(root3, root4, "Odd-leaf duplication rule must be consistent");
    }

    #[test]
    fn test_different_order_produces_different_root() {
        let l1 = MerkleTree::leaf_hash(b"a");
        let l2 = MerkleTree::leaf_hash(b"b");
        assert_ne!(
            MerkleTree::compute_root(&[l1, l2]),
            MerkleTree::compute_root(&[l2, l1])
        );
    }

    #[test]
    fn test_proof_verify() {
        let l1 = MerkleTree::leaf_hash(b"a");
        let l2 = MerkleTree::leaf_hash(b"b");
        let root = MerkleTree::compute_root(&[l1, l2]);
        let mut proof = MerkleProof::new();
        proof.add_sibling(l2, Direction::Right).unwrap();
        assert!(proof.verify(&l1, &root));
    }

    #[test]
    fn test_proof_rejects_wrong_leaf() {
        let l1 = MerkleTree::leaf_hash(b"a");
        let l2 = MerkleTree::leaf_hash(b"b");
        let l3 = MerkleTree::leaf_hash(b"c");
        let root = MerkleTree::compute_root(&[l1, l2]);
        let mut proof = MerkleProof::new();
        proof.add_sibling(l2, Direction::Right).unwrap();
        assert!(!proof.verify(&l3, &root));
    }
}
