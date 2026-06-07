// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use serde::{Deserialize, Serialize};

/// A quorum policy defines the threshold rules for a collective decision.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuorumPolicy {
    /// Minimum number of valid ballots required for the vote to be valid.
    pub min_participants: u64,
    /// Fraction of participants that must approve (numerator, denominator).
    /// For example, (2, 3) means two-thirds majority.
    pub approval_numerator: u64,
    pub approval_denominator: u64,
}

impl QuorumPolicy {
    pub fn simple_majority() -> Self {
        Self {
            min_participants: 1,
            approval_numerator: 1,
            approval_denominator: 2,
        }
    }

    pub fn super_majority_two_thirds() -> Self {
        Self {
            min_participants: 1,
            approval_numerator: 2,
            approval_denominator: 3,
        }
    }

    /// Check whether a vote passes given the total number of participants and
    /// the number of approvals.
    pub fn is_satisfied(&self, total_participants: u64, approvals: u64) -> bool {
        if total_participants < self.min_participants {
            return false;
        }
        // Multiply numerator by total and compare with denominator * approvals
        approvals * self.approval_denominator >= total_participants * self.approval_numerator
    }
}
