// Canonical deterministic serialization.
// Single source of truth: CanonicalWriter trait.

#![no_std]

pub mod budget;
pub mod canonical_sort;
pub mod containers;
pub mod decode;
pub mod domain;
pub mod encode;
pub mod hash;
pub mod transducer;
pub mod versioned;
pub mod writer;

pub use budget::DecodeBudget;
pub use canonical_sort::{canonical_sort, compare_by_canonical_bytes, CanonicalSortKey};
pub use containers::{
    encode_map, encode_sequence, encode_set, DuplicatePolicy, CONSTITUTIONAL_DUPLICATE_POLICY,
};
pub use decode::CanonicalDecode;
pub use domain::HashDomain;
pub use encode::CanonicalEncode;
pub use hash::hash_value_streaming;
pub use transducer::{CanonicalCursor, CanonicalTransducer, SliceCursor, U64Cursor};
pub use versioned::{ProtocolHeader, CURRENT_ENCODING_VERSION};
pub use writer::{BufferWriter, CanonicalWriter, HasherWriter, ShadowBufferWriter, WriteResult};
#[cfg(test)]
mod tests;
