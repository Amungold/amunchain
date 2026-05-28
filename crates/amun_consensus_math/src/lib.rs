#![forbid(unsafe_code)]

pub mod constants;
pub mod fixed;
pub mod sqrt;
pub mod exp;
pub mod rounding;

pub use fixed::Fixed;
pub use constants::{
    SCALE, FIXED_E, MAX_TOTAL_COUPLING, MAX_FIXED, MIN_FIXED,
    MIN_LEGITIMACY, MAX_LEGITIMACY, div_floor, mod_floor
};
pub use sqrt::f_sqrt;
pub use exp::f_exp;
pub use rounding::round_half_to_even;
