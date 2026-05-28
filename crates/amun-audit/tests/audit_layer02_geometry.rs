#[cfg(test)]
mod audit_geometry {
    use amun_storage_kernel::{Key256, SparseMerkleTree};

    // CONST-GEO-001: All proofs have exactly 256 steps
    #[test]
    fn geo001_proof_depth_invariant() {
        let key = Key256([0x11u8; 32]);
        let value = [0x22u8; 32];
        let tree = SparseMerkleTree::empty().insert(&key, &value, 0);

        let proof = tree
            .generate_inclusion_proof(&key)
            .expect("CONST-GEO-001: Must generate inclusion proof");
        assert_eq!(
            proof.steps.len(),
            256,
            "CONST-GEO-001 VIOLATION: Inclusion proof has {} steps, expected 256",
            proof.steps.len()
        );

        let absence_key = Key256([0xFFu8; 32]);
        let abs_proof = tree
            .generate_absence_proof(&absence_key)
            .expect("CONST-GEO-001: Must generate absence proof");
        assert_eq!(
            abs_proof.steps.len(),
            256,
            "CONST-GEO-001 VIOLATION: Absence proof has {} steps, expected 256",
            abs_proof.steps.len()
        );
    }

    // CONST-GEO-002: Empty ladder terminal node is ZERO
    #[test]
    fn geo002_empty_ladder_terminal() {
        let tree = SparseMerkleTree::empty();
        assert_eq!(
            tree.empty_ladder[256].0, [0u8; 32],
            "CONST-GEO-002 VIOLATION: Terminal empty node must be ZERO"
        );
    }

    // CONST-GEO-003: Empty ladder root is not ZERO
    #[test]
    fn geo003_empty_root_not_zero() {
        let tree = SparseMerkleTree::empty();
        assert_ne!(
            tree.empty_ladder[0].0, [0u8; 32],
            "CONST-GEO-003 VIOLATION: Empty root must not be ZERO"
        );
    }

    // CONST-GEO-004: Insert+delete returns to canonical empty
    #[test]
    fn geo004_insert_delete_cycle() {
        let key = Key256([0x55u8; 32]);
        let value = [0x66u8; 32];
        let tree = SparseMerkleTree::empty().insert(&key, &value, 0);
        let deleted = tree.delete(&key);
        assert_eq!(
            deleted.root().0,
            SparseMerkleTree::canonical_empty_root(),
            "CONST-GEO-004 VIOLATION: Insert+delete must return to canonical empty"
        );
    }

    // CONST-GEO-005: MAX_DEPTH is exactly 256
    #[test]
    fn geo005_max_depth_frozen() {
        assert_eq!(
            amun_storage_kernel::smt::tree::MAX_DEPTH,
            256,
            "CONST-GEO-005 VIOLATION: MAX_DEPTH must be exactly 256"
        );
    }
}
