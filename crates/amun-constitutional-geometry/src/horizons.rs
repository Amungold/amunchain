/// A causal horizon represents a boundary in constitutional spacetime
/// beyond which legitimate interaction is impossible.
#[derive(Debug, Clone)]
pub struct CausalHorizon {
    pub horizon_type: HorizonType,
    pub distance_to_horizon: f64,
    pub is_approaching: bool,
    pub is_crossed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HorizonType {
    /// Physics horizon: different physical universes cannot interact
    PhysicsHorizon,
    /// Replay horizon: replay divergence has made reconciliation impossible
    ReplayHorizon,
    /// Entropy horizon: constitutional entropy has exceeded collapse threshold
    EntropyHorizon,
    /// Treaty horizon: no treaty exists and none can be established
    TreatyHorizon,
    /// Temporal horizon: epoch separation exceeds reconciliation limit
    TemporalHorizon,
    /// Lineage horizon: ancestry divergence is irreconcilable
    LineageHorizon,
}

impl CausalHorizon {
    pub fn new(horizon_type: HorizonType, distance: f64) -> Self {
        Self {
            horizon_type,
            distance_to_horizon: distance,
            is_approaching: distance < 10.0,
            is_crossed: distance <= 0.0,
        }
    }

    /// Can interaction still occur across this horizon?
    pub fn can_interact(&self) -> bool {
        !self.is_crossed
    }

    /// Is the civilization in danger of crossing this horizon?
    pub fn warning(&self) -> Option<String> {
        if self.is_approaching && !self.is_crossed {
            Some(format!("Approaching {:?} horizon", self.horizon_type))
        } else if self.is_crossed {
            Some(format!(
                "Crossed {:?} horizon - interaction impossible",
                self.horizon_type
            ))
        } else {
            None
        }
    }
}
