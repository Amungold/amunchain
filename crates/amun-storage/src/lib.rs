#![no_std]
#![deny(clippy::unwrap_used)]

pub mod law;
pub mod wal;
pub mod snapshot;
pub mod store;
pub mod recovery;

pub use law::StorageLaw;
pub use wal::{WriteAheadLog, WalRecord, WalPayload};
pub use snapshot::StateSnapshot;
pub use store::PersistentStore;
pub use recovery::RecoveryEngine;

#[cfg(test)]
mod tests;
