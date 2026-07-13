use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RequestKey {
    pub peer: SocketAddr,
    pub request_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TipRequestPayload {
    pub request_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TipResponsePayload {
    pub request_id: u64,
    pub height: u64,
    pub hash: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockRangeRequestPayload {
    pub request_id: u64,
    pub start: u64,
    pub end: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockRangeResponsePayload {
    pub request_id: u64,
    pub records: Vec<Vec<u8>>,
}

pub const MAX_BLOCK_RANGE: u64 = 1000;
