// Constitutional Serialization Lock
// Every constitutional type MUST implement these traits.
// This ensures deterministic encoding across all implementations.

use amun_canonical_codec::{CanonicalReader, CanonicalWriter};

pub trait ConstitutionalEncode {
    fn constitutional_encode(&self, w: &mut CanonicalWriter);
    fn constitutional_decode(r: &mut CanonicalReader) -> Option<Self>
    where
        Self: Sized;
}

/// Macro to derive constitutional encoding for simple types.
#[macro_export]
macro_rules! constitutional_encode_impl {
    ($ty:ty, $encode:expr, $decode:expr) => {
        impl ConstitutionalEncode for $ty {
            fn constitutional_encode(&self, w: &mut CanonicalWriter) {
                $encode(self, w)
            }
            fn constitutional_decode(r: &mut CanonicalReader) -> Option<Self> {
                $decode(r)
            }
        }
    };
}
