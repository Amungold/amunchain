use crate::commitment::ConstitutionalCommitment;
use crate::roots::{commitment_root, compute_constitutional_root};
use crate::Hash32;

pub struct Verifier;

#[derive(Debug, PartialEq)]
pub struct VerificationResult {
    pub constitutional_root_match: bool,
    pub commitment_root_match: bool,
    pub recomputed_constitutional_root: Hash32,
    pub recomputed_commitment_root: Hash32,
    pub stored_constitutional_root: Hash32,
    pub stored_commitment_root: Hash32,
}

impl Verifier {
    pub fn verify(
        identity_root: Hash32,
        evidence_root: Hash32,
        governance_root: Hash32,
        economic_root: Hash32,
        stored_constitutional_root: Hash32,
        stored_commitment_root: Hash32,
    ) -> VerificationResult {
        let recomputed_constitutional = compute_constitutional_root(
            identity_root,
            evidence_root,
            governance_root,
            economic_root,
        );

        let commitment = ConstitutionalCommitment {
            version: 1,
            identity_root,
            evidence_root,
            governance_root,
            economic_root,
            constitutional_root: recomputed_constitutional,
        };
        let recomputed_commitment = commitment_root(&commitment);

        VerificationResult {
            constitutional_root_match: recomputed_constitutional == stored_constitutional_root,
            commitment_root_match: recomputed_commitment == stored_commitment_root,
            recomputed_constitutional_root: recomputed_constitutional,
            recomputed_commitment_root: recomputed_commitment,
            stored_constitutional_root,
            stored_commitment_root,
        }
    }

    pub fn verified(result: &VerificationResult) -> bool {
        result.constitutional_root_match && result.commitment_root_match
    }
}
