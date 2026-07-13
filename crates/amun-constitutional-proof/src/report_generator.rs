use serde::{Deserialize, Serialize};

use crate::{ConstitutionalVerdict, EvidenceArchive, ObligationRegistry, VerdictResult};

/// The output of the report generator, containing both human-readable
/// and machine-readable representations of the full constitutional state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalReport {
    pub project: String,
    pub report_type: String,
    pub generated_at: u64,
    pub total_obligations: usize,
    pub total_evidence: usize,
    pub total_verdicts: usize,
    pub verdicts: Vec<ConstitutionalVerdict>,
}

/// Generates constitutional validation reports from the obligation registry,
/// evidence archive, and verdicts.
pub struct ReportGenerator;

impl ReportGenerator {
    /// Generate the structured report from the three core components.
    pub fn generate_report(
        registry: &ObligationRegistry,
        archive: &EvidenceArchive,
        verdicts: Vec<ConstitutionalVerdict>,
        timestamp: u64,
    ) -> ConstitutionalReport {
        ConstitutionalReport {
            project: "AmunChain".into(),
            report_type: "N47 Constitutional Validation".into(),
            generated_at: timestamp,
            total_obligations: registry.total(),
            total_evidence: archive.total_count(),
            total_verdicts: verdicts.len(),
            verdicts,
        }
    }

    /// Render the report as Markdown.
    pub fn to_markdown(report: &ConstitutionalReport) -> String {
        let mut md = String::new();
        md.push_str("# N47 Constitutional Validation Report\n\n");
        md.push_str(&format!("**Project**: {}\n", report.project));
        md.push_str(&format!("**Report Type**: {}\n", report.report_type));
        md.push_str(&format!("**Generated At**: {}\n\n", report.generated_at));

        md.push_str("## Executive Summary\n\n");
        md.push_str(&format!(
            "- **Total Obligations**: {}\n",
            report.total_obligations
        ));
        md.push_str(&format!(
            "- **Total Evidence Records**: {}\n",
            report.total_evidence
        ));
        md.push_str(&format!(
            "- **Total Verdicts**: {}\n\n",
            report.total_verdicts
        ));

        md.push_str("## Phase Verdicts\n\n");
        for verdict in &report.verdicts {
            let status = match &verdict.overall_result {
                VerdictResult::Pass => "✅ PASS".to_string(),
                VerdictResult::ConditionalPass(conds) => {
                    format!("⚠️ CONDITIONAL PASS ({})", conds.join(", "))
                }
                VerdictResult::Fail(reasons) => {
                    format!("❌ FAIL ({})", reasons.join(", "))
                }
            };
            md.push_str(&format!("### {} — {}\n\n", verdict.phase, status));
            md.push_str(&format!(
                "- **Subject**: {} ({})\n",
                verdict.subject_id, verdict.subject_type
            ));
            md.push_str(&format!(
                "- **Obligations Checked**: {}\n",
                verdict.obligations_checked
            ));
            md.push_str(&format!(
                "- **Obligations Satisfied**: {}\n",
                verdict.obligations_satisfied
            ));
            md.push_str(&format!("- **Failed**: {}\n\n", verdict.failed_count()));
        }

        md
    }

    /// Render the report as JSON.
    pub fn to_json(report: &ConstitutionalReport) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(report)
    }
}
