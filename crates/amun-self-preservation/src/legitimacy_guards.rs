/// A legitimacy guard prevents constitutional evolution from
/// destroying the conditions that make legitimacy possible.
#[derive(Debug, Clone)]
pub struct LegitimacyGuard {
    pub guard_id: [u8; 32],
    pub protects: GuardedInvariant,
    pub is_active: bool,
    pub violation_count: u64,
    pub max_violations: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardedInvariant {
    ReplayDeterminism,
    CausalIrreflexivity,
    ConstitutionalIdentity,
    MetaAmendmentBounds,
    PhysicsConstants,
    ProofSemantics,
    EmptyRootEternal,
    DomainSeparatorUniqueness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardViolation {
    /// The guarded invariant was violated
    InvariantBroken {
        invariant: GuardedInvariant,
        at_state: [u8; 32],
    },
    /// The guard itself was disabled (meta-violation)
    GuardDisabled { guard_id: [u8; 32] },
    /// Maximum violations exceeded - guard is exhausted
    GuardExhausted { guard_id: [u8; 32], violations: u64 },
}

impl LegitimacyGuard {
    pub fn new(invariant: GuardedInvariant, max_violations: u64) -> Self {
        Self {
            guard_id: [0u8; 32],
            protects: invariant,
            is_active: true,
            violation_count: 0,
            max_violations,
        }
    }

    /// Report a violation. If max violations exceeded, guard is exhausted.
    pub fn report_violation(&mut self) -> Option<GuardViolation> {
        if !self.is_active {
            return Some(GuardViolation::GuardDisabled {
                guard_id: self.guard_id,
            });
        }
        self.violation_count += 1;
        if self.violation_count >= self.max_violations {
            self.is_active = false;
            return Some(GuardViolation::GuardExhausted {
                guard_id: self.guard_id,
                violations: self.violation_count,
            });
        }
        Some(GuardViolation::InvariantBroken {
            invariant: self.protects.clone(),
            at_state: [0u8; 32],
        })
    }

    /// A guard cannot guard itself - this would create a meta-paradox.
    /// Guards are protected by the meta-constitutional layer, not by themselves.
    pub fn can_guard(invariant: &GuardedInvariant) -> bool {
        !matches!(invariant, GuardedInvariant::MetaAmendmentBounds)
    }
}
