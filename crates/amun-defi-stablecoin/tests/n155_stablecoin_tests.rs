use amun_resource_core::ResourceRegistry;
use amun_defi_stablecoin::StablecoinEngine;

#[test]
fn n155_mint_and_burn() {
    let mut reg = ResourceRegistry::new(10);
    let mut engine = StablecoinEngine::new();
    let owner = [10u8; 32];
    let pos_id = engine.mint(&mut reg, owner, 300, 200).unwrap();
    assert_eq!(engine.total_supply, 200);
    engine.burn(&pos_id, 100).unwrap();
    assert_eq!(engine.total_supply, 100);
}

#[test]
fn n155_cannot_mint_above_collateral_ratio() {
    let mut reg = ResourceRegistry::new(10);
    let mut engine = StablecoinEngine::new();
    let result = engine.mint(&mut reg, [10u8; 32], 300, 250);
    assert!(result.is_err());
}

#[test]
fn n155_stablecoin_root_deterministic() {
    let mut reg1 = ResourceRegistry::new(10);
    let mut reg2 = ResourceRegistry::new(10);
    let mut engine1 = StablecoinEngine::new();
    let mut engine2 = StablecoinEngine::new();
    engine1.mint(&mut reg1, [10u8; 32], 300, 200).unwrap();
    engine2.mint(&mut reg2, [10u8; 32], 300, 200).unwrap();
    assert_eq!(engine1.compute_stablecoin_root(), engine2.compute_stablecoin_root());
}
