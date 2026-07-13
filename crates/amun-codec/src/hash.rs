// Canonical hash trait and streaming hash implementation.

use crate::encode::CanonicalEncode;
use amun_failure::AmunResult;
use amun_kernel_types::PublicHash32;

pub trait CanonicalHash {
    fn canonical_hash(&self) -> AmunResult<PublicHash32>;
}

impl<T: CanonicalEncode> CanonicalHash for T {
    fn canonical_hash(&self) -> AmunResult<PublicHash32> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_CANONICAL_HASH_V1:");
        self.encode_into_hasher(&mut hasher)?;
        Ok(PublicHash32::new(hasher.finalize().into()))
    }
}

pub fn hash_value_streaming<T: CanonicalEncode>(value: &T) -> AmunResult<PublicHash32> {
    value.canonical_hash()
}
