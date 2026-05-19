pub mod uniqueness;
pub mod equivocation;

pub use uniqueness::check_vote_uniqueness;
pub use equivocation::{EquivocationEvidence, detect_equivocation};
