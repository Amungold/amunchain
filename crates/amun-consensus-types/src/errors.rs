use amun_failure::{ConstitutionalFault, FailureContext};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ConsensusError {
    InvalidPhase = 0x1001,
    InvalidRound = 0x1002,
    DuplicateVote = 0x1003,
    EquivocationDetected = 0x1004,
    InvalidQuorum = 0x1005,
    InsufficientSigners = 0x1006,
    InvalidBlockHash = 0x1007,
    EpochMismatch = 0x1008,
}

impl ConsensusError {
    pub fn to_failure_context(self) -> FailureContext {
        FailureContext::new(
            ConstitutionalFault::InvalidStateTransition,
            0x0006,
            self as u16,
        )
    }
}
