// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use amun_constitutional_governance::amendment::AmendmentLifecycle;
use amun_constitutional_governance::voting::Tally;
use std::collections::BTreeMap;
use crate::state_machine::ConstitutionalStateMachine;
use crate::context::ExecutionContext;
use crate::enforcer::CapabilityEnforcer;
use amun_constitutional_governance::capability::Capability;

/// Activates an approved amendment by applying its state transition.
pub struct AmendmentActivator;

impl AmendmentActivator {
    /// Activate an amendment if its lifecycle is in the Approved state
    /// and the required capability is held.
    pub fn activate(
        machine: &mut ConstitutionalStateMachine,
        context: &ExecutionContext,
        capabilities: &[Capability],
        amendment: &mut AmendmentLifecycle,
        tally: &Tally,
    ) -> Result<u64, String> {
        // Guard: capability required for activation.
        CapabilityEnforcer::require(
            context,
            capabilities,
            "activate_amendment",
            "constitutional",
        )?;

        // Guard: amendment must be approved.
        match amendment.state {
            amun_constitutional_governance::amendment::AmendmentState::Approved => {}
            _ => return Err("Amendment is not in Approved state".into()),
        }

        // Guard: tally must show passing.
        if !tally.passed {
            return Err("Amendment tally did not pass".into());
        }

        // Apply the amendment as a state update.
        let mut updates = BTreeMap::new();
        updates.insert(
            format!("amendment.{}", amendment.proposal.proposal_id),
            "active".into(),
        );

        let (version, _receipt) = machine.transition(
            context,
            capabilities,
            "activate_amendment",
            "constitutional",
            updates,
        )?;

        amendment.activate();
        Ok(version)
    }
}
