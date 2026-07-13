use amun_evidence_root::EvidenceRoot;
use sha2::{Digest, Sha256};

/// Extends an existing EvidenceRoot with an NFT evidence root.
/// This preserves all existing constitutional evidence and adds NFT evidence binding.
pub fn extend_evidence_root_with_nft(
    original: &EvidenceRoot,
    nft_evidence_root: &[u8; 32],
) -> EvidenceRoot {
    let mut hasher = Sha256::new();
    hasher.update(b"AMUN_EVIDENCE_ROOT_V1");
    hasher.update(original.root);
    hasher.update(b"AMUN_NFT_EVIDENCE_V1");
    hasher.update(nft_evidence_root);
    let combined_root = hasher.finalize().into();

    EvidenceRoot {
        root: combined_root,
        state_root: original.state_root,
        commit_hash: original.commit_hash,
        replay_certificate: original.replay_certificate,
        audit_record: original.audit_record,
        previous_root: original.previous_root,
        height: original.height,
    }
}
