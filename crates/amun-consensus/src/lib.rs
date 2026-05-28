//! # AmunChain Consensus Module

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod crypto;
pub mod lineage;
pub mod constitutional_vote;
pub mod aliveness;
pub mod serialization;
pub mod consensus;
pub mod canonical;
pub mod replay;
pub mod constitutional;
pub mod state;
pub mod versioning;

// Explicit exports - constitutional surface
pub use crypto::types::{GenesisIdentity, NodeHash, ConstitutionalEpoch, ConstitutionalRound, ConstitutionalHeight, AuthorityReference, ConstitutionalHash, SignatureBytes};
pub use crypto::signatures::{SignatureAggregator, ValidatorSignature};
pub use crypto::verifier::{SignatureVerifier, PlaceholderVerifier, SigResult};
pub use lineage::node::ImmutableLineageNode;
pub use lineage::graph::LineageGraph;
pub use constitutional_vote::{ConstitutionalVote, ConstitutionalVoteSet, ConstitutionalDecision, AuthorityProof, QuorumAnalysis};
pub use aliveness::attestation::{AlivenessAttestation, ConsensusAliveness};
pub use aliveness::ordering::WeightedAliveness;
pub use consensus::{ByzantineSimulator, QCStore, QC, QCHash, QCStatus, ConsensusState, ValidatorStateMachine, Pacemaker, Round, LeaderSelection, PacemakerConfig, ThreeChainTracker, ForkChoiceEngine, AncestryProofVerifier};
pub use canonical::{CanonicalEncoder, CanonicalDecoder, CanonicalSerialize, CanonicalDeserialize};
pub use replay::{ReplayCertificate, ReplayVerifier};
pub use constitutional::{ConstitutionalHashable, VerifiedTransitionWitness, ExecutionWitness, WitnessChainVerifier, ConstitutionalState, ConstitutionalTransition};
pub use state::AmunState;
pub use versioning::{VersionedRoot, ConstitutionalRoot, CANONICAL_VERSION};

pub const CHAIN_ID: &str = "amun_constitutional";
pub const AMUN_GENESIS_SEED: [u8; 32] = [1u8; 32];

#[derive(Debug, Clone)]
pub struct GenesisConfig {
    pub chain_id: &'static str,
    pub genesis_seed: [u8; 32],
    pub initial_epoch: u64,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            chain_id: CHAIN_ID,
            genesis_seed: AMUN_GENESIS_SEED,
            initial_epoch: 0,
        }
    }
}

pub fn create_genesis_identity(cfg: &GenesisConfig) -> GenesisIdentity {
    GenesisIdentity::compute(cfg.chain_id, cfg.genesis_seed, cfg.initial_epoch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::versioning::VersionedRoot;

    #[test]
    fn test_genesis() {
        let cfg = GenesisConfig::default();
        let id = create_genesis_identity(&cfg);
        let node = ImmutableLineageNode::create_genesis(id);
        assert_ne!(node.as_bytes(), [0u8; 32]);
        assert!(node.is_origin());
    }

    #[test]
    fn test_constitutional_vote() {
        let v = ConstitutionalVote::new(1, 100);
        assert_eq!(v.block_height, 100);
        assert_eq!(v.validator_id, 1);
    }

    #[test]
    fn test_amun_state_versioned_roundtrip() {
        let state = AmunState::genesis();
        let bytes = state.to_versioned_bytes();
        let decoded = AmunState::from_versioned_bytes(&bytes);
        assert_eq!(decoded, Some(state));
    }

    #[test]
    fn test_canonical_version_is_frozen() {
        assert_eq!(CANONICAL_VERSION, 2);
    }
}
