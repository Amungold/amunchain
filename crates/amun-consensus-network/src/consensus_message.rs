use serde::{Deserialize, Serialize};

use crate::messages::{BlockProposal, ConsensusVote, FinalityCertificate, QuorumCertificate};

/// Unified consensus message type.
/// Wraps all consensus-level message variants into a single enum
/// for unified serialization, routing, and processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMessage {
    Proposal(BlockProposal),
    Vote(ConsensusVote),
    QuorumCertificate(QuorumCertificate),
    Finality(FinalityCertificate),
}
