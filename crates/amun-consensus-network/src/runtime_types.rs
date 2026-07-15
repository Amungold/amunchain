//! Runtime consensus types — compose protocol types (amun-consensus-types)
//! with execution metadata needed by the consensus engine.
//!
//! N144.2A: Introduce RuntimeVote alongside existing ConsensusVote.
//! Once all consumers migrate, ConsensusVote will be removed.

use crate::execution_commitment::ExecutionCommitment;
use amun_consensus_types::Vote;

/// A vote with runtime metadata (signature, state commitment, timestamp).
///
/// Composes the protocol-level Vote with execution-context fields
/// that the consensus engine needs for verification and bookkeeping.
#[derive(Debug, Clone)]
pub struct RuntimeVote {
    /// The protocol vote (phase, epoch, round, block_hash, validator_index, validator_id)
    pub protocol: Vote,
    /// Ed25519 signature over the vote payload
    pub signature: [u8; 64],
    /// State root at the time of voting (execution context)
    pub state_root: [u8; 32],
    /// Wall-clock timestamp when the vote was created
    pub timestamp: u64,
    /// N109.8: Cryptographic execution commitment (optional)
    pub commitment: Option<ExecutionCommitment>,
}
// ═══════════════════════════════════════════════════════════════
// N144.2B: Temporary migration adapters.
//
// NOTE: These conversions are INTENTIONALLY LOSSY.
// ConsensusVote does not carry the full protocol information
// required by amun_consensus_types::Vote (phase, round, validator_index).
// Placeholder values are used during migration.
//
// These adapters exist ONLY during N144 migration and MUST be
// removed once RuntimeVote becomes the primary runtime representation.
// ═══════════════════════════════════════════════════════════════

use crate::messages::ConsensusVote;
use amun_consensus_types::ConsensusPhase;
use amun_kernel_types::{BlockHash, Epoch, ValidatorId};

impl From<ConsensusVote> for RuntimeVote {
    fn from(cv: ConsensusVote) -> Self {
        RuntimeVote {
            protocol: Vote {
                phase: ConsensusPhase::CommitVote,
                epoch: Epoch::new(cv.height),
                round: amun_consensus_types::ConsensusRound::new(0),
                block_hash: BlockHash::new(cv.block_hash),
                validator_index: amun_consensus_types::ValidatorIndex(0),
                validator_id: ValidatorId::new(cv.voter_id),
            },
            signature: cv.signature,
            state_root: cv.state_root,
            timestamp: cv.timestamp,
            commitment: cv.commitment,
        }
    }
}

impl From<RuntimeVote> for ConsensusVote {
    fn from(rv: RuntimeVote) -> Self {
        let hash_bytes: [u8; 32] = rv
            .protocol
            .block_hash
            .as_bytes()
            .try_into()
            .unwrap_or([0u8; 32]);
        ConsensusVote {
            voter_id: rv.protocol.validator_id.0,
            height: rv.protocol.epoch.value(),
            block_hash: hash_bytes,
            state_root: rv.state_root,
            approve: matches!(
                rv.protocol.phase,
                ConsensusPhase::CommitVote | ConsensusPhase::PreCommitVote
            ),
            signature: rv.signature,
            timestamp: rv.timestamp,
            commitment: rv.commitment,
        }
    }
}
