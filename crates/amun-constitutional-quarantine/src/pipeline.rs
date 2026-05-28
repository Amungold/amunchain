use super::levels::{QuarantineLevel, QuarantineZone, VerificationRecord};
use amun_civilizational_relations::relation::CivilizationalRelation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantinePipeline {
    pub zone: QuarantineZone,
    pub current_level: QuarantineLevel,
}

impl QuarantinePipeline {
    pub fn new(relation: CivilizationalRelation) -> Self {
        let level = match relation {
            CivilizationalRelation::Identical => QuarantineLevel::None,
            CivilizationalRelation::DivergentBranch { .. } => QuarantineLevel::Observation,
            CivilizationalRelation::ReconciliableFork { .. } => {
                QuarantineLevel::IsolatedVerification
            }
            CivilizationalRelation::HostileFork { .. } => QuarantineLevel::FullQuarantine,
            CivilizationalRelation::ForeignCivilization => QuarantineLevel::PermanentSeparation,
            CivilizationalRelation::UnknownOrigin => QuarantineLevel::FullQuarantine,
        };

        Self {
            zone: QuarantineZone {
                level: level.clone(),
                source_relation: relation,
                admitted_snapshots: Vec::new(),
                verification_results: Vec::new(),
            },
            current_level: level,
        }
    }

    pub fn verify_physics(&mut self, snapshot_root: [u8; 32], passed: bool) {
        self.zone.verification_results.push(VerificationRecord {
            snapshot_root,
            physics_verified: passed,
            replay_verified: false,
            lineage_verified: false,
            released: false,
        });
        if passed && self.current_level > QuarantineLevel::Observation {
            self.current_level = QuarantineLevel::Observation;
        }
    }

    pub fn verify_replay(&mut self, snapshot_root: [u8; 32], passed: bool) {
        if let Some(record) = self
            .zone
            .verification_results
            .iter_mut()
            .find(|r| r.snapshot_root == snapshot_root)
        {
            record.replay_verified = passed;
            if passed && self.current_level > QuarantineLevel::IsolatedVerification {
                self.current_level = QuarantineLevel::IsolatedVerification;
            }
        }
    }

    pub fn verify_lineage(&mut self, snapshot_root: [u8; 32], passed: bool) {
        if let Some(record) = self
            .zone
            .verification_results
            .iter_mut()
            .find(|r| r.snapshot_root == snapshot_root)
        {
            record.lineage_verified = passed;
            if passed {
                record.released = true;
                self.current_level = QuarantineLevel::None;
            }
        }
    }

    pub fn is_fully_rehabilitated(&self) -> bool {
        self.current_level == QuarantineLevel::None
    }
}
