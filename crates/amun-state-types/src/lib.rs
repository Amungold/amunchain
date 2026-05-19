#![cfg_attr(not(test), deny(clippy::unwrap_used))]
// Structural state legality. Three-dimensional state tracking:
// Structural: Unverified -> Verified -> Committed -> Finalized
// Durability: Volatile -> Durable -> Journaled -> Snapshotted
// Consensus:  Proposed -> Voted -> QuorumCertified -> Executed
#![no_std]

pub mod states;
pub mod transitions;

pub use states::*;
pub use transitions::State;
#[cfg(test)]
mod tests;
