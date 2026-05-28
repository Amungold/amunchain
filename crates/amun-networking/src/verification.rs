use amun_snapshot_engine::ConstitutionalIdentity;
use super::quarantine::QuarantineEngine;
use super::risk::{ConstitutionalRisk, ConstitutionalRiskProfile};

/// Remote verification result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationResult {
    /// Remote peer is constitutionally compatible
    Compatible {
        remote_identity: ConstitutionalIdentity,
        risk_profile: ConstitutionalRiskProfile,
    },
    /// Remote peer requires quarantine
    QuarantineRequired {
        remote_identity: ConstitutionalIdentity,
        reason: String,
    },
    /// Remote peer is a foreign civilization
    ForeignCivilization {
        remote_identity_hash: [u8; 32],
    },
    /// Verification failed due to tampering
    TamperedIdentity,
}

/// Verifies remote peers against local constitutional identity.
pub struct RemoteVerifier;

impl RemoteVerifier {
    pub fn verify_remote(
        local: &ConstitutionalIdentity,
        remote: &ConstitutionalIdentity,
    ) -> VerificationResult {
        if !remote.verify() {
            return VerificationResult::TamperedIdentity;
        }

        let mut risk_profile = ConstitutionalRiskProfile::new();

        if local.matches(remote) {
            return VerificationResult::Compatible {
                remote_identity: remote.clone(),
                risk_profile,
            };
        }

        risk_profile.add_risk(ConstitutionalRisk::ForeignCivilization {
            remote_identity_hash: remote.identity_hash,
        });

        let quarantine = QuarantineEngine::classify_snapshot(local, remote);
        match quarantine {
            super::quarantine::QuarantineLevel::None => {
                VerificationResult::Compatible {
                    remote_identity: remote.clone(),
                    risk_profile,
                }
            }
            super::quarantine::QuarantineLevel::FullQuarantine => {
                VerificationResult::ForeignCivilization {
                    remote_identity_hash: remote.identity_hash,
                }
            }
            _ => {
                VerificationResult::QuarantineRequired {
                    remote_identity: remote.clone(),
                    reason: format!("Quarantine level: {:?}", quarantine),
                }
            }
        }
    }
}
