#![no_std]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]

pub mod nonce;
pub mod receipt;
pub mod state;
pub mod stf;
pub mod transition;

pub use nonce::NonceStore;
pub use receipt::ExecutionReceipt;
pub use state::{InMemoryState, StateStore};
pub use transition::StateTransition;

#[cfg(test)]
mod tests;
