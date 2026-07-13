use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipResponse {
    pub height: u64,
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRangeRequest {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockRangeResponse {
    // كل Record مشفر بـ postcard أو encode()
    pub records: Vec<crate::payload::Payload>,
}
