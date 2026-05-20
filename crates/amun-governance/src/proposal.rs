#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProposalType {
    Text,
    ParameterChange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProposalStatus {
    Deposit,
    Voting,
    Passed,
    Rejected,
    Executed,
}

pub struct Proposal {
    pub id: u64,
    pub status: ProposalStatus,
    pub yes: u64,
    pub no: u64,
    pub veto: u64,
    pub end_block: u64,
}

impl Proposal {
    pub fn new(id: u64, end: u64) -> Self {
        Self {
            id,
            status: ProposalStatus::Deposit,
            yes: 0,
            no: 0,
            veto: 0,
            end_block: end,
        }
    }
    pub fn is_passing(&self, total: u64) -> bool {
        self.yes > self.no && self.yes > total * 33 / 100
    }
}
