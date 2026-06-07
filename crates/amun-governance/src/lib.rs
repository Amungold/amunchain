#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]
#![no_std]
#![deny(clippy::unwrap_used)]
pub mod proposal;
pub use proposal::{Proposal, ProposalStatus, ProposalType};
#[cfg(test)]
mod tests;
