//! # Constitutional Validator Module
//! 
//! Defines the validator trait for constitution-aware consensus.

use crate::constitutional_vote::{ConstitutionalVote, ConstitutionalDecision, AuthorityProof};

// ============================================================
// Constitutional Validator Trait
// ============================================================

/// A validator that participates in constitution-aware consensus
pub trait ConstitutionalValidator {
    /// Get the validator's authority proof
    fn authority_proof(&self) -> AuthorityProof;
    
    /// Get the validator's lineage root
    fn lineage_root(&self) -> [u8; 32];
    
    /// Get the validator's suffocation indicator
    fn suffocation_indicator(&self) -> u64;
    
    /// Get the constitutional origin root
    fn origin_root(&self) -> [u8; 32];
    
    /// Validate a constitutional vote
    fn validate_constitutional_vote(&self, vote: &ConstitutionalVote) -> ConstitutionalDecision {
        // Rule 1: Necromancy check
        if vote.suffocation_indicator == 100 && vote.authority_proof.authority_root == [0u8; 32] {
            return ConstitutionalDecision::Necromancy;
        }
        
        // Rule 2: Suffocation check (origin missing)
        if !vote.has_origin() {
            return ConstitutionalDecision::Suffocated;
        }
        
        let origin = self.origin_root();
        
        // Rule 3: Shadowing detection (authority changed but origin present)
        if vote.authority_proof.authority_root != origin && vote.has_origin() {
            return ConstitutionalDecision::Shadowed;
        }
        
        // Rule 4: Warning threshold
        if vote.suffocation_indicator > 50 {
            return ConstitutionalDecision::Warning;
        }
        
        ConstitutionalDecision::Admissible
    }
    
    /// Verify if a vote is admissible
    fn verify_admissibility(&self, vote: &ConstitutionalVote) -> bool {
        self.validate_constitutional_vote(vote).is_admissible()
    }
    
    /// Check if a vote requires a constitutional alert
    fn requires_alert(&self, vote: &ConstitutionalVote) -> bool {
        self.validate_constitutional_vote(vote).requires_alert()
    }
}

// ============================================================
// Simple Implementation
// ============================================================

/// Simple implementation of ConstitutionalValidator for testing
#[derive(Debug, Clone)]
pub struct SimpleConstitutionalValidator {
    pub validator_id: u64,
    pub authority_root: [u8; 32],
    pub lineage_root: [u8; 32],
    pub suffocation_indicator: u64,
    pub origin_root: [u8; 32],
}

impl ConstitutionalValidator for SimpleConstitutionalValidator {
    fn authority_proof(&self) -> AuthorityProof {
        let origin_present = self.authority_root == self.origin_root;
        AuthorityProof::new(origin_present, self.authority_root)
    }
    
    fn lineage_root(&self) -> [u8; 32] {
        self.lineage_root
    }
    
    fn suffocation_indicator(&self) -> u64 {
        self.suffocation_indicator
    }
    
    fn origin_root(&self) -> [u8; 32] {
        self.origin_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constitutional_vote::AuthorityProof;
    
    fn origin_root() -> [u8; 32] {
        let mut root = [0u8; 32];
        root[0] = 1;
        root
    }
    
    fn create_vote(origin_present: bool, authority_root: [u8; 32], indicator: u64) -> ConstitutionalVote {
        ConstitutionalVote {
            validator_id: 1,
            block_height: 100,
            authority_proof: AuthorityProof::new(origin_present, authority_root),
            suffocation_indicator: indicator,
            ..ConstitutionalVote::new(1, 100)
        }
    }
    
    #[test]
    fn test_validator_admissible() {
        let origin = origin_root();
        let validator = SimpleConstitutionalValidator {
            validator_id: 1,
            authority_root: origin,
            lineage_root: origin,
            suffocation_indicator: 0,
            origin_root: origin,
        };
        
        let vote = create_vote(true, origin, 0);
        assert!(validator.verify_admissibility(&vote));
        assert_eq!(validator.validate_constitutional_vote(&vote), ConstitutionalDecision::Admissible);
    }
    
    #[test]
    fn test_validator_shadowed() {
        let origin = origin_root();
        let shadowed = [2u8; 32];
        let validator = SimpleConstitutionalValidator {
            validator_id: 1,
            authority_root: origin,
            lineage_root: origin,
            suffocation_indicator: 0,
            origin_root: origin,
        };
        
        let vote = create_vote(true, shadowed, 50);
        let decision = validator.validate_constitutional_vote(&vote);
        assert_eq!(decision, ConstitutionalDecision::Shadowed);
        assert!(decision.is_admissible());
        assert!(decision.requires_alert());
    }
    
    #[test]
    fn test_validator_suffocated() {
        let origin = origin_root();
        let validator = SimpleConstitutionalValidator {
            validator_id: 1,
            authority_root: origin,
            lineage_root: origin,
            suffocation_indicator: 0,
            origin_root: origin,
        };
        
        let vote = create_vote(false, [3u8; 32], 100);
        assert!(!validator.verify_admissibility(&vote));
        assert_eq!(validator.validate_constitutional_vote(&vote), ConstitutionalDecision::Suffocated);
    }
}
