use amun_nft_royalty::*;

#[test]
fn n136_direct_sale_royalty_5_percent() {
    let policy = RoyaltyPolicy {
        creator: [1u8; 32],
        royalty_bps: 500,
    }; // 5%
    let amount = RoyaltyEngine::compute_royalty(1000, policy.royalty_bps);
    assert_eq!(amount, 50);
}

#[test]
fn n136_auction_royalty_10_percent() {
    let policy = RoyaltyPolicy {
        creator: [2u8; 32],
        royalty_bps: 1000,
    }; // 10%
    let record = RoyaltyEngine::generate_royalty_record([10u8; 32], &policy, [3u8; 32], 5000, 42);
    assert_eq!(record.royalty_amount, 500);
    assert_eq!(record.creator, [2u8; 32]);
    assert_eq!(record.block_height, 42);
}

#[test]
fn n136_zero_royalty() {
    let policy = RoyaltyPolicy {
        creator: [4u8; 32],
        royalty_bps: 0,
    };
    let amount = RoyaltyEngine::compute_royalty(1000, policy.royalty_bps);
    assert_eq!(amount, 0);
}

#[test]
fn n136_overflow_safety() {
    let policy = RoyaltyPolicy {
        creator: [5u8; 32],
        royalty_bps: 10000,
    }; // 100%
    let amount = RoyaltyEngine::compute_royalty(u64::MAX, policy.royalty_bps);
    // Should not panic, and should be ~ u64::MAX (but may be limited by u128 conv)
    assert!(amount > 0);
}

#[test]
fn n136_deterministic_evidence_root() {
    let policy = RoyaltyPolicy {
        creator: [1u8; 32],
        royalty_bps: 500,
    };
    let record1 = RoyaltyEngine::generate_royalty_record([10u8; 32], &policy, [3u8; 32], 1000, 1);
    let record2 = RoyaltyEngine::generate_royalty_record([11u8; 32], &policy, [4u8; 32], 2000, 2);

    let root1 = accumulate_royalty_root(&[record1.clone(), record2.clone()]);
    let root2 = accumulate_royalty_root(&[record1, record2]);
    assert_eq!(root1, root2);
}

#[test]
fn n136_royalty_record_serialization_roundtrip() {
    let record = RoyaltyEngine::generate_royalty_record(
        [99u8; 32],
        &RoyaltyPolicy {
            creator: [7u8; 32],
            royalty_bps: 250,
        },
        [8u8; 32],
        10000,
        5,
    );
    let json = serde_json::to_string(&record).unwrap();
    let restored: RoyaltyRecord = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.royalty_amount, 250);
    assert_eq!(restored.block_height, 5);
}
