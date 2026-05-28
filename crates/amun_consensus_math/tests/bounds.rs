use amun_consensus_math::*;

#[test]
fn test_legitimacy_bounds() {
    let valid = Fixed::from_int(1);
    assert_eq!(valid.clamp_legitimacy(), valid);

    let above = Fixed::from_int(2);
    assert_eq!(above.clamp_legitimacy(), Fixed::ONE);

    let below = Fixed::from_int(-1);
    assert_eq!(below.clamp_legitimacy(), Fixed::ZERO);
}

#[test]
fn test_fixed_bounds() {
    let max_allowed = Fixed::from_raw(MAX_FIXED);
    assert_eq!(max_allowed.raw(), MAX_FIXED);

    // Test saturation via multiplication (no unused variables)
    let huge_mul = Fixed::from_int(10_000_000_001) * Fixed::ONE;
    assert_eq!(huge_mul.raw(), MAX_FIXED);
}
