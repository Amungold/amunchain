// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

//! Constitutional governance primitives.
//!
//! This crate provides the building blocks for replay-safe,
//! capability-based governance on top of the constitutional
//! authority layer.  The central abstraction is a *capability*:
//! a cryptographically provable right to perform a specific
//! action within a bounded scope.
//!
//! Delegation, voting, and amendment semantics are all expressed
//! as specialisations of capabilities, keeping the governance
//! model formal and replay-verifiable.

pub mod capability;
pub mod delegation;
pub mod quorum;
pub mod voting;
pub mod amendment;

pub use capability::{Capability, CapabilityCertificate};
pub use delegation::DelegateCertificate;
pub use quorum::QuorumPolicy;
pub use voting::{Proposal, Ballot, Tally};
pub use amendment::AmendmentLifecycle;
