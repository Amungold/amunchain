use crate::observer::TransitionObserver;
use amun_validator_api::error::PlatformResult;
use amun_validator_api::types::id::ValidatorId;
use amun_validator_api::types::state::{RuntimeState, StateMachine};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug, Clone)]
pub struct TransitionResult {
    pub from: RuntimeState,
    pub to: RuntimeState,
    pub timestamp_ms: u64,
    pub transition_count: u64,
    pub duration_ms: u64,
    pub node_id: ValidatorId,
    pub reason: Option<String>,
    pub sequence: u64,
    pub is_forced: bool,
}

#[derive(Debug, Clone)]
struct RuntimeStateData {
    current: RuntimeState,
    previous: Option<RuntimeState>,
    entered_at_ms: u64,
    transition_count: u64,
    sequence: u64,
}

impl RuntimeStateData {
    fn new(state: RuntimeState) -> Self {
        RuntimeStateData {
            current: state,
            previous: None,
            entered_at_ms: Self::now_ms(),
            transition_count: 0,
            sequence: 0,
        }
    }
    pub fn now_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

pub struct RuntimeStateMachine {
    machine: StateMachine,
    state: Mutex<RuntimeStateData>,
    observers: RwLock<Vec<Arc<dyn TransitionObserver>>>,
    node_id: ValidatorId,
}

impl RuntimeStateMachine {
    pub fn new(node_id: ValidatorId) -> Self {
        RuntimeStateMachine {
            machine: StateMachine::new(),
            state: Mutex::new(RuntimeStateData::new(RuntimeState::Created)),
            observers: RwLock::new(Vec::new()),
            node_id,
        }
    }

    pub fn attach_observer(&self, observer: Arc<dyn TransitionObserver>) {
        self.observers
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .push(observer);
    }

    fn clone_observers(&self) -> Vec<Arc<dyn TransitionObserver>> {
        self.observers
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    pub fn current(&self) -> RuntimeState {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).current
    }

    pub fn previous(&self) -> Option<RuntimeState> {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .previous
    }

    pub fn transition_count(&self) -> u64 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .transition_count
    }

    pub fn entered_at_ms(&self) -> u64 {
        self.state
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .entered_at_ms
    }

    /// Atomically validates and commits a state transition.
    /// The validate+update happens inside a single critical section.
    /// Observers are notified AFTER the commit (state is already new).
    /// Use `event.from` to know the old state.
    /// Calling `current()` during observer will return the new state.
    pub fn transition(&self, to: RuntimeState) -> PlatformResult<TransitionResult> {
        let start_ms = RuntimeStateData::now_ms();
        let observers = self.clone_observers();

        let committed_event;
        {
            let mut data = self.state.lock().unwrap_or_else(|e| e.into_inner());
            let from = data.current;
            if let Err(e) = self.machine.validate_transition(from, to) {
                drop(data);
                for obs in &observers {
                    obs.on_transition_failed(from, to, &e.to_string());
                }
                return Err(e);
            }

            data.previous = Some(from);
            data.current = to;
            data.entered_at_ms = RuntimeStateData::now_ms();
            data.transition_count += 1;
            data.sequence += 1;
            let count = data.transition_count;
            let seq = data.sequence;

            committed_event = TransitionResult {
                from,
                to,
                timestamp_ms: RuntimeStateData::now_ms(),
                transition_count: count,
                duration_ms: RuntimeStateData::now_ms().saturating_sub(start_ms),
                node_id: self.node_id,
                reason: None,
                sequence: seq,
                is_forced: false,
            };
        }

        for obs in &observers {
            obs.on_transition_committed(&committed_event);
        }

        Ok(committed_event)
    }

    /// Force a state transition bypassing validation (for emergency use only).
    /// This is intended for supervisor-initiated actions like shutdown or recovery.
    /// Does NOT go through validate_transition.
    pub(crate) fn force_transition(&self, to: RuntimeState) -> TransitionResult {
        let start_ms = RuntimeStateData::now_ms();
        let observers = self.clone_observers();

        let committed_event;
        {
            let mut data = self.state.lock().unwrap_or_else(|e| e.into_inner());
            let from = data.current;
            data.previous = Some(from);
            data.current = to;
            data.entered_at_ms = RuntimeStateData::now_ms();
            data.transition_count += 1;
            data.sequence += 1;
            let count = data.transition_count;
            let seq = data.sequence;

            committed_event = TransitionResult {
                from,
                to,
                timestamp_ms: RuntimeStateData::now_ms(),
                transition_count: count,
                duration_ms: RuntimeStateData::now_ms().saturating_sub(start_ms),
                node_id: self.node_id,
                reason: Some("force_transition".into()),
                sequence: seq,
                is_forced: true,
            };
        }

        for obs in &observers {
            obs.on_transition_committed(&committed_event);
        }

        committed_event
    }

    pub fn allowed_next_states(&self) -> Vec<RuntimeState> {
        self.machine.allowed_next_states(self.current())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex as StdMutex;
    fn node() -> ValidatorId {
        ValidatorId([1u8; 32])
    }

    #[test]
    fn test_created_to_provisioning() {
        let sm = RuntimeStateMachine::new(node());
        let r = sm.transition(RuntimeState::Provisioning).unwrap();
        assert_eq!(sm.current(), RuntimeState::Provisioning);
        assert_eq!(r.transition_count, 1);
        assert!(!r.is_forced);
    }

    #[test]
    fn test_illegal_transition_rejected() {
        let sm = RuntimeStateMachine::new(node());
        assert!(sm.transition(RuntimeState::Voting).is_err());
        assert_eq!(sm.current(), RuntimeState::Created);
    }

    #[test]
    fn test_force_transition() {
        let sm = RuntimeStateMachine::new(node());
        let r = sm.force_transition(RuntimeState::Voting);
        assert_eq!(sm.current(), RuntimeState::Voting);
        assert!(r.is_forced);
    }

    #[test]
    fn test_full_path_to_voting() {
        let sm = RuntimeStateMachine::new(node());
        for s in [
            RuntimeState::Provisioning,
            RuntimeState::IdentityReady,
            RuntimeState::StorageReady,
            RuntimeState::Bootstrapping,
            RuntimeState::GenesisReady,
            RuntimeState::NetworkReady,
            RuntimeState::Discovering,
            RuntimeState::Syncing,
            RuntimeState::Verifying,
            RuntimeState::Candidate,
            RuntimeState::Voting,
        ] {
            assert!(sm.transition(s).is_ok());
        }
        assert_eq!(sm.transition_count(), 11);
    }

    #[test]
    fn test_committed_observer_sees_event_fields() {
        struct Obs {
            seen: StdMutex<Option<(RuntimeState, RuntimeState)>>,
        }
        impl TransitionObserver for Obs {
            fn on_transition_committed(&self, e: &TransitionResult) {
                *self.seen.lock().unwrap() = Some((e.from, e.to));
            }
            fn on_transition_failed(&self, _f: RuntimeState, _t: RuntimeState, _r: &str) {}
        }
        let o = Arc::new(Obs {
            seen: StdMutex::new(None),
        });
        let sm = RuntimeStateMachine::new(node());
        sm.attach_observer(o.clone());
        sm.transition(RuntimeState::Provisioning).unwrap();
        assert_eq!(
            *o.seen.lock().unwrap(),
            Some((RuntimeState::Created, RuntimeState::Provisioning))
        );
    }

    #[test]
    fn test_committed_observer_current_is_new() {
        struct Obs {
            sm: Arc<RuntimeStateMachine>,
            seen: StdMutex<Option<RuntimeState>>,
        }
        impl TransitionObserver for Obs {
            fn on_transition_committed(&self, _e: &TransitionResult) {
                *self.seen.lock().unwrap() = Some(self.sm.current());
            }
            fn on_transition_failed(&self, _f: RuntimeState, _t: RuntimeState, _r: &str) {}
        }
        let sm = Arc::new(RuntimeStateMachine::new(node()));
        let o = Arc::new(Obs {
            sm: sm.clone(),
            seen: StdMutex::new(None),
        });
        sm.attach_observer(o.clone());
        sm.transition(RuntimeState::Provisioning).unwrap();
        assert_eq!(*o.seen.lock().unwrap(), Some(RuntimeState::Provisioning));
    }

    #[test]
    fn test_failed_observer_notified() {
        struct Obs {
            failed: StdMutex<bool>,
        }
        impl TransitionObserver for Obs {
            fn on_transition_committed(&self, _e: &TransitionResult) {}
            fn on_transition_failed(&self, _f: RuntimeState, _t: RuntimeState, _r: &str) {
                *self.failed.lock().unwrap() = true;
            }
        }
        let o = Arc::new(Obs {
            failed: StdMutex::new(false),
        });
        let sm = RuntimeStateMachine::new(node());
        sm.attach_observer(o.clone());
        let _ = sm.transition(RuntimeState::Voting);
        assert!(*o.failed.lock().unwrap());
    }

    #[test]
    fn test_atomic_transition_under_contention() {
        use std::thread;
        let sm = Arc::new(RuntimeStateMachine::new(node()));
        sm.transition(RuntimeState::Provisioning).unwrap();
        let sm1 = sm.clone();
        let sm2 = sm.clone();
        let sm3 = sm.clone();
        let r1 = Arc::new(StdMutex::new(None));
        let r2 = Arc::new(StdMutex::new(None));
        let r3 = Arc::new(StdMutex::new(None));
        let r1c = r1.clone();
        let r2c = r2.clone();
        let r3c = r3.clone();
        let h1 = thread::spawn(move || {
            *r1c.lock().unwrap() = Some(sm1.transition(RuntimeState::IdentityReady));
        });
        let h2 = thread::spawn(move || {
            *r2c.lock().unwrap() = Some(sm2.transition(RuntimeState::IdentityReady));
        });
        let h3 = thread::spawn(move || {
            *r3c.lock().unwrap() = Some(sm3.transition(RuntimeState::IdentityReady));
        });
        h1.join().unwrap();
        h2.join().unwrap();
        h3.join().unwrap();
        let ok1 = r1
            .lock()
            .unwrap()
            .as_ref()
            .map(|r| r.is_ok())
            .unwrap_or(false);
        let ok2 = r2
            .lock()
            .unwrap()
            .as_ref()
            .map(|r| r.is_ok())
            .unwrap_or(false);
        let ok3 = r3
            .lock()
            .unwrap()
            .as_ref()
            .map(|r| r.is_ok())
            .unwrap_or(false);
        let succeeded = [ok1, ok2, ok3].iter().filter(|&&x| x).count();
        assert_eq!(
            succeeded, 1,
            "Exactly one must succeed: ok1={} ok2={} ok3={}",
            ok1, ok2, ok3
        );
    }

    #[test]
    fn test_no_deadlock_observer_reads_state() {
        struct Obs {
            sm: Arc<RuntimeStateMachine>,
        }
        impl TransitionObserver for Obs {
            fn on_transition_committed(&self, _e: &TransitionResult) {
                let _ = self.sm.current();
            }
            fn on_transition_failed(&self, _f: RuntimeState, _t: RuntimeState, _r: &str) {}
        }
        let sm = Arc::new(RuntimeStateMachine::new(node()));
        sm.attach_observer(Arc::new(Obs { sm: sm.clone() }));
        assert!(sm.transition(RuntimeState::Provisioning).is_ok());
    }

    #[test]
    fn test_observer_adds_observer_during_callback() {
        struct Recursive {
            sm: Arc<RuntimeStateMachine>,
            added: StdMutex<bool>,
        }
        impl TransitionObserver for Recursive {
            fn on_transition_committed(&self, _e: &TransitionResult) {
                let mut a = self.added.lock().unwrap();
                if !*a {
                    *a = true;
                    struct I;
                    impl TransitionObserver for I {
                        fn on_transition_committed(&self, _e: &TransitionResult) {}
                        fn on_transition_failed(
                            &self,
                            _f: RuntimeState,
                            _t: RuntimeState,
                            _r: &str,
                        ) {
                        }
                    }
                    self.sm.attach_observer(Arc::new(I));
                }
            }
            fn on_transition_failed(&self, _f: RuntimeState, _t: RuntimeState, _r: &str) {}
        }
        let sm = Arc::new(RuntimeStateMachine::new(node()));
        sm.attach_observer(Arc::new(Recursive {
            sm: sm.clone(),
            added: StdMutex::new(false),
        }));
        assert!(sm.transition(RuntimeState::Provisioning).is_ok());
    }
}
