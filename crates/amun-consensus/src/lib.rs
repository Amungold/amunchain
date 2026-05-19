#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::indexing_slicing)]

pub mod engine;
pub mod proposal;
pub mod vote;
pub mod qc;
pub mod round;
pub mod validator;
pub mod safety;
pub mod liveness;
pub mod signature_verifier;
pub mod vote_tracker;

pub use engine::ConsensusEngine;
pub use proposal::BlockProposal;
pub use vote::ConsensusVote;
pub use qc::QuorumCert;
pub use round::RoundState;
pub use validator::ValidatorSet;
pub use safety::SafetyRules;
pub use liveness::LivenessRules;
pub use signature_verifier::SignatureVerifier;
pub use vote_tracker::VoteTracker;

#[cfg(test)]
mod tests;
