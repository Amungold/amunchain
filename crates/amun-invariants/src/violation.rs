#[derive(Debug, Clone)]
pub struct ViolationRecord {
    pub epoch: u64,
    pub block_height: u64,
    pub invariant_id: u32,
    pub description: &'static str,
    pub evidence_hash: [u8; 32],
}
