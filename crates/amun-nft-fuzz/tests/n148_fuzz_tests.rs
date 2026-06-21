use amun_nft_fuzz::*;

#[test]
fn n148_fuzz_mint_1000_iterations() {
    let result = fuzz_mint(1000);
    assert!(result.passed(), "Fuzz mint failed: {} crashes, {} invariants broken",
        result.crashes, result.state_invariants_broken);
}

#[test]
fn n148_fuzz_marketplace_500_iterations() {
    let result = fuzz_marketplace(500);
    assert!(result.passed(), "Fuzz marketplace failed: {} crashes, {} invariants broken",
        result.crashes, result.state_invariants_broken);
}

#[test]
fn n148_fuzz_royalty_10000_iterations() {
    let result = fuzz_royalty(10000);
    assert!(result.passed(), "Fuzz royalty failed: {} crashes, {} invariants broken",
        result.crashes, result.state_invariants_broken);
}

#[test]
fn n148_fuzz_governance_1000_iterations() {
    let result = fuzz_governance(1000);
    assert!(result.passed(), "Fuzz governance failed: {} crashes, {} invariants broken",
        result.crashes, result.state_invariants_broken);
}

#[test]
fn n148_fuzz_bridge_1000_iterations() {
    let result = fuzz_bridge(1000);
    assert!(result.passed(), "Fuzz bridge failed: {} crashes, {} invariants broken",
        result.crashes, result.state_invariants_broken);
}
