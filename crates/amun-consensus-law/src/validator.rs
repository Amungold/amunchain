#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorStatus {
    Active,
    Jailed { until_epoch: u64 },
    Slashed { amount: u64 },
    Tombstoned,
}

pub struct ValidatorObligations;

impl ValidatorObligations {
    pub fn must_prevote_on_proposal(
        has_proposal: bool,
        is_valid: bool,
    ) -> bool {
        has_proposal && is_valid
    }

    pub fn must_precommit_after_quorum_prevotes(
        prevote_quorum_reached: bool,
        has_locked: bool,
    ) -> bool {
        prevote_quorum_reached || has_locked
    }

    pub fn must_not_equivocate(
        signed_a: Option<[u8; 32]>,
        signed_b: Option<[u8; 32]>,
        same_round: bool,
    ) -> bool {
        if !same_round {
            return true;
        }
        signed_a == signed_b || signed_a.is_none() || signed_b.is_none()
    }

    pub fn slashable_offense(equivocation_proven: bool, downtime_exceeded: bool) -> bool {
        equivocation_proven || downtime_exceeded
    }
}
