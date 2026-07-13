// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use crate::quorum::QuorumPolicy;
use crate::voting::Proposal;

/// The amendment lifecycle is a simple state machine for constitutional changes.
pub enum AmendmentState {
    Draft,
    Proposed,
    Approved,
    Rejected,
    Active,
}

pub struct AmendmentLifecycle {
    pub proposal: Proposal,
    pub state: AmendmentState,
}

impl AmendmentLifecycle {
    pub fn new(title: String, description: String, epoch_start: String, epoch_end: String) -> Self {
        let quorum = QuorumPolicy::super_majority_two_thirds();
        let proposal = Proposal::new(title, description, epoch_start, epoch_end, quorum);
        Self {
            proposal,
            state: AmendmentState::Draft,
        }
    }

    pub fn propose(&mut self) {
        self.state = AmendmentState::Proposed;
    }

    pub fn approve(&mut self) {
        self.state = AmendmentState::Approved;
    }

    pub fn reject(&mut self) {
        self.state = AmendmentState::Rejected;
    }

    pub fn activate(&mut self) {
        self.state = AmendmentState::Active;
    }
}
