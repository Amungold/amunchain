use amun_failure::{module_ids, AmunResult, ConstitutionalFault, FailureContext};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epoch(pub u64);

impl Epoch {
    pub const ZERO: Self = Self(0);
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn next(self) -> AmunResult<Self> {
        self.0.checked_add(1).map(Self).ok_or_else(|| {
            FailureContext::new(
                ConstitutionalFault::ArithmeticOverflow,
                module_ids::AMUN_KERNEL_TYPES,
                0x0001,
            )
        })
    }

    pub fn previous(self) -> Self {
        Self(self.0.saturating_sub(1))
    }
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self::ZERO
    }
}
