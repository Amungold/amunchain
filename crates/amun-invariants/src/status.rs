use crate::violation::ViolationRecord;

#[derive(Debug, Clone)]
pub struct InvariantStatus {
    pub invariant_id: u32,
    pub currently_violated: bool,
    pub violation_count: u64,
    pub last_violation_epoch: Option<u64>,
    pub recent_violations: Vec<ViolationRecord>,
}

impl InvariantStatus {
    pub fn new(invariant_id: u32) -> Self {
        Self {
            invariant_id,
            currently_violated: false,
            violation_count: 0,
            last_violation_epoch: None,
            recent_violations: Vec::new(),
        }
    }

    pub fn record_violation(&mut self, record: ViolationRecord) {
        self.currently_violated = true;
        self.violation_count = self.violation_count.saturating_add(1);
        self.last_violation_epoch = Some(record.epoch);
        if self.recent_violations.len() >= 8 {
            self.recent_violations.remove(0);
        }
        self.recent_violations.push(record);
    }

    pub fn is_currently_violated(&self) -> bool {
        self.currently_violated
    }
}
