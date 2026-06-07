use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::types::{ClaimAction, Jurisdiction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub sovereigns: Vec<Sovereign>,
    pub recognition: Vec<Vec<bool>>,
    pub treaties: Vec<Vec<bool>>,
    pub jurisdictions: Vec<Jurisdiction>,
    pub claims: Vec<LegitimacyClaim>,
    pub effectiveness: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sovereign {
    pub id: usize,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegitimacyClaim {
    pub issuer: usize,
    pub subject: usize,
    pub action: ClaimAction,
    pub scope: Jurisdiction,
    pub epoch_start: u64,
    pub epoch_end: u64,
}

impl SimulationState {
    pub fn new(num_sovereigns: usize, recognition_density: f64, treaty_density: f64) -> Self {
        let mut rng = rand::thread_rng();

        let sovereigns: Vec<Sovereign> = (0..num_sovereigns)
            .map(|id| Sovereign { id, weight: 1.0 })
            .collect();

        let mut recognition = vec![vec![false; num_sovereigns]; num_sovereigns];
        for i in 0..num_sovereigns {
            for j in 0..num_sovereigns {
                if i != j && rng.gen::<f64>() < recognition_density {
                    recognition[i][j] = true;
                }
            }
        }

        let mut treaties = vec![vec![false; num_sovereigns]; num_sovereigns];
        for i in 0..num_sovereigns {
            for j in (i+1)..num_sovereigns {
                if rng.gen::<f64>() < treaty_density {
                    treaties[i][j] = true;
                    treaties[j][i] = true;
                }
            }
        }

        // Multiple regions: 2-5 per jurisdiction
        let jurisdictions: Vec<Jurisdiction> = (0..num_sovereigns)
            .map(|_| {
                let num_regions = rng.gen_range(2..6);
                Jurisdiction {
                    regions: (0..num_regions).map(|_| rng.gen_range(0..10)).collect(),
                }
            })
            .collect();

        let actions = vec![
            ClaimAction::Govern, ClaimAction::Trade, ClaimAction::Treaty,
            ClaimAction::Recognize, ClaimAction::Tax, ClaimAction::Defend,
        ];

        let claims: Vec<LegitimacyClaim> = (0..num_sovereigns)
            .map(|i| {
                let mut subject = (i + 1) % num_sovereigns;
                if num_sovereigns == 1 { subject = 0; }
                LegitimacyClaim {
                    issuer: i,
                    subject,
                    action: actions[rng.gen_range(0..actions.len())].clone(),
                    scope: jurisdictions[i].clone(),
                    epoch_start: 0,
                    epoch_end: 10000,
                }
            })
            .collect();

        let effectiveness = vec![0.0; claims.len()];

        Self { sovereigns, recognition, treaties, jurisdictions, claims, effectiveness }
    }
}
