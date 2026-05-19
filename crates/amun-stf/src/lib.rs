#![no_std]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]

pub mod state;
pub mod transition;
pub mod receipt;
pub mod stf;
pub mod nonce;

pub use state::{StateStore, InMemoryState};
pub use transition::StateTransition;
pub use receipt::ExecutionReceipt;
pub use nonce::NonceStore;

#[cfg(test)]
mod tests;
