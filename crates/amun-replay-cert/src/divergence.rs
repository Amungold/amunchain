use crate::platform::PlatformFingerprint;

#[derive(Debug, Clone)]
pub struct DivergenceReport {
    pub platform: PlatformFingerprint,
    pub expected_root: [u8; 32],
    pub actual_root: [u8; 32],
    pub divergence_point: u64,
    pub detail: String,
}
