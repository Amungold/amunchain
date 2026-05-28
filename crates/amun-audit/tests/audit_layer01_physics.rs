#[cfg(test)]
mod audit_physics {
    use amun_canonical_codec::CanonicalWriter;
    use amun_canonical_codec::{
        PROTOCOL_DOMAIN_BRANCH, PROTOCOL_DOMAIN_CHUNK, PROTOCOL_DOMAIN_CHUNK_MERKLE,
        PROTOCOL_DOMAIN_CONSTITUTION, PROTOCOL_DOMAIN_LEAF, PROTOCOL_DOMAIN_LINEAGE,
        PROTOCOL_DOMAIN_MANIFEST, PROTOCOL_DOMAIN_SNAPSHOT, PROTOCOL_DOMAIN_WAL,
    };
    use amun_storage_kernel::{Key256, SparseMerkleTree};
    use std::collections::HashSet;

    // CONST-PHYS-001: All domain separators are unique
    #[test]
    fn phys001_domain_separators_are_unique() {
        let domains: Vec<(&str, &[u8])> = vec![
            ("LEAF", PROTOCOL_DOMAIN_LEAF),
            ("BRANCH", PROTOCOL_DOMAIN_BRANCH),
            ("WAL", PROTOCOL_DOMAIN_WAL),
            ("LINEAGE", PROTOCOL_DOMAIN_LINEAGE),
            ("SNAPSHOT", PROTOCOL_DOMAIN_SNAPSHOT),
            ("MANIFEST", PROTOCOL_DOMAIN_MANIFEST),
            ("CHUNK", PROTOCOL_DOMAIN_CHUNK),
            ("CHUNK_MERKLE", PROTOCOL_DOMAIN_CHUNK_MERKLE),
            ("CONSTITUTION", PROTOCOL_DOMAIN_CONSTITUTION),
        ];
        let mut seen = HashSet::new();
        for (name, domain) in &domains {
            let hash = blake3::hash(domain);
            let inserted = seen.insert(hash.as_bytes().to_vec());
            assert!(
                inserted,
                "CONST-PHYS-001 VIOLATION: Domain collision for '{}'",
                name
            );
        }
    }

    // CONST-PHYS-002: Hash determinism is absolute
    #[test]
    fn phys002_hash_determinism() {
        let data = b"amun constitutional audit vector 002";
        let h1 = blake3::hash(data);
        let h2 = blake3::hash(data);
        let h3 = blake3::hash(data);
        assert_eq!(
            h1.as_bytes(),
            h2.as_bytes(),
            "CONST-PHYS-002 VIOLATION: Hash not deterministic (run 2)"
        );
        assert_eq!(
            h1.as_bytes(),
            h3.as_bytes(),
            "CONST-PHYS-002 VIOLATION: Hash not deterministic (run 3)"
        );
    }

    // CONST-PHYS-003: All integers are little-endian
    #[test]
    fn phys003_endian_consistency() {
        let value: u64 = 0x0102030405060708;
        let mut w = CanonicalWriter::new();
        w.write_u64(value);
        let bytes = w.into_bytes();
        assert_eq!(
            bytes.len(),
            8,
            "CONST-PHYS-003: u64 must serialize to 8 bytes"
        );
        assert_eq!(
            bytes[0], 0x08,
            "CONST-PHYS-003: Byte 0 must be LSB (little-endian)"
        );
        assert_eq!(
            bytes[7], 0x01,
            "CONST-PHYS-003: Byte 7 must be MSB (little-endian)"
        );
    }

    // CONST-PHYS-004: Same state produces identical serialized roots
    #[test]
    fn phys004_serialization_stability() {
        let key = Key256([0x42u8; 32]);
        let value = [0x99u8; 32];
        let tree1 = SparseMerkleTree::empty().insert(&key, &value, 1);
        let tree2 = SparseMerkleTree::empty().insert(&key, &value, 1);
        let tree3 = SparseMerkleTree::empty().insert(&key, &value, 1);
        assert_eq!(
            tree1.root().0,
            tree2.root().0,
            "CONST-PHYS-004 VIOLATION: Roots diverge (run 1 vs 2)"
        );
        assert_eq!(
            tree1.root().0,
            tree3.root().0,
            "CONST-PHYS-004 VIOLATION: Roots diverge (run 1 vs 3)"
        );
    }

    // CONST-PHYS-005: Canonical empty root is a frozen constant
    #[test]
    fn phys005_empty_root_is_constant() {
        let root1 = SparseMerkleTree::canonical_empty_root();
        let root2 = SparseMerkleTree::canonical_empty_root();
        assert_eq!(root1, root2, "CONST-PHYS-005: Empty root not stable");
        assert_ne!(
            root1, [0u8; 32],
            "CONST-PHYS-005: Empty root must not be zero hash"
        );
    }
}
