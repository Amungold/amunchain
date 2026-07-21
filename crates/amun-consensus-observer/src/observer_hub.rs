use crate::events::ConsensusEvent;
use crate::observer::ConsensusObserver;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct ObserverHub {
    observers: RwLock<Vec<Arc<dyn ConsensusObserver>>>,
    event_counter: AtomicU64,
    sequence_counter: AtomicU64,
    round_correlation_counter: AtomicU64,
}

impl ObserverHub {
    pub fn new() -> Self {
        Self {
            observers: RwLock::new(Vec::new()),
            event_counter: AtomicU64::new(1),
            sequence_counter: AtomicU64::new(0),
            round_correlation_counter: AtomicU64::new(1),
        }
    }

    /// Register an observer (shared ownership via Arc)
    pub fn register<O: ConsensusObserver + 'static>(&self, observer: Arc<O>) {
        let mut observers = self.observers.write();
        observers.push(observer);
    }

    /// Emit an event to all registered observers
    pub fn emit(&self, event: ConsensusEvent) {
        let observers = self.observers.read();
        for observer in observers.iter() {
            observer.on_event(&event);
        }
    }

    pub fn next_event_id(&self) -> u64 {
        self.event_counter.fetch_add(1, Ordering::SeqCst)
    }

    pub fn next_sequence(&self) -> u64 {
        self.sequence_counter.fetch_add(1, Ordering::SeqCst)
    }

    pub fn next_correlation_id(&self) -> u64 {
        self.round_correlation_counter
            .fetch_add(1, Ordering::SeqCst)
    }

    pub fn event_count(&self) -> u64 {
        self.event_counter.load(Ordering::SeqCst)
    }

    pub fn reset_all(&self) {
        let observers = self.observers.read();
        for observer in observers.iter() {
            observer.reset();
        }
    }
}

impl Default for ObserverHub {
    fn default() -> Self {
        Self::new()
    }
}
