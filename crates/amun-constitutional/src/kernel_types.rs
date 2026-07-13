//! Kernel types — foundational constitutional primitives.

pub type ConstitutionalHash = [u8; 32];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayDomain {
    Canonical = 0x01,
    Governance = 0x02,
    Bridge = 0x03,
    System = 0x04,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayFailure {
    pub failure_hash: ConstitutionalHash,
    pub domain: ReplayDomain,
    pub transcript_position: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayPolicy {
    pub allow_cross_domain: bool,
    pub max_divergences: u64,
    pub halt_on_first_divergence: bool,
}

impl Default for ReplayPolicy {
    fn default() -> Self {
        Self {
            allow_cross_domain: false,
            max_divergences: 0,
            halt_on_first_divergence: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptEntry {
    pub entry_hash: ConstitutionalHash,
    pub sequence: u64,
    pub domain: ReplayDomain,
}
