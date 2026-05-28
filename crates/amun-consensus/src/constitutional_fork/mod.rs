//! # Constitutional Fork Module
//! 
//! Handles fork detection and resolution based on constitutional topology.

use crate::constitutional_vote::{ConstitutionalVote, ConstitutionalVoteSet, QuorumAnalysis};
use crate::constitutional_validator::ConstitutionalValidator;
use std::collections::BTreeMap;

// ============================================================
// Fork Classification
// ============================================================

/// Classification of constitutional forks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForkClassification {
    /// No fork - keys match
    NoFork,
    /// Same authority, different lineage
    LineageFork,
    /// Different authority, one matches origin
    AuthorityForkWithOrigin,
    /// Different authority, neither matches origin
    AuthorityForkSuffocated,
    /// Unknown classification
    Unknown,
}

impl std::fmt::Display for ForkClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFork => write!(f, "NO_FORK"),
            Self::LineageFork => write!(f, "LINEAGE_FORK"),
            Self::AuthorityForkWithOrigin => write!(f, "AUTHORITY_FORK_WITH_ORIGIN"),
            Self::AuthorityForkSuffocated => write!(f, "AUTHORITY_FORK_SUFFOCATED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

// ============================================================
// Fork Resolution
// ============================================================

/// Result of fork resolution
#[derive(Debug, Clone)]
pub struct ForkResolution {
    pub canonical_authority_root: [u8; 32],
    pub canonical_lineage_root: [u8; 32],
    pub canonical_fork_key: ([u8; 32], [u8; 32]),
    pub vote_count: usize,
    pub is_constitutionally_valid: bool,
    pub suffocation_detected: bool,
    pub has_topology_divergence: bool,
}

// ============================================================
// Fork Detector
// ============================================================

/// Detects and resolves constitutional forks
pub struct ConstitutionalForkDetector;

impl ConstitutionalForkDetector {
    /// Create a new fork detector
    pub fn new() -> Self {
        Self
    }
    
    /// Resolve fork by highest admissible quorum
    pub fn resolve_fork<T: ConstitutionalValidator>(
        &self,
        vote_sets: &[ConstitutionalVoteSet],
        validators: &BTreeMap<u64, T>,
    ) -> Option<ForkResolution> {
        let mut admissible_counts: BTreeMap<([u8; 32], [u8; 32]), usize> = BTreeMap::new();
        
        for set in vote_sets {
            for vote in &set.votes {
                if let Some(validator) = validators.get(&vote.validator_id) {
                    if validator.verify_admissibility(vote) {
                        *admissible_counts.entry(vote.fork_key()).or_insert(0) += 1;
                    }
                }
            }
        }
        
        let best = admissible_counts.iter().max_by_key(|(_, &count)| count);
        
        match best {
            Some((&(authority_root, lineage_root), &count)) => {
                let origin = if let Some(first_validator) = validators.values().next() {
                    first_validator.origin_root()
                } else {
                    [0u8; 32]
                };
                
                Some(ForkResolution {
                    canonical_authority_root: authority_root,
                    canonical_lineage_root: lineage_root,
                    canonical_fork_key: (authority_root, lineage_root),
                    vote_count: count,
                    is_constitutionally_valid: authority_root == origin,
                    suffocation_detected: authority_root != origin && count > 0,
                    has_topology_divergence: admissible_counts.len() > 1,
                })
            }
            None => None
        }
    }
    
    /// Analyze quorum without resolving
    pub fn analyze_quorum<T: ConstitutionalValidator>(
        &self,
        votes: &[ConstitutionalVote],
        validators: &BTreeMap<u64, T>,
        threshold: usize,
    ) -> QuorumAnalysis {
        let admissible_votes: Vec<&ConstitutionalVote> = votes.iter()
            .filter(|v| {
                validators.get(&v.validator_id)
                    .map(|vld| vld.verify_admissibility(v))
                    .unwrap_or(false)
            })
            .collect();
        
        let mut fork_key_counts: BTreeMap<([u8; 32], [u8; 32]), usize> = BTreeMap::new();
        for vote in admissible_votes {
            *fork_key_counts.entry(vote.fork_key()).or_insert(0) += 1;
        }
        
        QuorumAnalysis {
            has_quorum: fork_key_counts.values().any(|&count| count >= threshold),
            has_topology_divergence: fork_key_counts.len() > 1,
            fork_key_count: fork_key_counts.len(),
            max_quorum_size: fork_key_counts.values().max().copied().unwrap_or(0),
        }
    }
    
    /// Detect fork between two keys
    pub fn detect_fork(&self, key_a: ([u8; 32], [u8; 32]), key_b: ([u8; 32], [u8; 32])) -> bool {
        key_a != key_b
    }
    
    /// Classify a fork between two keys
    pub fn classify_fork(
        &self,
        key_a: ([u8; 32], [u8; 32]),
        key_b: ([u8; 32], [u8; 32]),
        origin_root: [u8; 32],
    ) -> ForkClassification {
        if key_a == key_b {
            return ForkClassification::NoFork;
        }
        
        let (auth_a, lineage_a) = key_a;
        let (auth_b, lineage_b) = key_b;
        
        if auth_a == auth_b && lineage_a != lineage_b {
            return ForkClassification::LineageFork;
        }
        
        if auth_a != auth_b {
            if auth_a == origin_root || auth_b == origin_root {
                return ForkClassification::AuthorityForkWithOrigin;
            }
            return ForkClassification::AuthorityForkSuffocated;
        }
        
        ForkClassification::Unknown
    }
}

impl Default for ConstitutionalForkDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constitutional_validator::SimpleConstitutionalValidator;
    use crate::constitutional_vote::AuthorityProof;
    
    fn origin_root() -> [u8; 32] {
        let mut root = [0u8; 32];
        root[0] = 1;
        root
    }
    
    fn create_vote(id: u64, auth: [u8; 32], lineage: [u8; 32]) -> ConstitutionalVote {
        ConstitutionalVote {
            validator_id: id,
            block_height: 100,
            authority_proof: AuthorityProof::new(true, auth),
            lineage_root: lineage,
            ..ConstitutionalVote::new(id, 100)
        }
    }
    
    fn create_validator(id: u64, origin: [u8; 32]) -> SimpleConstitutionalValidator {
        SimpleConstitutionalValidator {
            validator_id: id,
            authority_root: origin,
            lineage_root: origin,
            suffocation_indicator: 0,
            origin_root: origin,
        }
    }
    
    #[test]
    fn test_fork_classification() {
        let detector = ConstitutionalForkDetector::new();
        let origin = origin_root();
        let suffocated = [2u8; 32];
        let lineage1 = [3u8; 32];
        let lineage2 = [4u8; 32];
        
        assert_eq!(
            detector.classify_fork((origin, lineage1), (origin, lineage1), origin),
            ForkClassification::NoFork
        );
        
        assert_eq!(
            detector.classify_fork((origin, lineage1), (origin, lineage2), origin),
            ForkClassification::LineageFork
        );
        
        assert_eq!(
            detector.classify_fork((origin, lineage1), (suffocated, lineage2), origin),
            ForkClassification::AuthorityForkWithOrigin
        );
        
        let other1 = [5u8; 32];
        let other2 = [6u8; 32];
        assert_eq!(
            detector.classify_fork((other1, lineage1), (other2, lineage2), origin),
            ForkClassification::AuthorityForkSuffocated
        );
    }
    
    #[test]
    fn test_resolve_fork() {
        let detector = ConstitutionalForkDetector::new();
        let origin = origin_root();
        let lineage_a = [3u8; 32];
        let lineage_b = [4u8; 32];
        
        let mut validators = BTreeMap::new();
        for i in 0..5 {
            validators.insert(i, create_validator(i, origin));
        }
        
        let mut set = ConstitutionalVoteSet::new(100);
        for i in 0..3 {
            set.add_vote(create_vote(i, origin, lineage_a));
        }
        for i in 3..5 {
            set.add_vote(create_vote(i, origin, lineage_b));
        }
        
        let result = detector.resolve_fork(&[set], &validators);
        assert!(result.is_some());
        
        let result = result.unwrap();
        assert_eq!(result.canonical_fork_key, (origin, lineage_a));
        assert_eq!(result.vote_count, 3);
        assert!(result.has_topology_divergence);
    }
}
