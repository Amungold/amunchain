use amun_kernel_types::PublicHash32;
use amun_consensus_types::ConsensusRound;
use crate::qc::QuorumCert;
use crate::validator::ValidatorSet;

pub struct SafetyRules;

impl SafetyRules {
    pub fn check_no_equivocation(new_round: ConsensusRound, new_hash: PublicHash32, locked_round: Option<ConsensusRound>, locked_hash: Option<PublicHash32>) -> bool {
        if let (Some(lr), Some(lh)) = (locked_round, locked_hash) {
            if new_round == lr && new_hash != lh { return false; }
        }
        true
    }
    pub fn check_lock_respected(new_round: ConsensusRound, locked_round: Option<ConsensusRound>, _locked_hash: Option<PublicHash32>, _proposal_hash: PublicHash32) -> bool {
        if let Some(lr) = locked_round { if new_round <= lr { return false; } }
        true
    }
    pub fn check_quorum(qc: &QuorumCert, validator_set: &ValidatorSet) -> bool { qc.is_valid(validator_set) }
    pub fn check_no_duplicate_signers(qc: &QuorumCert) -> bool {
        let len = qc.signer_indices.len();
        for i in 0..len { for j in (i+1)..len { if qc.signer_indices[i] == qc.signer_indices[j] { return false; } } }
        true
    }
}
