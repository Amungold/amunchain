// N147.1: Registry now re-exports ValidatorRecord from amun-validator-api (SSOT).
// NOTE: The API ValidatorRecord has additional fields (status, registered_epoch,
// last_seen, stake_epoch, protocol_version, identity_version) that the registry
// Record did not have. Existing code that constructs ValidatorRecord will get
// compile errors for missing fields. Those will be fixed in N147.2.

pub use amun_validator_api::types::record::ValidatorRecord;
pub use amun_validator_api::types::record::ValidatorStatus;
