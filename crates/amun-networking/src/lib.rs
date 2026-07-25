pub mod codec;
pub mod envelope;
pub mod message;
pub mod peer;
pub mod peer_identity;
pub mod signed_envelope;
pub mod sync_protocol;
pub mod tcp_transport;
pub mod transport_trait;

pub mod crypto_identity;
pub mod peer_discovery;
pub mod transaction_message;
pub mod trust_anchor;
pub mod validator_certificate;
pub mod validator_registry;

pub mod legacy;
pub use legacy::node::{NetworkNode, NodeLifecycle};

// ------------------------------------------------------------------
// Temporary compatibility layer for legacy simulation binaries.
// Remove after migrating all src/bin/test_* to LiveValidator.
// ------------------------------------------------------------------
pub mod node {
    pub use crate::legacy::node::*;
}
