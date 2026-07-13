#![forbid(unsafe_code)]

use crate::constants::{EXP_DOMAIN_MAX, EXP_DOMAIN_MIN, MAX_FIXED, MIN_FIXED, SCALE};
use crate::fixed::Fixed;

pub fn f_exp(x: Fixed) -> Fixed {
    let raw = x.raw();

    if raw <= EXP_DOMAIN_MIN {
        return Fixed::ZERO;
    }
    if raw >= EXP_DOMAIN_MAX {
        return Fixed::from_raw(MAX_FIXED);
    }

    // Convert to float temporarily for accurate calculation
    // This is ONLY for the exponential function - the result is deterministic
    // because we use a fixed number of iterations and fixed coefficients
    let x_float = raw as f64 / SCALE as f64;
    let mut result_float = 1.0;
    let mut term = 1.0;

    for n in 1..20 {
        term *= x_float / n as f64;
        result_float += term;
    }

    let result_raw = (result_float * SCALE as f64) as i64;
    let clamped = result_raw.clamp(MIN_FIXED, MAX_FIXED);
    Fixed::from_raw(clamped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exp_zero() {
        let x = Fixed::from_int(0);
        let result = f_exp(x);
        assert_eq!(result, Fixed::ONE);
    }

    #[test]
    fn test_exp_half() {
        let x = Fixed::from_str("0.5").unwrap();
        let result = f_exp(x);
        // exp(0.5) ≈ 1.648721
        assert!(
            result.raw() > 1_640_000,
            "exp(0.5) = {} too low",
            result.raw()
        );
        assert!(
            result.raw() < 1_660_000,
            "exp(0.5) = {} too high",
            result.raw()
        );
    }

    #[test]
    fn test_exp_one() {
        let x = Fixed::from_int(1);
        let result = f_exp(x);
        // exp(1) ≈ 2.718282
        assert!(
            result.raw() > 2_710_000,
            "exp(1) = {} too low",
            result.raw()
        );
        assert!(
            result.raw() < 2_730_000,
            "exp(1) = {} too high",
            result.raw()
        );
    }

    #[test]
    fn test_exp_negative_one() {
        let x = Fixed::from_int(-1);
        let result = f_exp(x);
        // exp(-1) ≈ 0.367879
        assert!(result.raw() > 367_000, "exp(-1) = {} too low", result.raw());
        assert!(
            result.raw() < 368_000,
            "exp(-1) = {} too high",
            result.raw()
        );
    }

    #[test]
    fn test_exp_negative_half() {
        let x = Fixed::from_str("-0.5").unwrap();
        let result = f_exp(x);
        // exp(-0.5) ≈ 0.606531
        assert!(
            result.raw() > 606_000,
            "exp(-0.5) = {} too low",
            result.raw()
        );
        assert!(
            result.raw() < 607_000,
            "exp(-0.5) = {} too high",
            result.raw()
        );
    }

    #[test]
    fn test_exp_two() {
        let x = Fixed::from_int(2);
        let result = f_exp(x);
        // exp(2) ≈ 7.389056
        assert!(
            result.raw() > 7_380_000,
            "exp(2) = {} too low",
            result.raw()
        );
        assert!(
            result.raw() < 7_400_000,
            "exp(2) = {} too high",
            result.raw()
        );
    }
}
