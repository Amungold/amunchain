// N147.1: Registry now re-exports types from amun-validator-api (SSOT).
// The local definitions are removed. All consumers use the same types.

pub use amun_validator_api::types::id::{PeerId, PublicKey, ValidatorId};
