#[cfg(test)]
mod audit_differential {
    use amun_canonical_codec::PROTOCOL_DOMAIN_LEAF;
    use amun_canonical_codec::{CanonicalHasher, CanonicalWriter};
    use amun_storage_kernel::SparseMerkleTree;

    // CONST-DIFF-001: Canonical encoding is deterministic
    #[test]
    fn diff001_canonical_encoding_determinism() {
        let data = b"differential test vector 001";
        let mut w1 = CanonicalWriter::new();
        w1.write_bytes(data);
        let bytes1 = w1.into_bytes();

        let mut w2 = CanonicalWriter::new();
        w2.write_bytes(data);
        let bytes2 = w2.into_bytes();

        assert_eq!(
            bytes1, bytes2,
            "CONST-DIFF-001 VIOLATION: Canonical encoding not deterministic"
        );
    }

    // CONST-DIFF-002: Domain-separated hashing is deterministic
    #[test]
    fn diff002_domain_hash_determinism() {
        let data = b"differential test vector 002";
        let mut h1 = CanonicalHasher::with_domain(PROTOCOL_DOMAIN_LEAF);
        h1.update(data);
        let hash1 = h1.finalize();

        let mut h2 = CanonicalHasher::with_domain(PROTOCOL_DOMAIN_LEAF);
        h2.update(data);
        let hash2 = h2.finalize();

        assert_eq!(
            hash1, hash2,
            "CONST-DIFF-002 VIOLATION: Domain hash not deterministic"
        );
    }

    // CONST-DIFF-003: Empty root is consistent across representations
    #[test]
    fn diff003_empty_root_consistency() {
        let root1 = SparseMerkleTree::canonical_empty_root();
        let tree = SparseMerkleTree::empty();
        let root2 = tree.root().0;
        assert_eq!(
            root1, root2,
            "CONST-DIFF-003 VIOLATION: Canonical empty root != tree empty root"
        );
    }
}
