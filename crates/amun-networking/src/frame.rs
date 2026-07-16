use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameKind {
    Ping,
    Pong,
    Vote,
    /// Unified consensus message (proposal, vote, QC, finality).
    ConsensusMessage,
    TipRequest,
    TipResponse,
    BlockRangeRequest,
    BlockRangeResponse,
    Handshake,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkFrame {
    pub kind: FrameKind,
    pub payload: crate::payload::Payload,
}

impl NetworkFrame {
    pub fn new(kind: FrameKind, payload: crate::payload::Payload) -> Self {
        Self { kind, payload }
    }
}
