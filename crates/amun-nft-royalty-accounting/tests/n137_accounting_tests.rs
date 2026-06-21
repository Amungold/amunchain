use amun_nft_royalty::RoyaltyRecord;
use amun_nft_royalty_accounting::RoyaltyLedger;

#[test]
fn n137_single_creator_accrual() {
    let mut ledger = RoyaltyLedger::new();
    let record = RoyaltyRecord {
        token_id: [1u8; 32],
        creator: [10u8; 32],
        payer: [20u8; 32],
        sale_price: 1000,
        royalty_amount: 50,
        block_height: 1,
    };
    ledger.settle(&record);
    assert_eq!(ledger.balance_of(&[10u8; 32]), 50);
}

#[test]
fn n137_multiple_sales_accumulation() {
    let mut ledger = RoyaltyLedger::new();
    let creator = [10u8; 32];
    for i in 0..5 {
        ledger.settle(&RoyaltyRecord {
            token_id: [i; 32],
            creator,
            payer: [20u8; 32],
            sale_price: 1000,
            royalty_amount: 100,
            block_height: i as u64,
        });
    }
    assert_eq!(ledger.balance_of(&creator), 500);
}

#[test]
fn n137_multiple_creators_independent_balances() {
    let mut ledger = RoyaltyLedger::new();
    ledger.settle(&RoyaltyRecord {
        token_id: [1u8; 32],
        creator: [10u8; 32],
        payer: [20u8; 32],
        sale_price: 1000,
        royalty_amount: 100,
        block_height: 1,
    });
    ledger.settle(&RoyaltyRecord {
        token_id: [2u8; 32],
        creator: [30u8; 32],
        payer: [40u8; 32],
        sale_price: 2000,
        royalty_amount: 200,
        block_height: 2,
    });
    assert_eq!(ledger.balance_of(&[10u8; 32]), 100);
    assert_eq!(ledger.balance_of(&[30u8; 32]), 200);
}

#[test]
fn n137_deterministic_accounting_root() {
    let mut ledger1 = RoyaltyLedger::new();
    let mut ledger2 = RoyaltyLedger::new();
    let record = RoyaltyRecord {
        token_id: [1u8; 32],
        creator: [99u8; 32],
        payer: [88u8; 32],
        sale_price: 5000,
        royalty_amount: 250,
        block_height: 42,
    };
    ledger1.settle(&record);
    ledger2.settle(&record);
    assert_eq!(ledger1.compute_accounting_root(), ledger2.compute_accounting_root());
}

#[test]
fn n137_overflow_safety() {
    let mut ledger = RoyaltyLedger::new();
    // Add a large amount
    ledger.settle(&RoyaltyRecord {
        token_id: [1u8; 32],
        creator: [10u8; 32],
        payer: [20u8; 32],
        sale_price: u64::MAX,
        royalty_amount: u64::MAX,
        block_height: 1,
    });
    // Add another amount; saturating_add should prevent panic
    ledger.settle(&RoyaltyRecord {
        token_id: [2u8; 32],
        creator: [10u8; 32],
        payer: [20u8; 32],
        sale_price: 1,
        royalty_amount: 1,
        block_height: 2,
    });
    assert_eq!(ledger.balance_of(&[10u8; 32]), u64::MAX);
}
