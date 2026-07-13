// Network synchrony model.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SynchronyModel {
    Async,
    PartiallySynchronous { delta_rounds: u32 },
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalStabilizationTime {
    pub gst_round: u64,
    pub delta_rounds: u32,
}

#[derive(Clone, Debug)]
pub struct SynchronyProof {
    pub sent_at_round: u64,
    pub received_at_round: u64,
    pub claimed_delta: u32,
}

impl SynchronyProof {
    pub fn verify(&self) -> Result<(), &'static str> {
        let actual = self.received_at_round.saturating_sub(self.sent_at_round);
        if actual <= self.claimed_delta as u64 {
            Ok(())
        } else {
            Err("Synchrony bound violated")
        }
    }
}
