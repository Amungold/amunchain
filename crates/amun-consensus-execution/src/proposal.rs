use amun_chain_position::ChainPosition;
use amun_quorum_certificate::QuorumCertificate;
use crate::commit::CommitRule;

#[derive(Debug, Clone)]
pub struct BlockProposal {
    pub proposer_id: u64,
    pub position: ChainPosition,
    pub round: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub parent_hash: Option<[u8; 32]>,
    pub justify_qc: Option<QuorumCertificate>,
}

impl BlockProposal {
    pub fn new(
        proposer_id: u64,
        position: ChainPosition,
        round: u64,
        block_hash: [u8; 32],
        state_root: [u8; 32],
        parent_hash: Option<[u8; 32]>,
    ) -> Self {
        Self {
            proposer_id,
            position,
            round,
            block_hash,
            state_root,
            parent_hash,
            justify_qc: None,
        }
    }

    pub fn verify(&self) -> bool {
        if self.position.sequence == 0 {
            return false;
        }
        if self.position.sequence > 1 && self.parent_hash.is_none() {
            return false;
        }
        true
    }

    pub fn is_safe(&self, commit_rule: &CommitRule) -> bool {
        if let Some(ref locked_qc) = commit_rule.locked_qc {
            if let Some(ref parent) = self.parent_hash {
                if !commit_rule.is_descendant(parent, &locked_qc.block_hash) {
                    if *parent != locked_qc.block_hash {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }
        true
    }
}
