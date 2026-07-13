// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use crate::context::ExecutionContext;
use amun_constitutional_governance::capability::Capability;

/// The CapabilityEnforcer validates that an operation is authorised by
/// a valid capability present in the execution context's authority chain.
pub struct CapabilityEnforcer;

impl CapabilityEnforcer {
    /// Require that `action` is permitted by a capability whose scope
    /// matches `required_scope` and whose epoch contains the context's epoch.
    pub fn require(
        context: &ExecutionContext,
        capabilities: &[Capability],
        action: &str,
        required_scope: &str,
    ) -> Result<(), String> {
        let epoch = &context.epoch;

        let found = capabilities.iter().any(|cap| {
            cap.action == action
                && cap.scope == required_scope
                && cap.epoch_start <= *epoch
                && cap.epoch_end >= *epoch
        });

        if !found {
            return Err(format!(
                "Capability not satisfied: action={}, scope={}, epoch={}",
                action, required_scope, epoch
            ));
        }
        Ok(())
    }
}
