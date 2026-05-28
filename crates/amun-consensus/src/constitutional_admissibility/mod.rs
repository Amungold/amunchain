//! # Constitutional Admissibility Module
//! 
//! Defines admissibility states separate from aliveness.

use crate::constitutional_vote::ConstitutionalDecision;

// ============================================================
// Admissibility State
// ============================================================

/// Admissibility state - independent from aliveness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdmissibilityState {
    /// Fully admissible
    Allowed,
    /// Admissible with warning
    AllowedWithWarning,
    /// Conditionally admissible (needs review)
    Conditional,
    /// Not admissible
    Rejected,
}

impl AdmissibilityState {
    /// Convert from constitutional decision
    pub fn from_constitutional_decision(decision: ConstitutionalDecision) -> Self {
        match decision {
            ConstitutionalDecision::Admissible => Self::Allowed,
            ConstitutionalDecision::Warning => Self::AllowedWithWarning,
            ConstitutionalDecision::Shadowed => Self::Conditional,
            ConstitutionalDecision::Suffocated => Self::Rejected,
            ConstitutionalDecision::Necromancy => Self::Rejected,
            ConstitutionalDecision::Rejected => Self::Rejected,
        }
    }
    
    /// Check if the state is allowed
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed | Self::AllowedWithWarning | Self::Conditional)
    }
    
    /// Check if the state requires an alert
    pub fn requires_alert(&self) -> bool {
        matches!(self, Self::AllowedWithWarning | Self::Conditional)
    }
}

// ============================================================
// Admissibility Verifier
// ============================================================

/// Verifies constitutional admissibility
#[derive(Debug, Clone)]
pub struct AdmissibilityVerifier {
    origin_root: [u8; 32],
}

impl AdmissibilityVerifier {
    /// Create a new verifier
    pub fn new(origin_root: [u8; 32]) -> Self {
        Self { origin_root }
    }
    
    /// Verify admissibility based on authority dependencies
    pub fn verify(&self, authority_dependencies: &[String], suffocation_indicator: u64) -> AdmissibilityState {
        let origin_present = authority_dependencies.iter().any(|dep| dep == "origin_001");
        
        if !origin_present {
            return AdmissibilityState::Rejected;
        }
        
        if suffocation_indicator > 80 {
            return AdmissibilityState::Conditional;
        }
        
        if suffocation_indicator > 50 {
            return AdmissibilityState::AllowedWithWarning;
        }
        
        AdmissibilityState::Allowed
    }
    
    /// Simplified rule: origin must be in authority dependencies
    pub fn simple_rule(&self, authority_dependencies: &[String]) -> bool {
        authority_dependencies.contains(&"origin_001".to_string())
    }
    
    /// Get origin root
    pub fn origin_root(&self) -> [u8; 32] {
        self.origin_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_admissibility_from_decision() {
        assert!(AdmissibilityState::from_constitutional_decision(ConstitutionalDecision::Admissible).is_allowed());
        assert!(AdmissibilityState::from_constitutional_decision(ConstitutionalDecision::Warning).is_allowed());
        assert!(AdmissibilityState::from_constitutional_decision(ConstitutionalDecision::Shadowed).is_allowed());
        assert!(!AdmissibilityState::from_constitutional_decision(ConstitutionalDecision::Suffocated).is_allowed());
    }
    
    #[test]
    fn test_simple_rule() {
        let verifier = AdmissibilityVerifier::new([1u8; 32]);
        
        let deps_with_origin = vec!["origin_001".to_string(), "I2".to_string()];
        assert!(verifier.simple_rule(&deps_with_origin));
        
        let deps_without_origin = vec!["I2".to_string(), "I3".to_string()];
        assert!(!verifier.simple_rule(&deps_without_origin));
    }
}
