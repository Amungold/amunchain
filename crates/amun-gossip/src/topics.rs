#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Topic {
    Blocks,
    Votes,
    QuorumCert,
    Transactions,
}

impl Topic {
    pub fn as_byte(&self) -> u8 {
        match self {
            Topic::Blocks => 0x10,
            Topic::Votes => 0x11,
            Topic::QuorumCert => 0x12,
            Topic::Transactions => 0x13,
        }
    }
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x10 => Some(Topic::Blocks),
            0x11 => Some(Topic::Votes),
            0x12 => Some(Topic::QuorumCert),
            0x13 => Some(Topic::Transactions),
            _ => None,
        }
    }
}
