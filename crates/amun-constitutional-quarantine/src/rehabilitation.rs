use super::levels::QuarantineLevel;

/// A rehabilitation path describes how a quarantined civilization
/// can progress from isolation to trusted interaction.
#[derive(Debug, Clone)]
pub struct RehabilitationPath {
    pub starting_level: QuarantineLevel,
    pub required_verifications: Vec<RehabilitationStep>,
    pub current_step: usize,
}

#[derive(Debug, Clone)]
pub enum RehabilitationStep {
    PhysicsVerification,
    ReplayContinuityVerification,
    LineageProofVerification,
    TemporalAlignmentVerification,
    ConstitutionalReconciliation,
    TreatyEstablishment,
}

impl RehabilitationPath {
    pub fn new(starting_level: QuarantineLevel) -> Self {
        let steps = match starting_level {
            QuarantineLevel::None => vec![],
            QuarantineLevel::Observation => vec![
                RehabilitationStep::PhysicsVerification,
                RehabilitationStep::ReplayContinuityVerification,
            ],
            QuarantineLevel::IsolatedVerification => vec![
                RehabilitationStep::PhysicsVerification,
                RehabilitationStep::ReplayContinuityVerification,
                RehabilitationStep::LineageProofVerification,
            ],
            QuarantineLevel::ConditionalAccess => vec![
                RehabilitationStep::PhysicsVerification,
                RehabilitationStep::ReplayContinuityVerification,
                RehabilitationStep::LineageProofVerification,
                RehabilitationStep::TemporalAlignmentVerification,
            ],
            QuarantineLevel::FullQuarantine => vec![
                RehabilitationStep::PhysicsVerification,
                RehabilitationStep::ReplayContinuityVerification,
                RehabilitationStep::LineageProofVerification,
                RehabilitationStep::TemporalAlignmentVerification,
                RehabilitationStep::ConstitutionalReconciliation,
            ],
            QuarantineLevel::PermanentSeparation => vec![], // Cannot be rehabilitated
        };
        Self {
            starting_level,
            required_verifications: steps,
            current_step: 0,
        }
    }

    pub fn advance(&mut self) {
        if self.current_step < self.required_verifications.len() {
            self.current_step += 1;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_step >= self.required_verifications.len()
    }
}
