//! Vote hashing

pub const VOTE_DOMAIN: &[u8] = b"AMUN_CONSTITUTIONAL_VOTE_V1";
pub const QC_DOMAIN: &[u8] = b"AMUN_CONSTITUTIONAL_QC_V1";

pub struct CanonicalVote;

impl CanonicalVote {
    pub fn hash() -> [u8; 32] {
        [0u8; 32]
    }
}
