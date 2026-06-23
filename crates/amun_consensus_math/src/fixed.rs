#![forbid(unsafe_code)]

use crate::constants::{
    div_floor, mod_floor, MAX_FIXED, MAX_LEGITIMACY, MIN_FIXED, MIN_LEGITIMACY, SCALE,
};

use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Fixed(i64);

impl Fixed {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(SCALE);
    pub const HALF: Self = Self(SCALE / 2);

    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        use std::str::FromStr;
        <Self as FromStr>::from_str(s).ok()
    }

    pub fn from_int(value: i64) -> Self {
        Self(value.saturating_mul(SCALE))
    }

    pub const fn raw(self) -> i64 {
        self.0
    }

    pub fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub fn saturating_add(self, other: Self) -> Self {
        let result = self.0.saturating_add(other.0);
        Self(result.clamp(MIN_FIXED, MAX_FIXED))
    }

    pub fn saturating_sub(self, other: Self) -> Self {
        let result = self.0.saturating_sub(other.0);
        Self(result.clamp(MIN_FIXED, MAX_FIXED))
    }

    pub fn saturating_mul(self, other: Self) -> Self {
        let product = (self.0 as i128).saturating_mul(other.0 as i128);
        let scaled = product / SCALE as i128;
        let result = scaled.clamp(MIN_FIXED as i128, MAX_FIXED as i128) as i64;

        Self(result)
    }

    pub fn saturating_div(self, other: Self) -> Self {
        if other.0 == 0 {
            return Self::ZERO;
        }

        let quotient = div_floor(self.0, other.0);
        let scaled = quotient.saturating_mul(SCALE);

        Self(scaled.clamp(MIN_FIXED, MAX_FIXED))
    }

    pub fn remainder(self, other: Self) -> Self {
        if other.0 == 0 {
            return Self::ZERO;
        }

        let rem = mod_floor(self.0, other.0);

        Self(rem.clamp(MIN_FIXED, MAX_FIXED))
    }

    pub fn clamp_legitimacy(self) -> Self {
        Self(self.0.clamp(MIN_LEGITIMACY, MAX_LEGITIMACY))
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self(self.0.clamp(min.0, max.0))
    }

    pub fn to_float(self) -> f64 {
        self.0 as f64 / SCALE as f64
    }

    pub fn abs(self) -> Self {
        Self(self.0.saturating_abs())
    }
}

impl FromStr for Fixed {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(());
        }

        let (sign, num_str) = if let Some(stripped) = s.strip_prefix('-') {
            (-1, stripped)
        } else if let Some(stripped) = s.strip_prefix('+') {
            (1, stripped)
        } else {
            (1, s)
        };

        let parts: Vec<&str> = num_str.split('.').collect();

        if parts.len() > 2 {
            return Err(());
        }

        let integer: i64 = parts[0].parse().map_err(|_| ())?;

        if parts[0].len() > 1 && parts[0].starts_with('0') {
            return Err(());
        }

        let fractional = if parts.len() == 2 {
            let frac_str = parts[1];

            if frac_str.len() > 6 {
                return Err(());
            }

            let mut frac_val: i64 = frac_str.parse().map_err(|_| ())?;

            for _ in frac_str.len()..6 {
                frac_val = frac_val.saturating_mul(10);
            }

            frac_val
        } else {
            0
        };

        let mut value = integer.saturating_mul(SCALE);
        value = value.saturating_add(fractional);

        if sign == -1 {
            value = value.saturating_neg();
        }

        if !(MIN_FIXED..=MAX_FIXED).contains(&value) {
            return Err(());
        }

        Ok(Self(value))
    }
}

impl Add for Fixed {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}

impl Sub for Fixed {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}

impl Mul for Fixed {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}

impl Div for Fixed {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}

impl Neg for Fixed {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.saturating_neg())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds() {
        let a = Fixed::from_int(1);
        assert_eq!(a.raw(), SCALE);
    }
}
