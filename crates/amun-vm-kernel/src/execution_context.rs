use amun_resource_core::ResourceId;

/// The context in which a contract execution occurs.
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub contract_id: ResourceId,
    pub caller: [u8; 32],
    pub block_height: u64,
    pub block_hash: [u8; 32],
    pub transaction_hash: [u8; 32],
    pub pre_state_root: [u8; 32],
    pub authority: [u8; 32],
}

