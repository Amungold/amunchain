/// An interoperability covenant defines the rules for safe interaction
/// between civilizations that are not identical but can coexist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteroperabilityCovenant {
    pub covenant_id: [u8; 32],
    pub civilizations: Vec<[u8; 32]>,
    pub allows_state_exchange: bool,
    pub allows_snapshot_import: bool,
    pub requires_quarantine: bool,
    pub max_epoch_divergence: u64,
    pub covenant_hash: [u8; 32],
}
