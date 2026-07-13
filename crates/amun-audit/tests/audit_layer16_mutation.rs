#[cfg(test)]
mod audit_mutation {
    use amun_canonical_codec::CanonicalWriter;
    use amun_storage_kernel::SparseMerkleTree;

    // CONST-MUT-001: Changing MAX_DEPTH would be detected by audit
    #[test]
    fn mut001_max_depth_is_frozen() {
        let depth = amun_storage_kernel::smt::tree::MAX_DEPTH;
        assert_eq!(
            depth, 256,
            "CONST-MUT-001: MAX_DEPTH mutation detected - must be 256, got {}",
            depth
        );
    }

    // CONST-MUT-002: Proof version must remain 0x01
    #[test]
    fn mut002_proof_version_frozen() {
        let version = amun_storage_kernel::smt::proof::PROOF_VERSION_V1;
        assert_eq!(
            version, 0x01,
            "CONST-MUT-002: Proof version mutation detected - must be 0x01, got {}",
            version
        );
    }

    // CONST-MUT-003: Empty root is not zero
    #[test]
    fn mut003_empty_root_not_mutable() {
        let root = SparseMerkleTree::canonical_empty_root();
        assert_ne!(
            root, [0u8; 32],
            "CONST-MUT-003: Empty root must not be zero hash"
        );
    }

    // CONST-MUT-004: Endianness invariant
    #[test]
    fn mut004_endian_invariant() {
        let mut w = CanonicalWriter::new();
        w.write_u32(0x01020304);
        let bytes = w.into_bytes();
        assert_eq!(
            bytes[0], 0x04,
            "CONST-MUT-004: Little-endian invariant violated at byte 0"
        );
        assert_eq!(
            bytes[3], 0x01,
            "CONST-MUT-004: Little-endian invariant violated at byte 3"
        );
    }
}
