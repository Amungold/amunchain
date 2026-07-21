use crate::events::ConsensusEvent;

pub trait ConsensusObserver: Send + Sync {
    fn name(&self) -> &str;
    fn on_event(&self, event: &ConsensusEvent);
    fn reset(&self) {}
    fn flush(&self) {}
    fn priority(&self) -> u8 {
        100
    }
}

pub trait ReplayableObserver: ConsensusObserver {
    fn replay(&self, events: &[ConsensusEvent]) {
        for event in events {
            self.on_event(event);
        }
    }
}

pub trait MetricsObserver: ConsensusObserver {
    fn export_prometheus(&self) -> String;
    fn reset_metrics(&self);
}
