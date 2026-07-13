/// A concrete cryptographic commitment from a quorum.
/// This replaces a trait object so that the state layer
/// remains serializable and replayable without coupling.
pub struct SealCommitment {
    pub commitment: [u8; 32],
}
