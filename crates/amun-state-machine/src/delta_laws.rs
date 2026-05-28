use super::delta_algebra::ConstitutionalDelta;

/// Delta algebra laws governing composition, conflict, and domination
/// of constitutional changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeltaLaw {
    /// Two deltas of the same type compose into one delta
    Composable(ConstitutionalDelta, ConstitutionalDelta),
    /// Two deltas conflict and cannot coexist
    Conflicting(ConstitutionalDelta, ConstitutionalDelta),
    /// One delta dominates another (superset of semantic change)
    Dominates(ConstitutionalDelta, ConstitutionalDelta),
    /// Deltas commute (order does not matter)
    Commutative(ConstitutionalDelta, ConstitutionalDelta),
}

impl DeltaLaw {
    /// Determine the relationship between two deltas.
    pub fn relate(a: &ConstitutionalDelta, b: &ConstitutionalDelta) -> DeltaLaw {
        if a.canonical_tag() != b.canonical_tag() {
            return DeltaLaw::Commutative(a.clone(), b.clone());
        }
        match (a, b) {
            (
                ConstitutionalDelta::GovernanceDelta { .. },
                ConstitutionalDelta::GovernanceDelta { .. },
            ) => DeltaLaw::Composable(a.clone(), b.clone()),
            (
                ConstitutionalDelta::IdentityDelta { .. },
                ConstitutionalDelta::IdentityDelta { .. },
            ) => DeltaLaw::Conflicting(a.clone(), b.clone()),
            (
                ConstitutionalDelta::ReplayDelta {
                    old_guarantee_rank: _,
                    new_guarantee_rank: na,
                },
                ConstitutionalDelta::ReplayDelta {
                    old_guarantee_rank: _,
                    new_guarantee_rank: nb,
                },
            ) => {
                if na >= nb {
                    DeltaLaw::Dominates(a.clone(), b.clone())
                } else {
                    DeltaLaw::Dominates(b.clone(), a.clone())
                }
            }
            _ => DeltaLaw::Composable(a.clone(), b.clone()),
        }
    }

    pub fn description(&self) -> String {
        match self {
            DeltaLaw::Composable(_, _) => "Deltas compose into single change".to_string(),
            DeltaLaw::Conflicting(_, _) => "Deltas conflict and cannot coexist".to_string(),
            DeltaLaw::Dominates(a, _) => {
                format!("Delta {:?} dominates the other", a.canonical_tag())
            }
            DeltaLaw::Commutative(_, _) => "Deltas commute (order independent)".to_string(),
        }
    }
}
