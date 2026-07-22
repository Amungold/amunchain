use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RpcConstitutionalStatusResponse {
    pub constitution_hash: String,
    pub active_verdicts: u64,
    pub total_evidence_records: u64,
    pub last_amendment_height: u64,
}

#[derive(Debug, Clone)]
pub struct ConstitutionalStatus {
    pub constitution_hash: String,
    pub active_verdicts: u64,
    pub total_evidence_records: u64,
    pub last_amendment_height: u64,
}

impl From<RpcConstitutionalStatusResponse> for ConstitutionalStatus {
    fn from(rpc: RpcConstitutionalStatusResponse) -> Self {
        Self {
            constitution_hash: rpc.constitution_hash,
            active_verdicts: rpc.active_verdicts,
            total_evidence_records: rpc.total_evidence_records,
            last_amendment_height: rpc.last_amendment_height,
        }
    }
}
