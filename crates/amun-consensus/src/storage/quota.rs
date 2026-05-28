use std::sync::atomic::{AtomicUsize, Ordering};
pub struct StorageQuota {
    used: AtomicUsize,
    limit: usize,
}
impl StorageQuota {
    pub fn new(limit: usize) -> Self { Self { used: AtomicUsize::new(0), limit } }
    pub fn reserve(&self, bytes: usize) -> Result<(), &'static str> {
        let mut cur = self.used.load(Ordering::Relaxed);
        loop {
            if cur + bytes > self.limit { return Err("Quota exceeded"); }
            match self.used.compare_exchange_weak(cur, cur + bytes, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => return Ok(()),
                Err(x) => cur = x,
            }
        }
    }
    pub fn release(&self, bytes: usize) { self.used.fetch_sub(bytes, Ordering::SeqCst); }
}
