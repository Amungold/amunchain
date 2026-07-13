pub mod equivocation;
pub mod uniqueness;

pub use equivocation::{detect_equivocation, EquivocationEvidence};
pub use uniqueness::check_vote_uniqueness;
