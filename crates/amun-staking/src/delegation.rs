use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
pub struct DelegationManager { pub total: u64 }
impl DelegationManager { pub fn new() -> Self { Self { total: 0 } } pub fn delegate(&mut self, s: &mut u64, a: u64) -> AmunResult<()> { *s = s.checked_add(a).ok_or_else(|| FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0013, 0x0010))?; self.total += a; Ok(()) } }
