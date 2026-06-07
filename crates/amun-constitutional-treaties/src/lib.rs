#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
pub mod covenant;
pub mod treaty;

pub use covenant::InteroperabilityCovenant;
pub use treaty::{ConstitutionalTreaty, TreatyStatus, TreatyType};
