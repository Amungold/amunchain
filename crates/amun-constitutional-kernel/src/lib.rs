// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

//! Constitutional Execution Kernel
//!
//! This crate provides the minimal deterministic runtime that enforces
//! constitutional authority during state transitions.  Every operation
//! that may affect constitutional reality MUST pass through this kernel.
//!
//! # Design principles
//! - Capability-gated execution: no action without a valid capability.
//! - Deterministic state machine: same inputs produce identical outputs.
//! - Replay-safe receipts: every transition leaves a verifiable proof.
//! - Kernel sovereignty boundary: nothing may bypass constitutional checks.

pub mod context;
pub mod enforcer;
pub mod state_machine;
pub mod amendment;
pub mod receipt;

pub use context::ExecutionContext;
pub use enforcer::CapabilityEnforcer;
pub use state_machine::ConstitutionalStateMachine;
pub use amendment::AmendmentActivator;
pub use receipt::ExecutionReceipt;
