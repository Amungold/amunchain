#![forbid(unsafe_code)]

use crate::fixed::Fixed;
use crate::constants::SCALE;

pub fn round_half_to_even(value: Fixed) -> i64 {
    let raw = value.raw();
    let sign = if raw < 0 { -1 } else { 1 };
    let abs_raw = raw.saturating_abs();
    let fractional = abs_raw % SCALE;
    let integer = abs_raw / SCALE;
    
    let rounded_abs = if fractional * 2 > SCALE {
        integer + 1
    } else if fractional * 2 == SCALE {
        if integer % 2 == 0 {
            integer
        } else {
            integer + 1
        }
    } else {
        integer
    };
    
    sign * rounded_abs
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rounding() {
        let a = Fixed::from_str("1.5").unwrap();
        assert_eq!(round_half_to_even(a), 2);
    }
}
