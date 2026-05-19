#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageType {
    PeerDiscovery = 0x01,
    PeerRequest = 0x02,
    PeerResponse = 0x03,
    BlockProposal = 0x10,
    VoteBroadcast = 0x11,
    QuorumCertAnnounce = 0x12,
    TransactionGossip = 0x13,
    Heartbeat = 0x20,
}

pub struct Frame {
    pub msg_type: MessageType,
    pub payload: heapless::Vec<u8, { crate::constants::MAX_MESSAGE_SIZE }>,
}

impl Frame {
    pub fn new(msg_type: MessageType) -> Self {
        Self { msg_type, payload: heapless::Vec::new() }
    }
    pub fn encode(&self) -> heapless::Vec<u8, { crate::constants::MAX_MESSAGE_SIZE }> {
        let mut buf = heapless::Vec::new();
        buf.push(self.msg_type as u8).ok();
        buf.extend_from_slice(&self.payload).ok();
        buf
    }
}
