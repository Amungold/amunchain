use amun_kernel_types::PublicKey; use heapless::Vec;
pub struct UnbondingEntry { pub validator: PublicKey, pub amount: u64, pub unlock: u64 }
pub struct UnbondingQueue { entries: Vec<UnbondingEntry, 64>, pub period: u64 }
impl UnbondingQueue { pub fn new() -> Self { Self { entries: Vec::new(), period: 1000 } } pub fn enqueue(&mut self, v: PublicKey, a: u64, cur: u64) -> Result<(), &str> { if self.entries.is_full() { return Err("full"); } self.entries.push(UnbondingEntry { validator: v, amount: a, unlock: cur + self.period }).map_err(|_| "push") } }
