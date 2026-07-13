#![allow(clippy::result_large_err)]
pub mod certifier;
pub mod divergence;
pub mod platform;
pub mod transcript;

pub use certifier::ReplayCertifier;
pub use divergence::DivergenceReport;
pub use platform::PlatformFingerprint;
pub use transcript::ReplayTranscript;

impl Default for ReplayTranscript {
    fn default() -> Self {
        Self::new()
    }
}
