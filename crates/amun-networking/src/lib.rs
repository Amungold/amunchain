#![allow(clippy::uninlined_format_args)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::no_effect_underscore_binding)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::stable_sort_primitive)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::similar_names)]
#![allow(clippy::float_cmp)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::unused_self)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::new_without_default)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_borrows_for_generic_args)]

pub mod codec;
pub mod crypto_identity;
pub mod envelope;
pub mod frame;
pub mod genesis_authority;
pub mod message;
pub mod node;
pub mod peer;
pub mod peer_discovery;
pub mod peer_identity;
pub mod signed_envelope;
pub mod sync_frames;
pub mod sync_protocol;
pub mod tcp_transport;
pub mod transport;
pub mod transport_trait;
pub mod trust_anchor;
pub mod validator_certificate;
pub mod validator_registry;

// New N102.4 modules
pub mod backpressure;
pub mod capability_enforcement;
pub mod connection_state;
pub mod frame_codec;
pub mod global_rate_limiter;
pub mod handshake;
pub mod priority_queue;

pub use node::{NetworkNode, NodeLifecycle};
// TODO: Migrate PeerId from amun_identity_core
pub mod payload;
pub mod rtt_tracker;
pub mod session_manager;
