// ============================================================================
// ADR-021: Consensus Event Definitions
// Pure events - no timestamps, no diagnostics
// ============================================================================

use serde::{Deserialize, Serialize};

/// Unique identifier for each event in the consensus lifecycle
pub type EventId = u64;

/// Correlation ID to track a round across view changes/retries
pub type RoundCorrelationId = u64;

/// Logical height (block height)
pub type Height = u64;

/// Round number within a height
pub type Round = u32;

/// Validator identifier
pub type ValidatorId = u64;

/// Sequence number for deterministic replay
pub type SequenceNumber = u64;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusEvent {
    /// Unique event identifier (monotonically increasing)
    pub event_id: EventId,

    /// Parent event that triggered this event (for traceability)
    pub parent_event_id: Option<EventId>,

    /// Correlation ID for the round (survives view changes)
    pub round_correlation_id: RoundCorrelationId,

    /// Monotonically increasing sequence number (for replay determinism)
    pub sequence: SequenceNumber,

    /// Logical height at the time of the event
    pub logical_height: Height,

    /// The actual event data
    pub event: ConsensusEventKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusEventKind {
    // ==================== Bootstrap ====================
    ValidatorRegistered {
        validator_id: ValidatorId,
    },

    // ==================== Leader Selection ====================
    LeaderSelected {
        height: Height,
        leader_id: ValidatorId,
        leader_index: usize,
    },

    // ==================== Round Lifecycle ====================
    RoundStarted {
        height: Height,
        round: Round,
        validator_id: ValidatorId,
        my_index: usize,
        proposer_index: usize,
        is_proposer: bool,
    },

    // ==================== Proposal Lifecycle ====================
    ProposalCreated {
        height: Height,
        proposer_id: ValidatorId,
        block_hash: [u8; 32],
    },
    ProposalBroadcast {
        height: Height,
        from: ValidatorId,
    },
    ProposalReceived {
        height: Height,
        from: ValidatorId,
        to: ValidatorId,
    },
    ProposalValidated {
        height: Height,
        valid: bool,
    },
    ProposalRejected {
        height: Height,
        reason: ProposalRejectionReason,
    },

    // ==================== Voting Lifecycle ====================
    VoteSent {
        height: Height,
        from: ValidatorId,
        to: ValidatorId,
    },
    VoteReceived {
        height: Height,
        from: ValidatorId,
        to: ValidatorId,
        valid: bool,
    },

    // ==================== Finality ====================
    QuorumCertificateFormed {
        height: Height,
        num_signatures: usize,
    },
    BlockFinalized {
        height: Height,
        block_hash: [u8; 32],
    },

    // ==================== Failures (Safety/Liveness) ====================
    LeaderTimeout {
        height: Height,
        expected_leader: ValidatorId,
    },
    RoundTimeout {
        height: Height,
        round: Round,
    },
    VoteRejected {
        height: Height,
        reason: VoteRejectionReason,
    },
    DoubleVote {
        height: Height,
        validator: ValidatorId,
    },
    DoubleProposal {
        height: Height,
        validator: ValidatorId,
    },

    // ==================== View Change ====================
    ViewChangeStarted {
        height: Height,
        new_round: Round,
    },
    ViewChangeCompleted {
        height: Height,
        new_round: Round,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalRejectionReason {
    InvalidSignature,
    InvalidProposer,
    InvalidBlockHash,
    DuplicateProposal,
    FutureHeight,
    PastHeight,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteRejectionReason {
    InvalidSignature,
    InvalidVoter,
    InvalidBlockHash,
    DuplicateVote,
    RoundMismatch,
    Other(String),
}

impl ConsensusEvent {
    /// Create a new event with auto-incrementing sequence
    pub fn new(
        event_id: EventId,
        parent_event_id: Option<EventId>,
        round_correlation_id: RoundCorrelationId,
        sequence: SequenceNumber,
        logical_height: Height,
        event: ConsensusEventKind,
    ) -> Self {
        Self {
            event_id,
            parent_event_id,
            round_correlation_id,
            sequence,
            logical_height,
            event,
        }
    }

    /// Check if this is a safety violation
    pub fn is_safety_violation(&self) -> bool {
        matches!(
            &self.event,
            ConsensusEventKind::DoubleVote { .. } | ConsensusEventKind::DoubleProposal { .. }
        )
    }

    /// Check if this is a liveness issue
    pub fn is_liveness_issue(&self) -> bool {
        matches!(
            &self.event,
            ConsensusEventKind::LeaderTimeout { .. }
                | ConsensusEventKind::RoundTimeout { .. }
                | ConsensusEventKind::ProposalRejected { .. }
                | ConsensusEventKind::VoteRejected { .. }
        )
    }
}
