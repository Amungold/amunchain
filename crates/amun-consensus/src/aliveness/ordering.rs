use super::attestation::ConsensusAliveness;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlivenessSeverity { Living = 0, Stable = 1, Weakened = 2, Inert = 3, Critical = 4, Dead = 5 }
impl From<ConsensusAliveness> for AlivenessSeverity { fn from(s: ConsensusAliveness) -> Self { match s { ConsensusAliveness::Living => AlivenessSeverity::Living, ConsensusAliveness::Stable => AlivenessSeverity::Stable, ConsensusAliveness::Weakened => AlivenessSeverity::Weakened, ConsensusAliveness::Inert => AlivenessSeverity::Inert, ConsensusAliveness::Critical => AlivenessSeverity::Critical, ConsensusAliveness::Dead => AlivenessSeverity::Dead } } }
pub struct WeightedAliveness;
impl WeightedAliveness { pub fn determine_consensus(votes: &[(ConsensusAliveness, usize)], thresh: usize) -> Option<ConsensusAliveness> { let mut valid: Vec<_> = votes.iter().filter(|(_,c)| *c >= thresh).copied().collect(); if valid.is_empty() { return None; } valid.sort_by(|a,b| { let sev_b: AlivenessSeverity = b.0.into(); let sev_a: AlivenessSeverity = a.0.into(); sev_b.cmp(&sev_a) }); Some(valid[0].0) } }
