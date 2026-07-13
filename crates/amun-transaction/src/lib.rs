#![no_std]
#![deny(clippy::unwrap_used)]

pub mod limits;
pub mod tx;

pub use limits::{MAX_PAYLOAD_BYTES_RUNTIME, MAX_TX_BYTES_WIRE, MIN_VERSION};
pub use tx::{TransactionType, UnsignedTransaction};

#[cfg(test)]
mod tests;
