#![no_std]
#![deny(clippy::unwrap_used)]
pub mod token; pub mod fee; pub mod reward; pub mod inflation; pub mod treasury; pub mod constants;
pub use token::Token; pub use fee::FeeMarket; pub use reward::RewardDistributor; pub use inflation::InflationCurve; pub use treasury::Treasury; pub use constants::*;
#[cfg(test)] mod tests;
