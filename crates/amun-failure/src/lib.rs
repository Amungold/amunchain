#![cfg_attr(not(test), deny(clippy::unwrap_used))]
// Unified failure taxonomy. Every error in the sovereign kernel flows through here.
// No String errors. No Box<dyn Error>. No panic-based error handling.
#![no_std]

pub mod kernel_state;
pub mod taxonomy;

pub use kernel_state::{KernelHealth, KernelState, QuarantineActions};
pub use taxonomy::{
    module_ids, operation_ids, AmunResult, ConstitutionalFault, FailureContext, FaultSeverity,
    Subsystem,
};
#[cfg(test)]
mod tests;
