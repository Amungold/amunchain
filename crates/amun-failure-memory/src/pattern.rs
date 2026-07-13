#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailureSeverity {
    Fatal,
    Critical,
    Degraded,
    Minor,
}

#[derive(Debug, Clone)]
pub struct FailurePattern {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub severity: FailureSeverity,
    pub provably_mitigated: bool,
    pub has_formal_proof: bool,
    pub mitigation_proof_hash: Option<[u8; 32]>,
    pub invariant_id: u32,
    pub defense_strategy: String,
    pub related_patterns: Vec<u32>,
}
