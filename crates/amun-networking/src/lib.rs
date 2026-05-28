// Constitutional Networking Layer - Orchestration Only
// All constitutional semantics are delegated to dedicated crates.
// This crate ONLY orchestrates: amun-civilizational-relations,
// amun-constitutional-quarantine, amun-temporal-alignment,
// amun-constitutional-treaties.

pub mod messages;
pub mod peers;
pub mod sync;

pub use messages::{ConstitutionalMessage, MessageType};
pub use peers::{PeerIdentity, PeerTrust, TrustLevel};
pub use sync::{SyncEngine, SyncResult, SyncState};
