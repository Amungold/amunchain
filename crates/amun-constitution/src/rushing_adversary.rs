// Rushing adversary model. Adversary sees honest messages before responding.

#[derive(Clone, Debug)]
pub struct RushingAdversary {
    pub max_corrupt_stake: u64,
    pub rushing: bool,
    pub adaptive: bool,
}
