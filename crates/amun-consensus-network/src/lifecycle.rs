#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Bootstrapping,
    CatchingUp,
    JoiningConsensus,
    Active,
}
