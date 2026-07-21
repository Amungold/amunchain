// ============================================================================
// ADR-021: Rule Engine for Safety & Liveness audit
// ============================================================================

use crate::events::{ConsensusEvent, ConsensusEventKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleCategory {
    Safety,
    Liveness,
}

#[derive(Debug, Clone)]
pub struct AuditViolation {
    pub rule_name: String,
    pub category: RuleCategory,
    pub message: String,
    pub height: u64,
}

pub trait AuditRule: Send + Sync {
    fn name(&self) -> &str;
    fn category(&self) -> RuleCategory;
    fn check(&self, events: &[ConsensusEvent]) -> Vec<AuditViolation>;
}

// ==================== Safety Rules ====================

pub struct NoDoubleVote;
impl AuditRule for NoDoubleVote {
    fn name(&self) -> &str {
        "NoDoubleVote"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Safety
    }

    fn check(&self, events: &[ConsensusEvent]) -> Vec<AuditViolation> {
        let mut violations = Vec::new();
        let mut votes: std::collections::HashMap<(u64, u64), usize> =
            std::collections::HashMap::new();

        for event in events {
            if let ConsensusEventKind::VoteSent { height, from, .. } = &event.event {
                let count = votes.entry((*height, *from)).or_insert(0);
                *count += 1;
                if *count > 1 {
                    violations.push(AuditViolation {
                        rule_name: self.name().into(),
                        category: self.category(),
                        message: format!("Validator {} double-voted at height {}", from, height),
                        height: *height,
                    });
                }
            }
        }

        violations
    }
}

pub struct NoDoubleProposal;
impl AuditRule for NoDoubleProposal {
    fn name(&self) -> &str {
        "NoDoubleProposal"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Safety
    }

    fn check(&self, events: &[ConsensusEvent]) -> Vec<AuditViolation> {
        let mut violations = Vec::new();
        let mut proposals: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();

        for event in events {
            if let ConsensusEventKind::ProposalCreated { height, .. } = &event.event {
                let count = proposals.entry(*height).or_insert(0);
                *count += 1;
                if *count > 1 {
                    violations.push(AuditViolation {
                        rule_name: self.name().into(),
                        category: self.category(),
                        message: format!("Multiple proposals at height {}", height),
                        height: *height,
                    });
                }
            }
        }

        violations
    }
}

// ==================== Liveness Rules ====================

pub struct RoundMustStart;
impl AuditRule for RoundMustStart {
    fn name(&self) -> &str {
        "RoundMustStart"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Liveness
    }

    fn check(&self, events: &[ConsensusEvent]) -> Vec<AuditViolation> {
        if events.is_empty() {
            return vec![AuditViolation {
                rule_name: self.name().into(),
                category: self.category(),
                message: "No events recorded - consensus never started".into(),
                height: 0,
            }];
        }
        vec![]
    }
}

pub struct ProposalMustExist;
impl AuditRule for ProposalMustExist {
    fn name(&self) -> &str {
        "ProposalMustExist"
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::Liveness
    }

    fn check(&self, events: &[ConsensusEvent]) -> Vec<AuditViolation> {
        let mut violations = Vec::new();
        let mut started_heights = std::collections::HashSet::new();
        let mut proposed_heights = std::collections::HashSet::new();

        for event in events {
            if let ConsensusEventKind::RoundStarted { height, .. } = &event.event {
                started_heights.insert(*height);
            }
            if let ConsensusEventKind::ProposalCreated { height, .. } = &event.event {
                proposed_heights.insert(*height);
            }
        }

        for height in &started_heights {
            if !proposed_heights.contains(height) {
                violations.push(AuditViolation {
                    rule_name: self.name().into(),
                    category: self.category(),
                    message: format!("Round started at height {} but no proposal created", height),
                    height: *height,
                });
            }
        }

        violations
    }
}

// ==================== Rule Engine ====================

pub struct RuleEngine {
    rules: Vec<Box<dyn AuditRule>>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn with_rule<R: AuditRule + 'static>(mut self, rule: R) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn check_all(&self, events: &[ConsensusEvent]) -> Vec<AuditViolation> {
        let mut all_violations = Vec::new();
        for rule in &self.rules {
            all_violations.extend(rule.check(events));
        }
        all_violations
    }

    pub fn check_safety(&self, events: &[ConsensusEvent]) -> Vec<AuditViolation> {
        self.rules
            .iter()
            .filter(|r| r.category() == RuleCategory::Safety)
            .flat_map(|r| r.check(events))
            .collect()
    }

    pub fn check_liveness(&self, events: &[ConsensusEvent]) -> Vec<AuditViolation> {
        self.rules
            .iter()
            .filter(|r| r.category() == RuleCategory::Liveness)
            .flat_map(|r| r.check(events))
            .collect()
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl AuditViolation {
    pub fn category_name(&self) -> &str {
        match self.category {
            RuleCategory::Safety => "SAFETY",
            RuleCategory::Liveness => "LIVENESS",
        }
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}
