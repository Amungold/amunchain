use amun_consensus_math::*;

#[test]
fn test_saturation_edges() {
    let max_fixed = Fixed::from_raw(MAX_FIXED);
    let min_fixed = Fixed::from_raw(MIN_FIXED);
    let one = Fixed::ONE;
    
    // Addition saturation
    let overflow_add = max_fixed + one;
    assert_eq!(overflow_add, max_fixed);
    
    let underflow_add = min_fixed - one;
    assert_eq!(underflow_add, min_fixed);
    
    // Multiplication saturation
    let overflow_mul = max_fixed * Fixed::from_int(2);
    assert_eq!(overflow_mul, max_fixed);
    
    // Division by zero
    let div_by_zero = one / Fixed::ZERO;
    assert_eq!(div_by_zero, Fixed::ZERO);
    
    // Legitimacy clamp
    let above = Fixed::from_int(2);
    assert_eq!(above.clamp_legitimacy(), Fixed::ONE);
    
    let below = Fixed::from_int(-1);
    assert_eq!(below.clamp_legitimacy(), Fixed::ZERO);
}

#[test]
fn test_floor_division_consistency() {
    let a = Fixed::from_int(-7);
    let b = Fixed::from_int(3);
    
    let result = a / b;
    let expected = Fixed::from_int(-3);
    
    assert_eq!(result, expected);
}
