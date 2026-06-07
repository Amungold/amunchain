//! Constitutional Peer Identity
//!
//! This crate defines what it means to be a sovereign actor inside
//! the AmunChain constitutional civilisation.  A peer is not merely
//! a network endpoint — it is a lineage-bound constitutional entity
//! whose identity is cryptographically provable.

pub mod peer_id;
pub mod certificate;
pub mod registry;
pub mod verifier;

pub use peer_id::ConstitutionalPeerId;
pub use certificate::PeerCertificate;
pub use registry::PeerRegistry;
pub use verifier::IdentityVerifier;
