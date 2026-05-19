use amun_chain_position::ChainPosition;

/// A message in the consensus network.
#[derive(Debug, Clone)]
pub enum NetworkMessage {
    Proposal {
        from: u64,
        to: u64,
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
    },
    Prevote {
        from: u64,
        to: u64,
        position: ChainPosition,
        round: u64,
        block_hash: Option<[u8; 32]>,
    },
    Precommit {
        from: u64,
        to: u64,
        position: ChainPosition,
        round: u64,
        block_hash: Option<[u8; 32]>,
    },
    QuorumCertificate {
        from: u64,
        to: u64,
        round: u64,
        qc_hash: [u8; 32],
    },
}

impl NetworkMessage {
    pub fn from_id(&self) -> u64 {
        match self {
            NetworkMessage::Proposal { from, .. } => *from,
            NetworkMessage::Prevote { from, .. } => *from,
            NetworkMessage::Precommit { from, .. } => *from,
            NetworkMessage::QuorumCertificate { from, .. } => *from,
        }
    }

    pub fn to_id(&self) -> u64 {
        match self {
            NetworkMessage::Proposal { to, .. } => *to,
            NetworkMessage::Prevote { to, .. } => *to,
            NetworkMessage::Precommit { to, .. } => *to,
            NetworkMessage::QuorumCertificate { to, .. } => *to,
        }
    }

    pub fn round(&self) -> u64 {
        match self {
            NetworkMessage::Proposal { round, .. } => *round,
            NetworkMessage::Prevote { round, .. } => *round,
            NetworkMessage::Precommit { round, .. } => *round,
            NetworkMessage::QuorumCertificate { round, .. } => *round,
        }
    }
}
