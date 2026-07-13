// Deterministic kernel health state. No atomics, no globals.
// Passed explicitly through every consensus function.

use crate::taxonomy::{module_ids, operation_ids, AmunResult, ConstitutionalFault, FailureContext};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelHealth {
    Healthy,
    Poisoned {
        fault: ConstitutionalFault,
        at_epoch: u64,
        at_round: u64,
    },
}

impl KernelHealth {
    pub const fn healthy() -> Self {
        Self::Healthy
    }

    pub fn poison(self, fault: ConstitutionalFault, epoch: u64, round: u64) -> Self {
        match self {
            Self::Poisoned { .. } => self,
            Self::Healthy => Self::Poisoned {
                fault,
                at_epoch: epoch,
                at_round: round,
            },
        }
    }

    pub const fn can_participate(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    pub fn quarantine_actions(&self) -> QuarantineActions {
        match self {
            Self::Healthy => QuarantineActions::none(),
            Self::Poisoned { fault, .. } => QuarantineActions {
                halt_consensus: true,
                seal_journal: true,
                invalidate_snapshots: true,
                preserve_evidence: true,
                evidence_fault: Some(*fault),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct QuarantineActions {
    pub halt_consensus: bool,
    pub seal_journal: bool,
    pub invalidate_snapshots: bool,
    pub preserve_evidence: bool,
    pub evidence_fault: Option<ConstitutionalFault>,
}

impl QuarantineActions {
    const fn none() -> Self {
        Self {
            halt_consensus: false,
            seal_journal: false,
            invalidate_snapshots: false,
            preserve_evidence: false,
            evidence_fault: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KernelState {
    pub health: KernelHealth,
    pub epoch: u64,
    pub round: u64,
    pub chain_id: u64,
    pub protocol_version: u8,
}

impl KernelState {
    pub const fn new(chain_id: u64, epoch: u64, round: u64) -> Self {
        Self {
            health: KernelHealth::healthy(),
            epoch,
            round,
            chain_id,
            protocol_version: 1,
        }
    }

    pub fn check_healthy(&self) -> AmunResult<()> {
        match self.health {
            KernelHealth::Healthy => Ok(()),
            KernelHealth::Poisoned { fault, .. } => Err(FailureContext::new(
                fault,
                module_ids::AMUN_FAILURE,
                operation_ids::KERNEL_CHECK_HEALTH,
            )),
        }
    }

    pub fn record_fault(mut self, ctx: &FailureContext) -> Self {
        if ctx.fault.should_halt() {
            self.health = self.health.poison(ctx.fault, self.epoch, self.round);
        }
        self
    }
}
