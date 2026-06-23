use crate::commitment::ConstitutionalCommitment;

pub const SERIALIZED_LENGTH: usize = 162;

pub fn serialize_v1(commitment: &ConstitutionalCommitment) -> [u8; SERIALIZED_LENGTH] {
    let mut buf = [0u8; SERIALIZED_LENGTH];
    let mut offset = 0;

    buf[offset..offset + 2].copy_from_slice(&commitment.version.to_be_bytes());
    offset += 2;

    buf[offset..offset + 32].copy_from_slice(&commitment.identity_root);
    offset += 32;

    buf[offset..offset + 32].copy_from_slice(&commitment.evidence_root);
    offset += 32;

    buf[offset..offset + 32].copy_from_slice(&commitment.governance_root);
    offset += 32;

    buf[offset..offset + 32].copy_from_slice(&commitment.economic_root);
    offset += 32;

    buf[offset..offset + 32].copy_from_slice(&commitment.constitutional_root);

    buf
}
