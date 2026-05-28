// Merged from amun-finality-law (Phase 48 Merge Strategy A)
// Functions only - tests remain in original crate until full migration
use amun_quorum_certificate::QuorumCertificate;

/// Check if a block is finalized with a direct 2-chain
pub fn is_finalized_simple(block_qc: &QuorumCertificate, child_qc: &QuorumCertificate) -> bool {
    if block_qc.votes.is_empty() || child_qc.votes.is_empty() {
        return false;
    }
    if child_qc.position.sequence != block_qc.position.sequence + 1 {
        return false;
    }
    if child_qc.parent_hash != block_qc.block_hash {
        return false;
    }
    true
}
