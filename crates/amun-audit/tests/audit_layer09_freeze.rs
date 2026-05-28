#[cfg(test)]
mod audit_freeze {
    use amun_storage_kernel::SparseMerkleTree;

    // CONST-FREEZE-001: MAX_DEPTH is frozen at 256
    #[test]
    fn freeze001_max_depth_is_256() {
        assert_eq!(
            amun_storage_kernel::smt::tree::MAX_DEPTH,
            256,
            "CONST-FREEZE-001 VIOLATION: MAX_DEPTH must be exactly 256"
        );
    }

    // CONST-FREEZE-002: Proof version is frozen at 0x01
    #[test]
    fn freeze002_proof_version_is_v1() {
        assert_eq!(
            amun_storage_kernel::smt::proof::PROOF_VERSION_V1,
            0x01,
            "CONST-FREEZE-002 VIOLATION: Proof version must be 0x01"
        );
    }

    // CONST-FREEZE-003: Canonical empty root is stable
    #[test]
    fn freeze003_empty_root_stability() {
        let root1 = SparseMerkleTree::canonical_empty_root();
        let root2 = SparseMerkleTree::canonical_empty_root();
        assert_eq!(
            root1, root2,
            "CONST-FREEZE-003 VIOLATION: Empty root not stable across calls"
        );
    }
}
