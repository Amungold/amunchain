//! Constitutional version framing for all root objects
//! Every serialized root object MUST start with CANONICAL_VERSION

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

use crate::canonical::{CanonicalEncoder, CanonicalDecoder};

/// Constitutional canonical version
///
/// Version history:
/// - v1: Hash had length prefix (33 bytes)
/// - v2: Fixed hash is raw 32 bytes (no length prefix)
pub const CANONICAL_VERSION: u16 = 2;

/// Versioned root object wrapper
/// All consensus-critical root objects MUST use this framing
///
/// IMPORTANT: Do NOT directly implement CanonicalSerialize for root objects.
/// Always use VersionedRoot for external serialization.
pub trait VersionedRoot: Sized {
    /// Encode with version prefix (use this for external serialization)
    fn encode_versioned(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_u16(CANONICAL_VERSION);
        self.encode_content(encoder);
    }

    /// Encode content without version (internal use only)
    fn encode_content(&self, encoder: &mut CanonicalEncoder);

    /// Decode with version prefix and exhaustion check
    fn decode_versioned(decoder: &mut CanonicalDecoder) -> Option<Self> {
        let version = decoder.read_u16()?;
        if version != CANONICAL_VERSION {
            return None;
        }
        let obj = Self::decode_content(decoder)?;
        if !decoder.is_exhausted() {
            return None;
        }
        Some(obj)
    }

    /// Decode content without version (internal use only)
    fn decode_content(decoder: &mut CanonicalDecoder) -> Option<Self>;

    /// Convert to versioned bytes (external API)
    fn to_versioned_bytes(&self) -> Vec<u8> {
        let mut encoder = CanonicalEncoder::new();
        self.encode_versioned(&mut encoder);
        encoder.into_bytes()
    }

    /// Convert from versioned bytes (external API)
    fn from_versioned_bytes(bytes: &[u8]) -> Option<Self> {
        let mut decoder = CanonicalDecoder::new(bytes);
        Self::decode_versioned(&mut decoder)
    }
}

/// Marker trait for root objects that MUST be versioned
pub trait ConstitutionalRoot: VersionedRoot {}

// Forward declarations - implementations are in their respective modules
// AmunState implements VersionedRoot in state/mod.rs
// ExecutionWitness implements VersionedRoot in constitutional/transition.rs
