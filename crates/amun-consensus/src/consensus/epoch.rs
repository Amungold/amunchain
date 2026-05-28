use crate::crypto::types::ConstitutionalEpoch;

#[derive(Debug, Clone)]
pub struct ValidatorChange { pub validator_id: u64, pub new_power: Option<u64> }

#[derive(Debug, Clone)]
pub struct EpochChangeProposal { pub new_epoch: ConstitutionalEpoch, pub changes: Vec<ValidatorChange>, pub proposer: u64 }

#[derive(Debug, Clone)]
pub struct EpochCoordinator { current_epoch: ConstitutionalEpoch, pending_changes: Vec<EpochChangeProposal>, threshold: usize }
impl EpochCoordinator {
    pub fn new(initial_epoch: ConstitutionalEpoch) -> Self { Self { current_epoch: initial_epoch, pending_changes: Vec::new(), threshold: 2 } }
    pub fn propose_change(&mut self, proposal: EpochChangeProposal) { self.pending_changes.push(proposal); }
    pub fn try_apply(&mut self, approvals: usize) -> bool { if approvals < self.threshold { return false; } for proposal in self.pending_changes.drain(..) { self.current_epoch = proposal.new_epoch; } true }
    pub fn current_epoch(&self) -> ConstitutionalEpoch { self.current_epoch }
}
