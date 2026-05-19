use crate::kernel::{InvariantDef, InvariantSeverity, IrreducibleInvariants};
use crate::status::InvariantStatus;
use crate::violation::ViolationRecord;

#[derive(Debug)]
pub struct InvariantRegistry {
    invariants: Vec<InvariantDef>,
    statuses: Vec<InvariantStatus>,
}

impl InvariantRegistry {
    pub fn with_kernel() -> Self {
        let mut invariants = Vec::new();
        let mut statuses = Vec::new();

        for inv in IrreducibleInvariants::all().iter() {
            invariants.push(inv.clone());
            statuses.push(InvariantStatus::new(inv.id));
        }

        Self {
            invariants,
            statuses,
        }
    }

    pub fn register(&mut self, inv: InvariantDef) -> Result<(), &'static str> {
        if self.invariants.len() >= 16 {
            return Err("invariant registry full");
        }
        if self.invariants.iter().any(|e| e.id == inv.id) {
            return Err("duplicate invariant id");
        }
        self.invariants.push(inv.clone());
        self.statuses.push(InvariantStatus::new(inv.id));
        Ok(())
    }

    pub fn record_violation(
        &mut self,
        inv_id: u32,
        record: ViolationRecord,
    ) -> Result<(), &'static str> {
        if let Some(status) = self.statuses.iter_mut().find(|s| s.invariant_id == inv_id) {
            status.record_violation(record);
            Ok(())
        } else {
            Err("invariant not found")
        }
    }

    pub fn invariant_count(&self) -> usize {
        self.invariants.len()
    }

    pub fn overall_health(&self) -> InvariantHealth {
        let total = self.invariants.len();
        let violated = self
            .statuses
            .iter()
            .filter(|s| s.is_currently_violated())
            .count();
        let fatal_violated = self
            .statuses
            .iter()
            .filter(|s| s.is_currently_violated())
            .filter(|s| {
                self.invariants
                    .iter()
                    .any(|i| i.id == s.invariant_id && i.severity == InvariantSeverity::Fatal)
            })
            .count();

        if fatal_violated > 0 {
            InvariantHealth::Critical
        } else if violated > 0 {
            InvariantHealth::Degraded
        } else if total == 0 {
            InvariantHealth::Unknown
        } else {
            InvariantHealth::AllInvariantsHold
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InvariantHealth {
    AllInvariantsHold,
    Degraded,
    Critical,
    Unknown,
}
