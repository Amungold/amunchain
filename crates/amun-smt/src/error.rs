/// All errors returned by SMT operations.
/// Consensus paths MUST NOT panic — every invariant violation returns Err.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SmtError {
    #[error("Depth overflow: depth {depth} exceeds 255")]
    DepthOverflow { depth: usize },

    #[error("Branch has an empty child at depth {depth}")]
    EmptyChild { depth: usize },

    #[error("Branch skip_len {skip_len} exceeds maximum {max} at depth {depth}")]
    SkipLenTooLarge { skip_len: u8, max: u8, depth: usize },

    #[error("Branch skip_len {skip_len} is not minimal (should be {min})")]
    SkipLenNotMinimal { skip_len: u8, min: u8 },

    #[error("Prefix mismatch at bit {bit_idx}")]
    PrefixMismatch { bit_idx: usize },

    #[error("Partition violation: {key_pos} at depth {depth}")]
    PartitionViolation { key_pos: String, depth: usize },

    #[error("Structural cycle detected at depth {depth}")]
    CycleDetected { depth: usize },

    #[error("Proof verification failed: {reason}")]
    ProofVerificationFailed { reason: String },

    #[error("Proof too large: {steps} steps (max {max})")]
    ProofTooLarge { steps: usize, max: usize },

    #[error("Leaf witness missing for divergence proof")]
    MissingLeafWitness,

    #[error("Node not found in context")]
    NodeNotFound,

    #[error("Arithmetic overflow in skip_len calculation")]
    SkipLenOverflow,
}
