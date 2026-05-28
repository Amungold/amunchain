//! Constitutional State Object - The Heart of AmunChain

use crate::canonical::{CanonicalEncoder, CanonicalDecoder, CanonicalSerialize, CanonicalDeserialize};
use crate::constitutional::{ConstitutionalState, ConstitutionalTransition};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmunState {
    pub state_root: [u8; 32],
    pub height: u64,
    pub epoch: u64,
    pub validator_set_hash: [u8; 32],
}

impl AmunState {
    pub fn genesis() -> Self {
        Self {
            state_root: [0u8; 32],
            height: 0,
            epoch: 0,
            validator_set_hash: [0u8; 32],
        }
    }
}

impl CanonicalSerialize for AmunState {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_fixed_hash(&self.state_root);
        encoder.write_u64(self.height);
        encoder.write_u64(self.epoch);
        encoder.write_fixed_hash(&self.validator_set_hash);
    }
}

impl CanonicalDeserialize for AmunState {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        let state_root = decoder.read_fixed_hash()?;
        let height = decoder.read_u64()?;
        let epoch = decoder.read_u64()?;
        let validator_set_hash = decoder.read_fixed_hash()?;
        Some(AmunState {
            state_root,
            height,
            epoch,
            validator_set_hash,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferTransition {
    pub from: u64,
    pub to: u64,
    pub amount: u64,
    pub new_state_root: [u8; 32],
}

impl CanonicalSerialize for TransferTransition {
    fn encode(&self, encoder: &mut CanonicalEncoder) {
        encoder.write_u64(self.from);
        encoder.write_u64(self.to);
        encoder.write_u64(self.amount);
        encoder.write_fixed_hash(&self.new_state_root);
    }
}

impl CanonicalDeserialize for TransferTransition {
    fn decode(decoder: &mut CanonicalDecoder) -> Option<Self> {
        let from = decoder.read_u64()?;
        let to = decoder.read_u64()?;
        let amount = decoder.read_u64()?;
        let new_state_root = decoder.read_fixed_hash()?;
        Some(TransferTransition {
            from,
            to,
            amount,
            new_state_root,
        })
    }
}

impl ConstitutionalTransition for TransferTransition {
    type State = AmunState;

    fn verify(&self, pre_state: &Self::State) -> bool {
        self.new_state_root != pre_state.state_root
    }

    fn compute_post_hash(&self, _pre_hash: [u8; 32]) -> [u8; 32] {
        self.new_state_root
    }
}

impl ConstitutionalState for AmunState {
    type Transition = TransferTransition;

    fn apply_transition(self, transition: &Self::Transition) -> Result<Self, &'static str> {
        if !transition.verify(&self) {
            return Err("Transition verification failed");
        }
        Ok(AmunState {
            state_root: transition.new_state_root,
            height: self.height + 1,
            epoch: self.epoch,
            validator_set_hash: self.validator_set_hash,
        })
    }
}

// Implement VersionedRoot for AmunState
use crate::versioning::VersionedRoot;

impl VersionedRoot for AmunState {
    fn encode_content(&self, encoder: &mut crate::canonical::CanonicalEncoder) {
        encoder.write_fixed_hash(&self.state_root);
        encoder.write_u64(self.height);
        encoder.write_u64(self.epoch);
        encoder.write_fixed_hash(&self.validator_set_hash);
    }

    fn decode_content(decoder: &mut crate::canonical::CanonicalDecoder) -> Option<Self> {
        let state_root = decoder.read_fixed_hash()?;
        let height = decoder.read_u64()?;
        let epoch = decoder.read_u64()?;
        let validator_set_hash = decoder.read_fixed_hash()?;
        Some(AmunState {
            state_root,
            height,
            epoch,
            validator_set_hash,
        })
    }
}

// ConstitutionalHashable implementation
impl crate::constitutional::ConstitutionalHashable for AmunState {
    const DOMAIN_TAG: &'static [u8] = b"AMUN_STATE_V1";
}

impl crate::constitutional::ConstitutionalHashable for TransferTransition {
    const DOMAIN_TAG: &'static [u8] = b"AMUN_TRANSFER_V1";
}
