use amun_kernel::canonical::CanonicalEncode;
use amun_kernel::canonical::CanonicalEncoder;
use amun_kernel::hashing::domain_tags;

#[test]
fn test_canonical_encoding_injective() {
    let pair1 = (String::from("a"), String::from("bc"));
    let pair2 = (String::from("ab"), String::from("c"));
    let mut enc1 = Vec::new();
    pair1.encode_canonical(&mut enc1);
    let mut enc2 = Vec::new();
    pair2.encode_canonical(&mut enc2);
    assert_ne!(enc1, enc2);
    let h1 = CanonicalEncoder::hash_value(&pair1, domain_tags::STATE_ROOT);
    let h2 = CanonicalEncoder::hash_value(&pair2, domain_tags::STATE_ROOT);
    assert_ne!(h1, h2);
}

#[test]
fn test_canonical_hash_sorted_rejects_unsorted() {
    let items = vec![3u64, 1u64, 2u64];
    let result = std::panic::catch_unwind(|| {
        CanonicalEncoder::hash_sorted(items.clone(), domain_tags::STATE_ROOT);
    });
    assert!(result.is_err());
}
