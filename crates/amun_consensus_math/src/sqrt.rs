#![forbid(unsafe_code)]

use crate::constants::{MAX_FIXED, SCALE, SQRT_ITERATIONS};
use crate::fixed::Fixed;

fn initial_sqrt_estimate(raw: i64) -> i64 {
    if raw <= 0 {
        return SCALE;
    }
    let bits = 64 - raw.leading_zeros();
    let half_bits = bits / 2;
    let mut est = 1_i64 << half_bits;
    est = est.saturating_mul(SCALE);
    est.clamp(SCALE, MAX_FIXED)
}

pub fn f_sqrt(x: Fixed) -> Fixed {
    let raw = x.raw();
    if raw < 0 {
        return Fixed::ZERO;
    }
    if raw == 0 {
        return Fixed::ZERO;
    }

    let mut y = initial_sqrt_estimate(raw);

    for _ in 0..SQRT_ITERATIONS {
        let raw_i128 = raw as i128;
        let y_i128 = y as i128;
        let scale_i128 = SCALE as i128;

        let x_div_y = ((raw_i128 * scale_i128) / y_i128) as i64;
        let y_next = (y + x_div_y) / 2;

        if (y_next - y).abs() <= 1 {
            return Fixed::from_raw(y_next);
        }
        y = y_next;
    }

    Fixed::from_raw(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_basic() {
        let x = Fixed::from_int(4);
        let result = f_sqrt(x);
        assert_eq!(result.raw(), 2 * SCALE);
    }
}
