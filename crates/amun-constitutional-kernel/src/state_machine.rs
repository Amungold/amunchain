// Copyright (c) 2026 Amungold Global
// SPDX-License-Identifier: AGPL-3.0-or-later
// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use amun_constitutional_governance::capability::Capability;
use crate::context::ExecutionContext;
use crate::receipt::ExecutionReceipt;

/// A minimal deterministic constitutional state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstitutionalState {
    pub version: u64,
    pub fields: BTreeMap<String, String>,
}

impl ConstitutionalState {
    pub fn new() -> Self {
        Self {
            version: 0,
            fields: BTreeMap::new(),
        }
    }

    pub fn apply(&self, updates: BTreeMap<String, String>) -> Self {
        let mut new_fields = self.fields.clone();
        for (k, v) in updates {
            new_fields.insert(k, v);
        }
        Self {
            version: self.version + 1,
            fields: new_fields,
        }
    }
}

impl Default for ConstitutionalState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConstitutionalStateMachine {
    pub state: ConstitutionalState,
}

impl ConstitutionalStateMachine {
    pub fn new() -> Self {
        Self {
            state: ConstitutionalState::new(),
        }
    }

    pub fn transition(
        &mut self,
        context: &ExecutionContext,
        capabilities: &[Capability],
        action: &str,
        scope: &str,
        updates: BTreeMap<String, String>,
    ) -> Result<(u64, ExecutionReceipt), String> {
        crate::enforcer::CapabilityEnforcer::require(context, capabilities, action, scope)?;

        let old_version = self.state.version;
        self.state = self.state.apply(updates);
        let new_version = self.state.version;

        let receipt = ExecutionReceipt::new(
            action.into(),
            scope.into(),
            old_version,
            new_version,
            context.epoch.clone(),
        );

        Ok((new_version, receipt))
    }
}

impl Default for ConstitutionalStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
