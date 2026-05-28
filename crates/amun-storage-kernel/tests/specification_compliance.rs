#[cfg(test)]
mod specification_tests {
    use amun_storage_kernel::{smt::constants::CANONICAL_EMPTY_ROOT_V1, Key256, SparseMerkleTree};

    // ============================================================
    // THEOREM 1: Order Independence
    // ============================================================
    #[test]
    fn theorem_order_independence_two_keys() {
        let k1 = Key256([1u8; 32]);
        let k2 = Key256([2u8; 32]);
        let v1 = [10u8; 32];
        let v2 = [20u8; 32];

        let root_a = SparseMerkleTree::empty()
            .insert(&k1, &v1, 0)
            .insert(&k2, &v2, 0)
            .root();

        let root_b = SparseMerkleTree::empty()
            .insert(&k2, &v2, 0)
            .insert(&k1, &v1, 0)
            .root();

        assert_eq!(root_a.0, root_b.0, "Theorem 1 violated: order independence");
    }

    // ============================================================
    // THEOREM 2: Delete-Reinsert Identity
    // ============================================================
    #[test]
    fn theorem_delete_reinsert_identity() {
        let k = Key256([99u8; 32]);
        let v = [42u8; 32];

        let fresh = SparseMerkleTree::empty().insert(&k, &v, 0);
        let cycled = SparseMerkleTree::empty()
            .insert(&k, &v, 0)
            .delete(&k)
            .insert(&k, &v, 0);

        assert_eq!(
            fresh.root().0,
            cycled.root().0,
            "Theorem 2 violated: delete-reinsert identity"
        );
    }

    // ============================================================
    // THEOREM 3: Empty Identity
    // ============================================================
    #[test]
    fn theorem_empty_identity() {
        let k = Key256([55u8; 32]);
        let v = [77u8; 32];

        let result = SparseMerkleTree::empty().insert(&k, &v, 0).delete(&k);

        assert_eq!(
            result.root().0,
            CANONICAL_EMPTY_ROOT_V1,
            "Theorem 3 violated: empty identity"
        );
    }

    // ============================================================
    // THEOREM 4: Proof Depth Invariant
    // ============================================================
    #[test]
    fn theorem_proof_depth() {
        let k = Key256([33u8; 32]);
        let v = [66u8; 32];
        let tree = SparseMerkleTree::empty().insert(&k, &v, 0);

        let proof = tree.generate_inclusion_proof(&k).unwrap();
        assert_eq!(
            proof.steps.len(),
            256,
            "Theorem 4 violated: proof depth != 256"
        );

        let abs_proof = tree.generate_absence_proof(&Key256([44u8; 32])).unwrap();
        assert_eq!(
            abs_proof.steps.len(),
            256,
            "Theorem 4 violated: absence proof depth != 256"
        );
    }

    // ============================================================
    // THEOREM 5: Delete Nonexistent is No-op
    // ============================================================
    #[test]
    fn theorem_delete_nonexistent_noop() {
        let k1 = Key256([10u8; 32]);
        let k2 = Key256([20u8; 32]);
        let v = [99u8; 32];

        let tree = SparseMerkleTree::empty().insert(&k1, &v, 0);
        let root_before = tree.root().0;
        let tree_after = tree.delete(&k2);
        assert_eq!(
            tree_after.root().0,
            root_before,
            "Theorem 5 violated: delete nonexistent changed root"
        );
    }

    // ============================================================
    // THEOREM 6: Proof Verification Roundtrip
    // ============================================================
    #[test]
    fn theorem_proof_roundtrip() {
        let k = Key256([12u8; 32]);
        let v = [34u8; 32];
        let tree = SparseMerkleTree::empty().insert(&k, &v, 0);
        let root = tree.root().0;

        let proof = tree.generate_inclusion_proof(&k).unwrap();
        assert!(
            proof.verify(root),
            "Theorem 6 violated: proof verification failed"
        );
    }

    // ============================================================
    // THEOREM 7: Terminal Empty Node is ZERO
    // ============================================================
    #[test]
    fn theorem_terminal_empty_is_zero() {
        let tree = SparseMerkleTree::empty();
        assert_eq!(
            tree.empty_ladder[256].0, [0u8; 32],
            "Theorem 7 violated: terminal empty != ZERO"
        );
    }

    // ============================================================
    // THEOREM 8: Empty Tree Absence Proof
    // ============================================================
    #[test]
    fn theorem_empty_tree_absence_proof() {
        let k = Key256([88u8; 32]);
        let tree = SparseMerkleTree::empty();
        let proof = tree.generate_absence_proof(&k).unwrap();
        let root = tree.root().0;
        assert!(
            proof.verify(root),
            "Theorem 8 violated: empty tree absence proof"
        );
    }
}
