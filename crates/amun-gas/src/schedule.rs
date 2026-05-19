pub struct GasSchedule;

impl GasSchedule {
    pub const TX_BASE: u64 = 21000;
    pub const TRANSFER: u64 = 21000;
    pub const STAKE: u64 = 50000;
    pub const UNSTAKE: u64 = 50000;
    pub const SIGNATURE_VERIFY: u64 = 5000;
    pub const HASH_OPERATION: u64 = 100;
}
