use amun_defi_stress::*;

#[test]
fn n158_stress_amm_swaps_1000() {
    let result = stress_amm_swaps(1000);
    assert!(result.passed(), "AMM stress failed: {} failed, {} invariants broken", result.failed, result.invariants_broken);
}

#[test]
fn n158_stress_lending_liquidations_500() {
    let result = stress_lending_liquidations(500);
    assert!(result.passed(), "Lending stress failed: {} failed, {} invariants broken", result.failed, result.invariants_broken);
}

#[test]
fn n158_stress_stablecoin_mint_burn_1000() {
    let result = stress_stablecoin_mint_burn(1000);
    assert!(result.passed(), "Stablecoin stress failed: {} failed, {} invariants broken", result.failed, result.invariants_broken);
}

#[test]
fn n158_stress_nft_collateral_flow_100() {
    let result = stress_nft_collateral_flow(100);
    assert!(result.passed(), "NFT Collateral stress failed: {} failed, {} invariants broken", result.failed, result.invariants_broken);
}

#[test]
fn n158_full_defi_integration_stress() {
    let amm = stress_amm_swaps(200);
    let lending = stress_lending_liquidations(200);
    let stablecoin = stress_stablecoin_mint_burn(200);
    let nft_col = stress_nft_collateral_flow(50);
    assert!(amm.passed(), "AMM failed");
    assert!(lending.passed(), "Lending failed");
    assert!(stablecoin.passed(), "Stablecoin failed");
    assert!(nft_col.passed(), "NFT Collateral failed");
}
