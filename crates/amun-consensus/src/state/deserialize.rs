//! Canonical deserialization for state objects

use crate::canonical::{CanonicalDecoder, CanonicalDeserialize};
use super::{AmunState, TransferTransition};

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
