#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerificationPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}
