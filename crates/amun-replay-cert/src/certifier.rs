use crate::divergence::DivergenceReport;
use crate::platform::PlatformFingerprint;
use crate::transcript::ReplayTranscript;
use amun_truth_engine::TruthEngine;
use blake3::Hasher;

pub struct ReplayCertifier {
    engine: TruthEngine,
    transcript: ReplayTranscript,
    platforms: Vec<PlatformFingerprint>,
    verified: bool,
}

impl ReplayCertifier {
    pub fn new(engine: TruthEngine) -> Self {
        Self {
            engine,
            transcript: ReplayTranscript::new(),
            platforms: Vec::new(),
            verified: false,
        }
    }

    pub fn record_platform(&mut self, platform: PlatformFingerprint) {
        self.platforms.push(platform);
    }

    pub fn certify(&mut self, target_height: u64) -> Result<CertificationResult, DivergenceReport> {
        self.transcript.reset();

        let reference_root =
            self.engine
                .compute_chain_root(target_height)
                .map_err(|e| DivergenceReport {
                    platform: PlatformFingerprint::current(),
                    expected_root: [0u8; 32],
                    actual_root: [0u8; 32],
                    divergence_point: 0,
                    detail: format!("{:?}", e),
                })?;

        let mut results = Vec::new();

        for platform in &self.platforms {
            let platform_root =
                self.engine
                    .compute_chain_root(target_height)
                    .map_err(|e| DivergenceReport {
                        platform: platform.clone(),
                        expected_root: reference_root,
                        actual_root: [0u8; 32],
                        divergence_point: 0,
                        detail: format!("{:?}", e),
                    })?;

            if platform_root != reference_root {
                return Err(DivergenceReport {
                    platform: platform.clone(),
                    expected_root: reference_root,
                    actual_root: platform_root,
                    divergence_point: 0,
                    detail: format!("Platform {} diverged", platform.name()),
                });
            }

            results.push(PlatformResult {
                platform: platform.clone(),
                root: platform_root,
                matches: true,
            });
        }

        let cert_hash = {
            let mut hasher = Hasher::new();
            hasher.update(b"AMUN_REPLAY_CERT_V1");
            hasher.update(&reference_root);
            for result in &results {
                hasher.update(&result.platform.tag());
                hasher.update(&result.root);
            }
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
            hash
        };

        self.verified = true;
        self.transcript.finalize(reference_root);

        Ok(CertificationResult {
            reference_root,
            platform_results: results,
            certification_hash: cert_hash,
            target_height,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CertificationResult {
    pub reference_root: [u8; 32],
    pub platform_results: Vec<PlatformResult>,
    pub certification_hash: [u8; 32],
    pub target_height: u64,
}

#[derive(Debug, Clone)]
pub struct PlatformResult {
    pub platform: PlatformFingerprint,
    pub root: [u8; 32],
    pub matches: bool,
}
