#![no_std]
#![deny(clippy::unwrap_used)]

pub mod law;
pub mod recovery;
pub mod snapshot;
pub mod store;
pub mod wal;

pub use law::StorageLaw;
pub use recovery::RecoveryEngine;
pub use snapshot::StateSnapshot;
pub use store::PersistentStore;
pub use wal::{WalPayload, WalRecord, WriteAheadLog};

#[cfg(test)]
mod tests;
