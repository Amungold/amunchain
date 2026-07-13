#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidatorId(pub [u8; 32]);

impl ValidatorId {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}
