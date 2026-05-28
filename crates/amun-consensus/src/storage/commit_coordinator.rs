use std::sync::Mutex;
pub struct CommitCoordinator { lock: Mutex<()> }
impl CommitCoordinator {
    pub fn new() -> Self { Self { lock: Mutex::new(()) } }
    pub fn serialize_commit<F, T>(&self, f: F) -> T where F: FnOnce() -> T {
        let _g = self.lock.lock().unwrap();
        f()
    }
}
