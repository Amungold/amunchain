#![no_std]
#![deny(clippy::unwrap_used)]

pub mod tx;
pub mod limits;

pub use tx::{UnsignedTransaction, TransactionType};
pub use limits::{MAX_TX_BYTES_WIRE, MAX_PAYLOAD_BYTES_RUNTIME, MIN_VERSION};

#[cfg(test)]
mod tests;
