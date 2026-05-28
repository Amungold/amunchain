//! # Slashing Adjudication Pipeline
//! 
//! Replay-safe evidence validation and penalty application.

use std::collections::BTreeMap;
use super::evidence::{SlashingEvidence, ConstitutionalOffense, OffenseSeverity};
use super::super::validator::set::ValidatorSet;

/// Adjudication result
#[derive(Debug, Clone)]
pub enum AdjudicationResult {
    ValidSlash(u64),  // penalty percentage
    InvalidEvidence,
    AlreadySlashed,
    InsufficientProof,
    StaleEvidence,
}

/// Slashing adjudicator with replay protection
#[derive(Debug, Clone)]
pub struct SlashingAdjudicator {
    processed_evidence: BTreeMap<[u8; 32], bool>,  // evidence_hash -> processed
    slash_history: BTreeMap<u64, Vec<ConstitutionalOffense>>,
}

impl SlashingAdjudicator {
    pub fn new() -> Self {
        Self {
            processed_evidence: BTreeMap::new(),
            slash_history: BTreeMap::new(),
        }
    }
    
    /// Adjudicate slashing evidence with replay protection
    pub fn adjudicate(&mut self, evidence: &SlashingEvidence, validator_set: &mut ValidatorSet) -> AdjudicationResult {
        // Compute evidence hash for replay protection
        let evidence_hash = evidence.hash();
        
        // Replay protection
        if self.processed_evidence.contains_key(&evidence_hash) {
            return AdjudicationResult::StaleEvidence;
        }
        
        // Check if already slashed
        if let Some(history) = self.slash_history.get(&evidence.validator_id) {
            let already_slashed = history.iter().any(|o| o.severity() >= OffenseSeverity::Major);
            if already_slashed {
                return AdjudicationResult::AlreadySlashed;
            }
        }
        
        // Verify evidence
        if !evidence.verify() {
            return AdjudicationResult::InvalidEvidence;
        }
        
        // Check statute of limitations (within last 100 epochs)
        if evidence.epoch + 100 < validator_set.current_epoch().as_u64() {
            return AdjudicationResult::StaleEvidence;
        }
        
        // Calculate penalty
        let penalty = evidence.offense.slashing_percentage();
        
        // Apply slash
        validator_set.slash_validator(evidence.validator_id, penalty);
        
        // Record
        self.processed_evidence.insert(evidence_hash, true);
        self.slash_history
            .entry(evidence.validator_id)
            .or_insert_with(Vec::new)
            .push(evidence.offense.clone());
        
        AdjudicationResult::ValidSlash(penalty)
    }
    
    pub fn is_evidence_processed(&self, evidence_hash: &[u8; 32]) -> bool {
        self.processed_evidence.contains_key(evidence_hash)
    }
    
    pub fn get_slash_history(&self, validator_id: u64) -> Vec<&ConstitutionalOffense> {
        self.slash_history
            .get(&validator_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
}

impl Default for SlashingAdjudicator {
    fn default() -> Self {
        Self::new()
    }
}
