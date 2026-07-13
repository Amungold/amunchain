use amun_governance::proposal::{Proposal, ProposalType};
use crate::types::SdkResult;

pub struct GovernanceApi {
    pub proposals: heapless::Vec<Proposal, 32>,
    pub next_id: u64,
}

impl GovernanceApi {
    pub fn new() -> Self { Self { proposals: heapless::Vec::new(), next_id: 1 } }

    pub fn create_proposal(&mut self, _proposer: amun_kernel_types::PublicHash32, _proposal_type: ProposalType, end_block: u64) -> SdkResult<u64> {
        let id = self.next_id;
        let p = Proposal::new(id, end_block);
        if self.proposals.push(p).is_err() { return SdkResult::err("Too many proposals"); }
        self.next_id += 1;
        SdkResult::ok(id)
    }

    pub fn vote(&mut self, proposal_id: u64, yes: bool) -> SdkResult<()> {
        if let Some(p) = self.proposals.iter_mut().find(|p| p.id == proposal_id) {
            if yes { p.yes += 1 } else { p.no += 1 }
            return SdkResult::ok(());
        }
        SdkResult::err("Proposal not found")
    }

    pub fn get_proposal_status(&self, proposal_id: u64, total_stake: u64) -> SdkResult<bool> {
        if let Some(p) = self.proposals.iter().find(|p| p.id == proposal_id) {
            return SdkResult::ok(p.is_passing(total_stake));
        }
        SdkResult::err("Proposal not found")
    }

    pub fn proposal_count(&self) -> SdkResult<usize> { SdkResult::ok(self.proposals.len()) }
}
