#![forbid(unsafe_code)]

pub const SCALE: i64 = 1_000_000;
pub const PRECISION: u32 = 6;

pub const FIXED_E: i64 = 2_718_281;
pub const MAX_TOTAL_COUPLING: i64 = 1_500_000;

pub const MIN_LEGITIMACY: i64 = 0;
pub const MAX_LEGITIMACY: i64 = SCALE;

pub const MAX_FIXED: i64 = 10_000_000_000 * SCALE;
pub const MIN_FIXED: i64 = -MAX_FIXED;

pub const EXP_ITERATIONS: usize = 24;
pub const SQRT_ITERATIONS: usize = 30;

pub const EXP_DOMAIN_MIN: i64 = -10 * SCALE;
pub const EXP_DOMAIN_MAX: i64 = 10 * SCALE;

#[inline(always)]
pub fn div_floor(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 0;
    }
    let q = a / b;
    let r = a % b;
    if r != 0 && ((a < 0) != (b < 0)) {
        q - 1
    } else {
        q
    }
}

#[inline(always)]
pub fn mod_floor(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 0;
    }
    let r = a % b;
    if r == 0 {
        0
    } else if (a < 0) != (b < 0) {
        r + b
    } else {
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_floor() {
        assert_eq!(div_floor(7, 3), 2);
        assert_eq!(div_floor(-7, 3), -3);
        assert_eq!(div_floor(7, -3), -3);
        assert_eq!(div_floor(-7, -3), 2);
    }

    #[test]
    fn test_mod_floor() {
        assert_eq!(mod_floor(7, 3), 1);
        assert_eq!(mod_floor(-7, 3), 2);
        assert_eq!(mod_floor(7, -3), -2);
        assert_eq!(mod_floor(-7, -3), -1);
    }
}
