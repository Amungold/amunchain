#![no_std]
#![deny(clippy::unwrap_used)]
pub mod proposal;
pub use proposal::{Proposal, ProposalStatus, ProposalType};
#[cfg(test)]
mod tests;
