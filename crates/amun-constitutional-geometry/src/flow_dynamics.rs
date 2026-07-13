use super::metric_tensor::MetricTensorField;

/// Constitutional Flow Dynamics.
/// Equations of motion describing how civilizations move through
/// constitutional phase space under the influence of attractors,
/// invariants, entropy gradients, and governance forces.
#[derive(Debug, Clone)]
pub struct ConstitutionalFlow {
    /// Current position in constitutional space (7-dimensional)
    pub position: [f64; 7],
    /// Current velocity (direction and speed of evolution)
    pub velocity: [f64; 7],
    /// The metric tensor at the current position
    pub local_metric: MetricTensorField,
    /// Forces acting on the civilization
    pub forces: Vec<ConstitutionalForce>,
    /// Total force vector
    pub total_force: [f64; 7],
}

#[derive(Debug, Clone)]
pub enum ConstitutionalForce {
    /// Attraction toward invariant stability
    InvariantForce {
        strength: f64,
        toward_hash: [u8; 32],
    },
    /// Attraction toward replay determinism
    ReplayForce { strength: f64 },
    /// Repulsion from high-entropy regions
    EntropyRepulsion { strength: f64, from_entropy: f64 },
    /// Attraction toward governance coherence
    GovernanceForce { strength: f64 },
    /// Attraction toward treaty networks
    TreatyForce { strength: f64, treaty_count: u64 },
    /// Resistance to change (constitutional inertia)
    InertialResistance { mass: f64 },
}

impl ConstitutionalFlow {
    pub fn new(initial_position: [f64; 7]) -> Self {
        Self {
            position: initial_position,
            velocity: [0.0; 7],
            local_metric: MetricTensorField::new([0u8; 32]),
            forces: Vec::new(),
            total_force: [0.0; 7],
        }
    }

    /// Add a force acting on the civilization.
    pub fn add_force(&mut self, force: ConstitutionalForce) {
        self.forces.push(force);
    }

    /// Compute the total force vector from all constituent forces.
    pub fn compute_total_force(&mut self) {
        let mut total = [0.0; 7];
        for force in &self.forces {
            match force {
                ConstitutionalForce::InvariantForce { strength, .. } => {
                    total[0] += strength; // Physics dimension
                }
                ConstitutionalForce::ReplayForce { strength } => {
                    total[1] += strength; // Replay dimension
                }
                ConstitutionalForce::EntropyRepulsion {
                    strength,
                    from_entropy,
                } => {
                    total[4] -= strength * (*from_entropy / 100.0); // Entropy dimension
                }
                ConstitutionalForce::GovernanceForce { strength } => {
                    total[5] += strength; // Treaty/governance dimension
                }
                ConstitutionalForce::TreatyForce { strength, .. } => {
                    total[5] += strength; // Treaty dimension
                }
                ConstitutionalForce::InertialResistance { mass } => {
                    // Inertia opposes motion in all dimensions
                    for i in 0..7 {
                        total[i] -= self.velocity[i] * mass * 0.1;
                    }
                }
            }
        }
        self.total_force = total;
    }

    /// Evolve the civilization by one time step using the equations of motion.
    /// dv/dt = F/m  (simplified constitutional dynamics)
    /// dx/dt = v
    pub fn evolve_step(&mut self, time_step: f64, constitutional_mass: f64) {
        self.compute_total_force();

        // Update velocity: dv = (F/m) * dt
        let inv_mass = if constitutional_mass > 0.0 {
            1.0 / constitutional_mass
        } else {
            1.0
        };
        for i in 0..7 {
            self.velocity[i] += self.total_force[i] * inv_mass * time_step;
        }

        // Update position: dx = v * dt
        for i in 0..7 {
            self.position[i] += self.velocity[i] * time_step;
        }

        // Clamp positions to valid range [0, 100]
        for i in 0..7 {
            self.position[i] = self.position[i].max(0.0).min(100.0);
        }
    }

    /// Check if the civilization is in a stable configuration.
    pub fn is_stable(&self) -> bool {
        let total_force_magnitude: f64 = self.total_force.iter().map(|f| f.abs()).sum();
        let total_velocity_magnitude: f64 = self.velocity.iter().map(|v| v.abs()).sum();
        total_force_magnitude < 0.01 && total_velocity_magnitude < 0.01
    }
}
