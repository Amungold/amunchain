use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

/// Unified lifecycle interface for all runtime services.
///
/// ADR-023 Phase 6: Every runtime service implements this trait,
/// allowing NodeRuntime to manage them uniformly.
pub trait RuntimeService: Send + Sync {
    fn start(&self) -> Result<Vec<JoinHandle<()>>, String>;
    fn stop(&self);
    fn is_running(&self) -> bool;
}

/// NodeRuntime owns all services and manages their lifecycle.
///
/// ADR-023 Phase 6: Replaces direct service ownership in LiveValidator.
/// LiveValidator becomes a thin facade over NodeRuntime.
pub struct NodeRuntime {
    services: Vec<Box<dyn RuntimeService>>,
    running: Arc<Mutex<bool>>,
}

impl NodeRuntime {
    pub fn new(running: Arc<Mutex<bool>>) -> Self {
        Self {
            services: Vec::new(),
            running,
        }
    }

    /// Register a service to be managed by this runtime.
    pub fn register(&mut self, service: Box<dyn RuntimeService>) {
        self.services.push(service);
    }

    /// Start all registered services in order.
    /// Returns all thread handles for lifecycle management.
    pub fn start_all(&self) -> Result<Vec<JoinHandle<()>>, String> {
        *self.running.lock().unwrap() = true;
        let mut handles = Vec::new();
        for service in &self.services {
            let h = service.start()?;
            handles.extend(h);
        }
        Ok(handles)
    }

    /// Stop all registered services in reverse order.
    pub fn stop_all(&self) {
        *self.running.lock().unwrap() = false;
        for service in self.services.iter().rev() {
            service.stop();
        }
    }
}
