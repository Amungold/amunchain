use amun_nft_royalty::RoyaltyRecord;
use amun_nft_royalty_accounting::RoyaltyLedger;
use amun_nft_royalty_settlement::SettlementEngine;

#[test]
fn n142_settle_accumulated_royalties() {
    let mut ledger = RoyaltyLedger::new();
    let creator = [10u8; 32];
    ledger.settle(&RoyaltyRecord {
        token_id: [1u8; 32], creator, payer: [20u8; 32], sale_price: 1000, royalty_amount: 50, block_height: 1,
    });
    ledger.settle(&RoyaltyRecord {
        token_id: [2u8; 32], creator, payer: [20u8; 32], sale_price: 2000, royalty_amount: 100, block_height: 2,
    });

    let mut engine = SettlementEngine::new();
    let result = engine.settle(&ledger, &creator, 3, vec![]);
    assert!(result.is_some());
    assert_eq!(result.unwrap().amount, 150);
}

#[test]
fn n142_no_settlement_for_zero_balance() {
    let ledger = RoyaltyLedger::new();
    let mut engine = SettlementEngine::new();
    let result = engine.settle(&ledger, &[99u8; 32], 5, vec![]);
    assert!(result.is_none());
}

#[test]
fn n142_deterministic_settlement_root() {
    let mut ledger = RoyaltyLedger::new();
    let creator = [10u8; 32];
    ledger.settle(&RoyaltyRecord {
        token_id: [1u8; 32], creator, payer: [20u8; 32], sale_price: 500, royalty_amount: 25, block_height: 10,
    });

    let mut e1 = SettlementEngine::new();
    let mut e2 = SettlementEngine::new();
    let records = vec![];
    e1.settle(&ledger, &creator, 11, records.clone());
    e2.settle(&ledger, &creator, 11, records);
    assert_eq!(e1.compute_settlement_root(), e2.compute_settlement_root());
}

#[test]
fn n142_multiple_settlements_differ() {
    let mut ledger = RoyaltyLedger::new();
    let c1 = [10u8; 32];
    let c2 = [20u8; 32];
    ledger.settle(&RoyaltyRecord {
        token_id: [1u8; 32], creator: c1, payer: [30u8; 32], sale_price: 100, royalty_amount: 10, block_height: 1,
    });
    ledger.settle(&RoyaltyRecord {
        token_id: [2u8; 32], creator: c2, payer: [30u8; 32], sale_price: 200, royalty_amount: 20, block_height: 2,
    });

    let mut engine = SettlementEngine::new();
    let s1 = engine.settle(&ledger, &c1, 3, vec![]).unwrap();
    let s2 = engine.settle(&ledger, &c2, 3, vec![]).unwrap();
    assert_ne!(s1.amount, s2.amount);
    assert_eq!(s1.creator, c1);
    assert_eq!(s2.creator, c2);
}
