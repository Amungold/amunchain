#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerificationCategory {
    Economics = 0,
    Consensus = 1,
    Runtime = 2,
    Governance = 3,
    Security = 4,
    NFT = 5,
    Replay = 6,
    Recovery = 7,
    Constitutional = 8,
}
