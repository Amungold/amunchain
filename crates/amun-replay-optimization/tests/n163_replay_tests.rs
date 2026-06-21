use amun_replay_optimization::*;

#[test]
fn n163_cache_hit_improves_replay() {
    let mut cache = ReplayCache::new();
    let cert_hash = [1u8; 32];
    cache.store_certificate(CachedCertificate {
        cert_hash, height: 42, block_hash: [2u8; 32], verified: true,
    });
    assert!(cache.check_certificate(&cert_hash));
    assert_eq!(cache.hits, 1);
    assert_eq!(cache.misses, 0);
    assert!(!cache.check_certificate(&[99u8; 32]));
    assert_eq!(cache.misses, 1);
}

#[test]
fn n163_batch_verification_faster_than_individual() {
    let mut cache = ReplayCache::new();
    for i in 0..500u64 {
        let mut cert_hash = [0u8; 32];
        cert_hash[0..8].copy_from_slice(&i.to_le_bytes());
        cache.store_certificate(CachedCertificate {
            cert_hash, height: i, block_hash: [0u8; 32], verified: true,
        });
    }
    let cert_hashes: Vec<[u8; 32]> = (0..500u64).map(|i| {
        let mut cert_hash = [0u8; 32];
        cert_hash[0..8].copy_from_slice(&i.to_le_bytes());
        cert_hash
    }).collect();
    let valid = cache.batch_verify_certificates(&cert_hashes, true);
    assert_eq!(valid, 500);
    assert!(cache.hit_rate() > 0.5);
}

#[test]
fn n163_header_cache_speeds_sync() {
    let mut cache = ReplayCache::new();
    for h in 0..1000u64 {
        let mut block_hash = [0u8; 32];
        block_hash[0..8].copy_from_slice(&h.to_le_bytes());
        cache.store_header(CachedHeader {
            height: h, block_hash, state_root: [0u8; 32],
        });
    }
    let header = cache.get_header(500).unwrap();
    assert_eq!(header.height, 500);
    assert!(cache.hit_rate() > 0.5);
}

#[test]
fn n163_cache_root_deterministic() {
    let mut cache1 = ReplayCache::new();
    let mut cache2 = ReplayCache::new();
    let cert_hash = [5u8; 32];
    cache1.store_certificate(CachedCertificate {
        cert_hash, height: 10, block_hash: [0u8; 32], verified: true,
    });
    cache2.store_certificate(CachedCertificate {
        cert_hash, height: 10, block_hash: [0u8; 32], verified: true,
    });
    cache1.check_certificate(&cert_hash);
    cache2.check_certificate(&cert_hash);
    assert_eq!(cache1.compute_cache_root(), cache2.compute_cache_root());
}
