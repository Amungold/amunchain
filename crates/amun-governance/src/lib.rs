#![no_std] #![deny(clippy::unwrap_used)]
pub mod proposal; pub use proposal::{Proposal, ProposalType, ProposalStatus};
#[cfg(test)] mod tests;
