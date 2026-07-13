#![forbid(unsafe_code)]

pub mod event;
pub mod state;
pub mod transition;
pub mod scheduler;
pub mod log;
pub mod snapshot;
pub mod receipt;
pub mod certification;
pub mod ordering;

pub const TRANSITION_VERSION: u32 = 1;
pub const STATE_LAYOUT_VERSION: u32 = 1;
pub const RECEIPT_LAYOUT_VERSION: u32 = 1;

pub use event::{Event, EventType};
pub use state::{ConstitutionalState, Account};
pub use transition::{TransitionEngine, TransitionResult};
pub use scheduler::DeterministicScheduler;
pub use log::{CausalLog, LogEntry};
pub use snapshot::{StateSnapshot, SnapshotManager};
pub use receipt::{ExecutionReceipt, ExecutionReceipts, ErrorCode};
pub use certification::{ExecutionCertificate, ReplayCertificate, compute_execution_fingerprint};
pub use ordering::CanonicalOrdering;
