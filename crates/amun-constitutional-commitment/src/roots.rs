use crate::commitment::ConstitutionalCommitment;
use crate::serializer::serialize_v1;
use crate::Hash32;

const DOMAIN_CONSTITUTIONAL_ROOT: &[u8] = b"AMUN_CONSTITUTIONAL_ROOT_V1";
const DOMAIN_COMMITMENT_ROOT: &[u8] = b"AMUN_CONSTITUTIONAL_COMMITMENT_V1";

pub fn compute_constitutional_root(
    identity_root: Hash32,
    evidence_root: Hash32,
    governance_root: Hash32,
    economic_root: Hash32,
) -> Hash32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_CONSTITUTIONAL_ROOT);
    hasher.update(&identity_root);
    hasher.update(&evidence_root);
    hasher.update(&governance_root);
    hasher.update(&economic_root);
    *hasher.finalize().as_bytes()
}

pub fn commitment_root(commitment: &ConstitutionalCommitment) -> Hash32 {
    let serialized = serialize_v1(commitment);
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN_COMMITMENT_ROOT);
    hasher.update(&serialized);
    *hasher.finalize().as_bytes()
}
