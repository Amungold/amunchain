#![forbid(unsafe_code)]

pub mod constants;
pub mod exp;
pub mod fixed;
pub mod rounding;
pub mod sqrt;

pub use constants::{
    div_floor, mod_floor, FIXED_E, MAX_FIXED, MAX_LEGITIMACY, MAX_TOTAL_COUPLING, MIN_FIXED,
    MIN_LEGITIMACY, SCALE,
};
pub use exp::f_exp;
pub use fixed::Fixed;
pub use rounding::round_half_to_even;
pub use sqrt::f_sqrt;
