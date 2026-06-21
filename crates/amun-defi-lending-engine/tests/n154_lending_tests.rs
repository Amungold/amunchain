use amun_defi_lending_core::InterestModel;
use amun_defi_lending_engine::LendingEngine;

#[test]
fn n154_loan_creation_and_repayment() {
    let mut engine = LendingEngine::new();
    let borrower = [10u8; 32];
    let (loan_id, _collateral_id) = engine.create_loan(&mut amun_resource_core::ResourceRegistry::new(100), borrower, 1000, 500, 2000, [30u8; 32], 1).unwrap();
    engine.accrue_interest(&loan_id.0, 1000);
    let repaid = engine.repay(&loan_id.0, 500).unwrap();
    assert_eq!(repaid, 500);
}

#[test]
fn n154_interest_accrual_increases_debt() {
    let mut engine = LendingEngine::new();
    let borrower = [10u8; 32];
    let (loan_id, _) = engine.create_loan(&mut amun_resource_core::ResourceRegistry::new(100), borrower, 10000, 500, 20000, [30u8; 32], 1).unwrap();
    let interest = engine.accrue_interest(&loan_id.0, 210240).unwrap();
    assert!(interest > 0);
}

#[test]
fn n154_liquidation_triggered_when_health_factor_low() {
    let mut engine = LendingEngine::new();
    let borrower = [10u8; 32];
    let (loan_id, _) = engine.create_loan(&mut amun_resource_core::ResourceRegistry::new(100), borrower, 1000, 500, 500, [30u8; 32], 1).unwrap();
    let health = engine.get_health_factor(&loan_id.0, 2_000_000);
    assert!(InterestModel::is_liquidatable(health));
}

#[test]
fn n154_full_repayment_closes_loan() {
    let mut engine = LendingEngine::new();
    let borrower = [10u8; 32];
    let (loan_id, _) = engine.create_loan(&mut amun_resource_core::ResourceRegistry::new(100), borrower, 1000, 0, 2000, [30u8; 32], 1).unwrap();
    engine.repay(&loan_id.0, 1000).unwrap();
    let loan = engine.loans.get(&loan_id.0).unwrap();
    assert!(!loan.active);
}

#[test]
fn n154_lending_root_deterministic() {
    let mut engine1 = LendingEngine::new();
    let mut engine2 = LendingEngine::new();
    let (_id1, _) = engine1.create_loan(&mut amun_resource_core::ResourceRegistry::new(100), [10u8; 32], 500, 300, 1000, [40u8; 32], 1).unwrap();
    let (_id2, _) = engine2.create_loan(&mut amun_resource_core::ResourceRegistry::new(100), [10u8; 32], 500, 300, 1000, [40u8; 32], 1).unwrap();
    assert_eq!(engine1.compute_lending_root(), engine2.compute_lending_root());
}
