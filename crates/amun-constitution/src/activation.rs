use crate::canonical_form::ConstitutionDomain;
use amun_kernel_types::Hash;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivationStatus {
    Pending,
    GracePeriod { started_at: u64, ends_at: u64 },
    Active { activated_at: u64 },
    Rejected,
}

#[derive(Clone, Debug)]
pub struct ActivationSchedule {
    pub new_constitution_hash: Hash<ConstitutionDomain>,
    pub signalling_epoch: u64,
    pub activation_epoch: u64,
    pub deprecation_epoch: u64,
}

impl ActivationSchedule {
    pub fn status_at(&self, current_epoch: u64) -> ActivationStatus {
        if current_epoch < self.signalling_epoch {
            ActivationStatus::Pending
        } else if current_epoch < self.activation_epoch {
            ActivationStatus::GracePeriod {
                started_at: self.signalling_epoch,
                ends_at: self.activation_epoch,
            }
        } else {
            ActivationStatus::Active {
                activated_at: self.activation_epoch,
            }
        }
    }
}
