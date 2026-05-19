#[derive(Debug, Clone)]
pub struct LockViolation {
    pub validator_id: u64,
    pub violation_type: LockViolationType,
    pub round: u64,
    pub evidence: LockViolationEvidence,
}

#[derive(Debug, Clone)]
pub enum LockViolationType {
    LockRegression,
    ConflictingLock,
    PrevoteViolatesLock,
    PrecommitViolatesLock,
}

#[derive(Debug, Clone)]
pub struct LockViolationEvidence {
    pub locked_round: u64,
    pub locked_value: Option<[u8; 32]>,
    pub attempted_round: u64,
    pub attempted_value: Option<[u8; 32]>,
}
