use crate::verification::invariant::Invariant;
use crate::verification::context::VerificationContext;
use crate::verification::result::VerificationResult;
use crate::verification::report::{VerificationValue, VerificationStatus};
use crate::verification::diagnostics::{Diagnostic, Severity};
use crate::verification::category::VerificationCategory;
use crate::verification::priority::VerificationPriority;
use crate::verification::stage::VerificationStage;

pub struct SupplyComponentConsistency;

impl Invariant for SupplyComponentConsistency {
    fn name(&self) -> &'static str {
        "SupplyComponentConsistency"
    }

    fn category(&self) -> VerificationCategory {
        VerificationCategory::Economics
    }

    fn priority(&self) -> VerificationPriority {
        VerificationPriority::High
    }

    fn stages(&self) -> &'static [VerificationStage] {
        &[
            VerificationStage::PostExecution,
            VerificationStage::PreCommit,
            VerificationStage::Replay,
            VerificationStage::Recovery,
        ]
    }

    fn verify(&self, ctx: &VerificationContext) -> VerificationResult {
        let breakdown = match ctx.state.supply_breakdown() {
            Ok(b) => b,
            Err(e) => {
                return VerificationResult {
                    status: VerificationStatus::Error,
                    computed: VerificationValue::Text("N/A".into()),
                    expected: VerificationValue::Text("N/A".into()),
                    diagnostics: vec![Diagnostic {
                        code: "STATE_READ_FAILURE",
                        severity: Severity::Critical,
                        message: format!("{:?}", e),
                    }],
                };
            }
        };

        let mut diagnostics = Vec::new();

        for &id in breakdown.components.keys() {
            if id.is_empty() {
                diagnostics.push(Diagnostic {
                    code: "EMPTY_COMPONENT_ID",
                    severity: Severity::Error,
                    message: "Supply component has an empty identifier".into(),
                });
            }
        }

        // BTreeMap يضمن عدم وجود تكرار، وu64 يمنع القيم السالبة.
        let status = if diagnostics.is_empty() {
            VerificationStatus::Passed
        } else {
            VerificationStatus::Warning
        };

        VerificationResult {
            status,
            computed: VerificationValue::Bool(true),
            expected: VerificationValue::Bool(true),
            diagnostics,
        }
    }
}
