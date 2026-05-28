// Constitutional Amendment System
// Governed evolution through formal amendment proposals.

pub mod activation;
pub mod proposal;
pub mod ratification;

pub use activation::{ActivationEpoch, ActivationRules};
pub use proposal::{Amendment, AmendmentStatus, AmendmentType};
pub use ratification::{RatificationProof, RatificationQuorum};
