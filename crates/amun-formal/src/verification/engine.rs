use super::invariant::Invariant;
use super::state_reader::StateReader;
use super::report::{VerificationReport, VerificationStatus, VerificationValue};
use super::config::VerificationConfig;
use super::registry::InvariantRegistry;
use super::context::VerificationContext;
use super::stage::VerificationStage;
use std::time::Instant;

pub struct InvariantEngine {
    registry: InvariantRegistry,
    config: VerificationConfig,
}

impl InvariantEngine {
    pub fn new(registry: InvariantRegistry, config: VerificationConfig) -> Self {
        Self { registry, config }
    }

    /// تشغيل جميع الـ invariants المناسبة للمرحلة
    pub fn run_stage(
        &self,
        stage: VerificationStage,
        state: &dyn StateReader,
    ) -> Vec<VerificationReport> {
        let (block_height, epoch, state_root, chain_id) = (
            state.block_height(),
            state.epoch(),
            state.state_root().unwrap_or([0u8; 32]),
            state.chain_id(),
        );

        let ctx = VerificationContext {
            state,
            block_height,
            epoch,
            state_root,
            chain_id,
            stage,
        };

        let grouped = self.registry.grouped_by_category();
        let mut reports = Vec::new();

        for (_category, invariants) in &grouped {
            let mut category_failed = false;
            for inv in invariants {
                // تطبيق المراحل المسموحة
                if !inv.stages().contains(&stage) {
                    continue;
                }

                if self.config.fail_fast && category_failed {
                    reports.push(Self::skipped_report(inv, &ctx));
                    continue;
                }

                let start = Instant::now();
                let result = inv.verify(&ctx);
                let elapsed = start.elapsed().as_nanos() as u64;

                let report = VerificationReport {
                    invariant_name: inv.name(),
                    category: inv.category(),
                    status: result.status,
                    computed: result.computed,
                    expected: result.expected,
                    diagnostics: if self.config.record_reports {
                        result.diagnostics
                    } else {
                        Vec::new()
                    },
                    duration_ns: elapsed,
                    stage,
                    block_height,
                    epoch,
                    state_root,
                    chain_id,
                };

                if report.status == VerificationStatus::Failed
                    || report.status == VerificationStatus::Error
                {
                    category_failed = true;
                }

                reports.push(report);
            }
        }

        reports
    }

    fn skipped_report(inv: &dyn Invariant, ctx: &VerificationContext) -> VerificationReport {
        VerificationReport {
            invariant_name: inv.name(),
            category: inv.category(),
            status: VerificationStatus::Skipped,
            computed: VerificationValue::Text("skipped".into()),
            expected: VerificationValue::Text("skipped".into()),
            diagnostics: Vec::new(),
            duration_ns: 0,
            stage: ctx.stage,
            block_height: ctx.block_height,
            epoch: ctx.epoch,
            state_root: ctx.state_root,
            chain_id: ctx.chain_id,
        }
    }
}
