#[derive(Debug, Clone)]
pub enum ByzantineStrategy {
    Equivocate,
    Silence,
    DelayMessages { delay_ms: u64 },
    Censor,
}
