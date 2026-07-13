use amun_defi_evidence::DefiEvidence;

#[test]
fn n153_swap_evidence_deterministic() {
    let e1 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
    let e2 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
    assert_eq!(e1, e2);
}

#[test]
fn n153_swap_evidence_differs_by_amount() {
    let e1 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
    let e2 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 200, 180, 42);
    assert_ne!(e1, e2);
}

#[test]
fn n153_liquidity_evidence_deterministic() {
    let e1 = DefiEvidence::generate_liquidity_evidence([2u8; 32], [20u8; 32], 500, 500, 1000, 10);
    let e2 = DefiEvidence::generate_liquidity_evidence([2u8; 32], [20u8; 32], 500, 500, 1000, 10);
    assert_eq!(e1, e2);
}
