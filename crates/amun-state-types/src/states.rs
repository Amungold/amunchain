// State markers for three-dimensional state tracking.

pub struct Unverified;
pub struct Verified;
pub struct Committed;
pub struct Finalized;

pub struct Volatile;
pub struct Durable;
pub struct Journaled;
pub struct Snapshotted;

pub struct Proposed;
pub struct Voted;
pub struct QuorumCertified;
pub struct Executed;
