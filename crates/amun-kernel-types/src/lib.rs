#![cfg_attr(not(test), deny(clippy::unwrap_used))]
// Sovereign kernel primitive types.
// Every type is a newtype wrapper — no bare type aliases.
// Constitutional capacity constants are defined here to avoid
// circular dependencies between amun-codec and amun-constitution.
#![no_std]

pub mod amounts;
pub mod capacity;
pub mod crypto;
pub mod epoch;
pub mod hash;
pub mod identity;
pub mod round;

pub use amounts::{Amount, BlockHeight, ChainId, Gas, Nonce};
pub use capacity::constitutional_capacity;
pub use crypto::{PublicKey, Signature};
pub use epoch::Epoch;
pub use hash::{
    BlockDomain, BlockHash, CommitmentHash32, EpochBoundaryDomain, Hash, JournalEntryDomain,
    PublicHash32, QcHash, QuorumCertificateDomain, SecretHash32, StateCommitment,
    StateCommitmentDomain, TransactionDomain, TxHash, ValidatorSetDomain, VoteDomain, VoteHash,
};
pub use identity::ValidatorId;
pub use round::Round;
#[cfg(test)]
mod tests;
