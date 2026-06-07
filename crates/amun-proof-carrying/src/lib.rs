// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

//! Proof-carrying execution receipts.
//!
//! This crate transforms plain ExecutionReceipts into portable
//! constitutional proof objects that can be verified independently
//! without a trusted node or full chain replay.

pub mod receipt;
pub mod verifier;

pub use receipt::ProofCarryingReceipt;
pub use verifier::ProofVerifier;
