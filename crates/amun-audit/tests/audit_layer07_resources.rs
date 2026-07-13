#[cfg(test)]
mod audit_resources {
    use amun_canonical_codec::MAX_CANONICAL_ALLOCATION;
    use amun_snapshot_engine::snapshot::MAX_CHUNK_SIZE;

    // CONST-RES-001: MAX_CANONICAL_ALLOCATION is reasonable
    #[test]
    fn res001_allocation_guard() {
        const _: () = assert!(
            MAX_CANONICAL_ALLOCATION > 0,
            "CONST-RES-001 VIOLATION: MAX_CANONICAL_ALLOCATION must be positive"
        );
        const _: () = assert!(
            MAX_CANONICAL_ALLOCATION <= 256 * 1024 * 1024,
            "CONST-RES-001 VIOLATION: MAX_CANONICAL_ALLOCATION must be <= 256MB"
        );
    }

    // CONST-RES-002: Chunk size is exactly 16MB constitutional constant
    #[test]
    fn res002_chunk_size_frozen() {
        assert_eq!(
            MAX_CHUNK_SIZE,
            16 * 1024 * 1024,
            "CONST-RES-002 VIOLATION: Chunk size must be exactly 16MB"
        );
    }
}
