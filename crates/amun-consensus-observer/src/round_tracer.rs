use crate::events::ConsensusEvent;
use crate::observer::ConsensusObserver;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::time::Instant;

const TRACE_BUFFER_SIZE: usize = 256;

#[derive(Debug, Clone)]
pub struct TracedEvent {
    pub event: ConsensusEvent,
    pub monotonic_us: u64,
}

pub struct RoundTracer {
    buffer: Mutex<VecDeque<TracedEvent>>,
    start_instant: Instant,
}

impl RoundTracer {
    pub fn new() -> Self {
        Self {
            buffer: Mutex::new(VecDeque::with_capacity(TRACE_BUFFER_SIZE)),
            start_instant: Instant::now(),
        }
    }

    pub fn events(&self) -> Vec<TracedEvent> {
        let buf = self.buffer.lock();
        buf.iter().cloned().collect()
    }

    pub fn event_count(&self) -> usize {
        self.buffer.lock().len()
    }

    pub fn generate_report(&self) -> String {
        let buf = self.buffer.lock();
        if buf.is_empty() {
            return "NO EVENTS RECORDED".into();
        }

        let mut report = String::new();
        let base_time = buf[0].monotonic_us;

        for (i, traced) in buf.iter().enumerate() {
            let relative_us = traced.monotonic_us.saturating_sub(base_time);
            report.push_str(&format!(
                "{:3}. {:?} @ {}μs\n",
                i + 1,
                traced.event.event,
                relative_us,
            ));
        }

        report
    }
}

impl ConsensusObserver for RoundTracer {
    fn name(&self) -> &str {
        "RoundTracer"
    }

    fn on_event(&self, event: &ConsensusEvent) {
        let traced = TracedEvent {
            event: event.clone(),
            monotonic_us: self.start_instant.elapsed().as_micros() as u64,
        };

        let mut buf = self.buffer.lock();
        if buf.len() >= TRACE_BUFFER_SIZE {
            buf.pop_front();
        }
        buf.push_back(traced);
    }

    fn reset(&self) {
        self.buffer.lock().clear();
    }
}

impl Default for RoundTracer {
    fn default() -> Self {
        Self::new()
    }
}
