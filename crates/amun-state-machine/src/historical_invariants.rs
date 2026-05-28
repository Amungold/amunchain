/// Historical invariants that span multiple transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoricalInvariant {
    /// No freeze/unfreeze oscillation (max N cycles per epoch)
    MaxFreezeUnfreezeCycles {
        max_per_epoch: u64,
        current_count: u64,
    },
    /// No amendment oscillation (max N active amendments per epoch)
    MaxAmendmentsPerEpoch {
        max_per_epoch: u64,
        current_count: u64,
    },
    /// Replay divergence must not accumulate over lineage
    MaxReplayDivergence {
        max_cumulative_bytes: u64,
        current: u64,
    },
    /// Governance must not degrade below minimum compatibility
    MinGovernanceCompatibility { min_rank: u8, current_rank: u8 },
}

impl HistoricalInvariant {
    pub fn check(&self) -> Result<(), String> {
        match self {
            HistoricalInvariant::MaxFreezeUnfreezeCycles {
                max_per_epoch,
                current_count,
            } => {
                if current_count > max_per_epoch {
                    return Err(format!(
                        "Freeze/unfreeze cycle limit exceeded: {} > {}",
                        current_count, max_per_epoch
                    ));
                }
            }
            HistoricalInvariant::MaxAmendmentsPerEpoch {
                max_per_epoch,
                current_count,
            } => {
                if current_count > max_per_epoch {
                    return Err(format!(
                        "Amendment limit exceeded: {} > {}",
                        current_count, max_per_epoch
                    ));
                }
            }
            HistoricalInvariant::MaxReplayDivergence {
                max_cumulative_bytes,
                current,
            } => {
                if current > max_cumulative_bytes {
                    return Err(format!(
                        "Replay divergence accumulation exceeded: {} > {}",
                        current, max_cumulative_bytes
                    ));
                }
            }
            HistoricalInvariant::MinGovernanceCompatibility {
                min_rank,
                current_rank,
            } => {
                if current_rank < min_rank {
                    return Err(format!(
                        "Governance compatibility degraded: {} < {}",
                        current_rank, min_rank
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Engine that tracks historical invariants across multiple transitions.
#[derive(Debug, Clone)]
pub struct HistoricalInvariantEngine {
    pub freeze_unfreeze_count: u64,
    pub amendments_this_epoch: u64,
    pub cumulative_replay_divergence: u64,
    pub governance_compatibility_rank: u8,
    pub current_epoch: u64,
}

impl Default for HistoricalInvariantEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoricalInvariantEngine {
    pub fn new() -> Self {
        Self {
            freeze_unfreeze_count: 0,
            amendments_this_epoch: 0,
            cumulative_replay_divergence: 0,
            governance_compatibility_rank: 5,
            current_epoch: 0,
        }
    }

    pub fn record_freeze_unfreeze(&mut self) {
        self.freeze_unfreeze_count += 1;
    }
    pub fn record_amendment(&mut self) {
        self.amendments_this_epoch += 1;
    }
    pub fn record_replay_divergence(&mut self, bytes: u64) {
        self.cumulative_replay_divergence += bytes;
    }
    pub fn update_governance_rank(&mut self, rank: u8) {
        self.governance_compatibility_rank = rank;
    }

    pub fn new_epoch(&mut self, epoch: u64) {
        if epoch > self.current_epoch {
            self.current_epoch = epoch;
            self.freeze_unfreeze_count = 0;
            self.amendments_this_epoch = 0;
        }
    }

    pub fn check_all(&self) -> Result<(), Vec<String>> {
        let invariants = vec![
            HistoricalInvariant::MaxFreezeUnfreezeCycles {
                max_per_epoch: 3,
                current_count: self.freeze_unfreeze_count,
            },
            HistoricalInvariant::MaxAmendmentsPerEpoch {
                max_per_epoch: 5,
                current_count: self.amendments_this_epoch,
            },
            HistoricalInvariant::MaxReplayDivergence {
                max_cumulative_bytes: 1024,
                current: self.cumulative_replay_divergence,
            },
            HistoricalInvariant::MinGovernanceCompatibility {
                min_rank: 2,
                current_rank: self.governance_compatibility_rank,
            },
        ];
        let mut errors = Vec::new();
        for inv in &invariants {
            if let Err(e) = inv.check() {
                errors.push(e);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
