#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::indexing_slicing)]

pub mod engine;
pub mod liveness;
pub mod proposal;
pub mod qc;
pub mod round;
pub mod safety;
pub mod signature_verifier;
pub mod validator;
pub mod vote;
pub mod vote_tracker;

pub use engine::ConsensusEngine;
pub use liveness::LivenessRules;
pub use proposal::BlockProposal;
pub use qc::QuorumCert;
pub use round::RoundState;
pub use safety::SafetyRules;
pub use signature_verifier::SignatureVerifier;
pub use validator::ValidatorSet;
pub use vote::ConsensusVote;
pub use vote_tracker::VoteTracker;

#[cfg(test)]
mod tests;
