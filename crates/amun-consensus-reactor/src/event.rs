use amun_chain_position::ChainPosition;

/// Events that drive the consensus reactor loop.
#[derive(Debug, Clone)]
pub enum ReactorEvent {
    /// A new proposal arrived
    ProposalReceived {
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
    },
    /// A quorum of prevotes was achieved
    PrevoteQuorum {
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
    },
    /// A quorum of precommits was achieved
    PrecommitQuorum {
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
    },
    /// A round timed out
    RoundTimeout {
        round: u64,
    },
    /// Enough view-change messages to advance
    ViewChangeQuorum {
        new_round: u64,
    },
    /// An epoch transition should occur
    EpochTransition {
        new_epoch: u64,
    },
    /// Heartbeat tick
    Tick,
}
