use amun_kernel_types::{ValidatorId, PublicKey};
use amun_consensus_types::ValidatorIndex;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use heapless::Vec;

#[derive(Clone, Debug)]
pub struct ValidatorInfo {
    pub id: ValidatorId,
    pub pubkey: PublicKey,
    pub index: ValidatorIndex,
    pub is_active: bool,
    pub total_votes_cast: u64,
    pub last_voted_round: u64,
}

pub struct ValidatorSet {
    validators: Vec<ValidatorInfo, 256>,
    total_active_stake: u64,
}

impl ValidatorSet {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
            total_active_stake: 0,
        }
    }

    pub fn add_validator(
        &mut self,
        id: ValidatorId,
        pubkey: PublicKey,
    ) -> Result<(), FailureContext> {
        if self.validators.is_full() {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x000C,
                0x0001,
            ));
        }
        let index = ValidatorIndex::new(self.validators.len() as u16);
        self.validators
            .push(ValidatorInfo {
                id,
                pubkey,
                index,
                is_active: true,
                total_votes_cast: 0,
                last_voted_round: 0,
            })
            .map_err(|_| {
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000C, 0x0002)
            })?;
        self.total_active_stake = self.total_active_stake.checked_add(1).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x000C, 0x0003)
        })?;
        Ok(())
    }

    pub fn active_count(&self) -> usize {
        self.validators.iter().filter(|v| v.is_active).count()
    }

    pub fn total_count(&self) -> usize {
        self.validators.len()
    }

    pub fn byzantine_threshold(&self) -> usize {
        let n = self.active_count();
        if n == 0 {
            return 0;
        }
        (n - 1) / 3
    }

    pub fn quorum_threshold(&self) -> usize {
        2 * self.byzantine_threshold() + 1
    }

    pub fn get(&self, index: ValidatorIndex) -> Option<&ValidatorInfo> {
        self.validators.get(index.value() as usize)
    }

    pub fn get_mut(&mut self, index: ValidatorIndex) -> Option<&mut ValidatorInfo> {
        self.validators.get_mut(index.value() as usize)
    }

    pub fn is_quorum(&self, count: usize) -> bool {
        count >= self.quorum_threshold()
    }

    pub fn record_vote(&mut self, index: ValidatorIndex, round: u64) -> AmunResult<()> {
        if let Some(validator) = self.get_mut(index) {
            validator.total_votes_cast = validator.total_votes_cast.saturating_add(1);
            validator.last_voted_round = round;
            Ok(())
        } else {
            Err(FailureContext::new(
                ConstitutionalFault::InvalidInput,
                0x000C,
                0x0004,
            ))
        }
    }
}
