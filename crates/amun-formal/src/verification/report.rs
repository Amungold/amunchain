use super::diagnostics::Diagnostic;
use super::category::VerificationCategory;
use super::stage::VerificationStage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationValue {
    U64(u64),
    I64(i64),
    Bool(bool),
    Hash([u8; 32]),
    Bytes(Vec<u8>),
    Text(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    Passed,
    Warning,
    Failed,
    Skipped,
    Error,
}

/// تقرير مكتمل يُبنى بواسطة المحرك
#[derive(Debug, Clone)]
pub struct VerificationReport {
    pub invariant_name: &'static str,
    pub category: VerificationCategory,
    pub status: VerificationStatus,
    pub computed: VerificationValue,
    pub expected: VerificationValue,
    pub diagnostics: Vec<Diagnostic>,
    pub duration_ns: u64,
    pub stage: VerificationStage,
    pub block_height: u64,
    pub epoch: u64,
    pub state_root: [u8; 32],
    pub chain_id: u32,
}
