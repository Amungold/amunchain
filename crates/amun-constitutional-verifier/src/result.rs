#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationFailure {
    InvalidSignature { validator_id: u64 },
    DuplicateValidator { validator_id: u64 },
    UnknownValidator { validator_id: u64 },
    PhaseMismatch,
    RoundMismatch,
    PositionMismatch,
    BlockHashMismatch,
    InsufficientQuorum { have: u64, need: u64 },
    QcHashInvalid,
    VoteHashInvalid,
    CanonicalOrderingViolated,
}

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub valid: bool,
    pub total_weight: u64,
    pub quorum_reached: bool,
    pub failures: Vec<VerificationFailure>,
    pub validator_count: usize,
}

impl VerificationResult {
    pub fn success(total_weight: u64, validator_count: usize) -> Self {
        Self {
            valid: true,
            total_weight,
            quorum_reached: true,
            failures: Vec::new(),
            validator_count,
        }
    }

    pub fn failure(failures: Vec<VerificationFailure>) -> Self {
        Self {
            valid: false,
            total_weight: 0,
            quorum_reached: false,
            failures,
            validator_count: 0,
        }
    }
}
