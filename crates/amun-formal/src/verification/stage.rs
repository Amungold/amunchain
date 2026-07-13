#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStage {
    PreExecution,
    PostExecution,
    PreCommit,
    PostCommit,
    Replay,
    Recovery,
    Snapshot,
}
