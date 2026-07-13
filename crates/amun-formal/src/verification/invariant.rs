use super::result::VerificationResult;
use super::context::VerificationContext;
use super::category::VerificationCategory;
use super::priority::VerificationPriority;
use super::stage::VerificationStage;

pub trait Invariant: Send + Sync {
    fn name(&self) -> &'static str;
    fn category(&self) -> VerificationCategory;
    fn priority(&self) -> VerificationPriority;
    /// المراحل التي يُنفذ عليها هذا الـ invariant
    fn stages(&self) -> &'static [VerificationStage];
    /// ينفذ التحقق ويعيد نتيجة مجردة
    fn verify(&self, ctx: &VerificationContext) -> VerificationResult;
}
