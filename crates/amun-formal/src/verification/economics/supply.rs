use crate::verification::invariant::Invariant;
use crate::verification::context::VerificationContext;
use crate::verification::result::VerificationResult;
use crate::verification::report::{VerificationValue, VerificationStatus};
use crate::verification::diagnostics::{Diagnostic, Severity};
use crate::verification::category::VerificationCategory;
use crate::verification::priority::VerificationPriority;
use crate::verification::stage::VerificationStage;

pub struct SupplyConservation;

impl Invariant for SupplyConservation {
    fn name(&self) -> &'static str {
        "SupplyConservation"
    }

    fn category(&self) -> VerificationCategory {
        VerificationCategory::Economics
    }

    fn priority(&self) -> VerificationPriority {
        VerificationPriority::Critical
    }

    fn stages(&self) -> &'static [VerificationStage] {
        &[
            VerificationStage::PostExecution,
            VerificationStage::PreCommit,
            VerificationStage::Replay,
            VerificationStage::Recovery,
            VerificationStage::Snapshot,
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

        let computed = breakdown.computed_total();
        let expected = breakdown.total_supply;
        let success = computed == expected;
        let mut diagnostics = Vec::new();

        if !success {
            diagnostics.push(Diagnostic {
                code: "SUPPLY_MISMATCH",
                severity: Severity::Critical,
                message: format!(
                    "Total supply mismatch: computed {}, expected {}",
                    computed, expected
                ),
            });
            for comp in breakdown.component_list() {
                diagnostics.push(Diagnostic {
                    code: "COMPONENT_DETAIL",
                    severity: Severity::Info,
                    message: format!("{}: {}", comp.id, comp.amount),
                });
            }
        }

        VerificationResult {
            status: if success {
                VerificationStatus::Passed
            } else {
                VerificationStatus::Failed
            },
            computed: VerificationValue::U64(computed),
            expected: VerificationValue::U64(expected),
            diagnostics,
        }
    }
}
