// Deterministic execution marker — safe marker trait.
// The unsafe version lives in amun-unsafe per constitutional law.

pub trait DeterministicExecution: Sized {}

impl DeterministicExecution for u8 {}
impl DeterministicExecution for u16 {}
impl DeterministicExecution for u32 {}
impl DeterministicExecution for u64 {}
impl DeterministicExecution for [u8; 32] {}
impl DeterministicExecution for [u8; 64] {}
