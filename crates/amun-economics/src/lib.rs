#![no_std]
#![deny(clippy::unwrap_used)]
pub mod constants;
pub mod fee;
pub mod inflation;
pub mod reward;
pub mod token;
pub mod treasury;
pub use constants::*;
pub use fee::FeeMarket;
pub use inflation::InflationCurve;
pub use reward::RewardDistributor;
pub use token::Token;
pub use treasury::Treasury;
#[cfg(test)]
mod tests;
