use super::diagnostics::Diagnostic;
use super::report::{VerificationStatus, VerificationValue};

/// نتيجة مجردة من أي سياق، ينتجها invariant
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub status: VerificationStatus,
    pub computed: VerificationValue,
    pub expected: VerificationValue,
    pub diagnostics: Vec<Diagnostic>,
}
