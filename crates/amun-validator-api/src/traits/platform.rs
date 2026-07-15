use crate::error::PlatformResult;
use crate::types::audit::AuditReport;
use crate::types::enrollment::{EnrollmentConfig, EnrollmentResult};
use crate::types::state::RuntimeState;

pub trait ValidatorPlatform: Send + Sync {
    fn enroll(&self, config: EnrollmentConfig) -> PlatformResult<EnrollmentResult>;
    fn verify(&self) -> PlatformResult<AuditReport>;
    fn state(&self) -> PlatformResult<RuntimeState>;
    fn audit(&self) -> PlatformResult<AuditReport>;
}
