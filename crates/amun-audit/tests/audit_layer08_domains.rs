#[cfg(test)]
mod audit_domains {
    use amun_canonical_codec::{
        PROTOCOL_CHAIN_ID, PROTOCOL_DOMAIN_BRANCH, PROTOCOL_DOMAIN_CHUNK,
        PROTOCOL_DOMAIN_CHUNK_MERKLE, PROTOCOL_DOMAIN_CONSTITUTION, PROTOCOL_DOMAIN_LEAF,
        PROTOCOL_DOMAIN_LINEAGE, PROTOCOL_DOMAIN_MANIFEST, PROTOCOL_DOMAIN_SNAPSHOT,
        PROTOCOL_DOMAIN_WAL,
    };
    use std::collections::HashSet;

    // CONST-DOMAIN-001: All domains are unique
    #[test]
    fn domain001_all_domains_unique() {
        let mut seen = HashSet::new();
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
        for (name, domain) in &domains {
            let hash = blake3::hash(domain);
            assert!(
                seen.insert(hash.as_bytes().to_vec()),
                "CONST-DOMAIN-001 VIOLATION: Domain collision for '{}'",
                name
            );
        }
    }

    // CONST-DOMAIN-002: Chain ID is exactly 32 bytes
    #[test]
    fn domain002_chain_id_32_bytes() {
        assert_eq!(
            PROTOCOL_CHAIN_ID.len(),
            32,
            "CONST-DOMAIN-002 VIOLATION: Chain ID must be exactly 32 bytes"
        );
        assert_ne!(
            PROTOCOL_CHAIN_ID, [0u8; 32],
            "CONST-DOMAIN-002 VIOLATION: Chain ID must not be all zeros"
        );
    }

    // CONST-DOMAIN-003: All domains contain version suffix
    #[test]
    fn domain003_domains_are_versioned() {
        let domains: Vec<&[u8]> = vec![
            PROTOCOL_DOMAIN_LEAF,
            PROTOCOL_DOMAIN_BRANCH,
            PROTOCOL_DOMAIN_WAL,
            PROTOCOL_DOMAIN_LINEAGE,
            PROTOCOL_DOMAIN_SNAPSHOT,
            PROTOCOL_DOMAIN_MANIFEST,
            PROTOCOL_DOMAIN_CHUNK,
            PROTOCOL_DOMAIN_CHUNK_MERKLE,
            PROTOCOL_DOMAIN_CONSTITUTION,
        ];
        for domain in &domains {
            let s = String::from_utf8_lossy(domain);
            assert!(
                s.contains("_V1"),
                "CONST-DOMAIN-003 VIOLATION: Domain '{}' missing _V1 suffix",
                s
            );
        }
    }
}
