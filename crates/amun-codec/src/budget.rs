// Decode budget for resource accounting during deserialization.

use amun_failure::{module_ids, operation_ids, AmunResult, ConstitutionalFault, FailureContext};

#[derive(Clone, Debug)]
pub struct DecodeBudget {
    bytes_remaining: u32,
    objects_remaining: u32,
    depth_remaining: u8,
}

impl DecodeBudget {
    pub fn new(max_bytes: u32, max_objects: u32, max_depth: u8) -> Self {
        Self {
            bytes_remaining: max_bytes,
            objects_remaining: max_objects,
            depth_remaining: max_depth,
        }
    }

    pub fn consume_bytes(&mut self, count: u32) -> AmunResult<()> {
        self.bytes_remaining = self.bytes_remaining.checked_sub(count).ok_or_else(|| {
            FailureContext::new(
                ConstitutionalFault::DecodeBudgetExceeded,
                module_ids::AMUN_CODEC,
                operation_ids::BUDGET_BYTES,
            )
        })?;
        Ok(())
    }

    pub fn consume_object(&mut self) -> AmunResult<()> {
        self.objects_remaining = self.objects_remaining.checked_sub(1).ok_or_else(|| {
            FailureContext::new(
                ConstitutionalFault::DecodeBudgetExceeded,
                module_ids::AMUN_CODEC,
                operation_ids::BUDGET_OBJECT,
            )
        })?;
        Ok(())
    }

    pub fn enter_nested(&mut self) -> AmunResult<NestingGuard<'_>> {
        self.depth_remaining = self.depth_remaining.checked_sub(1).ok_or_else(|| {
            FailureContext::new(
                ConstitutionalFault::DecodeBudgetExceeded,
                module_ids::AMUN_CODEC,
                operation_ids::BUDGET_DEPTH,
            )
        })?;
        Ok(NestingGuard { budget: self })
    }
}

pub struct NestingGuard<'a> {
    budget: &'a mut DecodeBudget,
}

impl<'a> Drop for NestingGuard<'a> {
    fn drop(&mut self) {
        self.budget.depth_remaining = self.budget.depth_remaining.saturating_add(1);
    }
}
