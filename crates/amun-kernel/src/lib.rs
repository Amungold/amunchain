//! # Amun Constitutional Kernel
//!
//! This crate defines the **sovereign protocol primitives** that
//! all other layers depend upon. It owns:
//! - Canonical encoding traits
//! - Domain separation constants
//! - Protocol versioning

pub mod canonical;
pub mod hashing;
pub mod version;

// Re-exports
pub use canonical::{CanonicalEncode, CanonicalEncoder};
pub use hashing::domain_tags;
pub use version::PROTOCOL_VERSION;
