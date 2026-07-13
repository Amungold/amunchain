#[derive(Debug, Clone)]
pub struct VerificationConfig {
    pub fail_fast: bool,
    pub run_parallel: bool,     // غير مفعل بعد
    pub record_reports: bool,
    pub max_duration_ms: u64,   // محجوز للاستخدام المستقبلي
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            fail_fast: false,
            run_parallel: false,
            record_reports: true,
            max_duration_ms: 500,
        }
    }
}
