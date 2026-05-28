use crate::state_tree::MerkleProof;
pub struct ProofLaw;
impl ProofLaw {
    pub fn validate(proof: &MerkleProof) -> bool { proof.steps.len() <= 256 }
}
