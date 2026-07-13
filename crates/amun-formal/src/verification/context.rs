use super::stage::VerificationStage;
use super::state_reader::StateReader;

/// سياق موحد يُمرر لكل invariant
pub struct VerificationContext<'a> {
    pub state: &'a dyn StateReader,
    pub block_height: u64,
    pub epoch: u64,
    pub state_root: [u8; 32],
    pub chain_id: u32,
    pub stage: VerificationStage,
}
