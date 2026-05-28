/// Rules governing when an amendment becomes active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivationRules {
    /// Minimum epochs between ratification and activation
    pub activation_delay_epochs: u64,
    /// Whether activation requires a specific epoch boundary
    pub require_epoch_boundary: bool,
    /// Whether old proofs remain valid after activation
    pub preserve_historical_proofs: bool,
}

/// The epoch at which an amendment becomes effective.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationEpoch(pub u64);
