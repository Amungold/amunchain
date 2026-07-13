use crate::Hash32;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConstitutionalCommitment {
    pub version: u16,
    pub identity_root: Hash32,
    pub evidence_root: Hash32,
    pub governance_root: Hash32,
    pub economic_root: Hash32,
    pub constitutional_root: Hash32,
}
