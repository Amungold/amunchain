#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PublicKey(pub [u8; 48]);

impl PublicKey {
    pub const fn new(bytes: [u8; 48]) -> Self {
        Self(bytes)
    }
}

impl Default for PublicKey {
    fn default() -> Self {
        Self([0u8; 48])
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Signature(pub [u8; 96]);

impl Signature {
    pub const fn new(bytes: [u8; 96]) -> Self {
        Self(bytes)
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self([0u8; 96])
    }
}
