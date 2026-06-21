use amun_resource_core::ResourceRegistry;
use amun_defi_amm::AmmEngine;

#[test]
fn n153_create_pool_and_swap() {
    let mut reg = ResourceRegistry::new(100);
    let mut amm = AmmEngine::new();
    let token_a = [1u8; 32];
    let token_b = [2u8; 32];
    let pool_id = amm.create_pool(&mut reg, token_a, token_b, [10u8; 32]).unwrap();
    amm.add_liquidity(&pool_id.0, 100_000, 100_000);
    let swap_out = amm.swap(&pool_id.0, 100, true).unwrap();
    assert!(swap_out > 0);
}

#[test]
fn n153_pool_evidence_root_deterministic() {
    let mut reg1 = ResourceRegistry::new(100);
    let mut reg2 = ResourceRegistry::new(100);
    let mut amm1 = AmmEngine::new();
    let mut amm2 = AmmEngine::new();
    let pool_id1 = amm1.create_pool(&mut reg1, [1u8; 32], [2u8; 32], [10u8; 32]).unwrap();
    let pool_id2 = amm2.create_pool(&mut reg2, [1u8; 32], [2u8; 32], [10u8; 32]).unwrap();
    amm1.add_liquidity(&pool_id1.0, 500, 500);
    amm2.add_liquidity(&pool_id2.0, 500, 500);
    assert_eq!(amm1.compute_evidence_root(), amm2.compute_evidence_root());
}

#[test]
fn n153_swap_changes_evidence_root() {
    let mut reg = ResourceRegistry::new(100);
    let mut amm = AmmEngine::new();
    let pool_id = amm.create_pool(&mut reg, [1u8; 32], [2u8; 32], [10u8; 32]).unwrap();
    amm.add_liquidity(&pool_id.0, 1000, 1000);
    let root_before = amm.compute_evidence_root();
    amm.swap(&pool_id.0, 100, true);
    let root_after = amm.compute_evidence_root();
    assert_ne!(root_before, root_after);
}
