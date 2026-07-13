#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    ProposalBroadcast,
    ProcessInbox,
    CheckTimeout,
    DeliverPending,
}

#[derive(Debug, Clone)]
pub struct ScheduledEvent {
    pub time: u64,
    pub node_id: String,
    pub event_type: EventType,
    id: u64,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .time
            .cmp(&self.time)
            .then_with(|| other.id.cmp(&self.id))
    }
}
impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.id == other.id
    }
}
impl Eq for ScheduledEvent {}

pub trait SchedulingPolicy {
    fn transform_time(&self, time: u64, _event_type: &EventType) -> u64 {
        time
    }
    fn filter_event(&self, _event: &ScheduledEvent) -> bool {
        true
    }
}

pub struct DefaultPolicy;
impl SchedulingPolicy for DefaultPolicy {}

pub struct EventScheduler {
    queue: BinaryHeap<ScheduledEvent>,
    next_id: u64,
    policy: Option<Box<dyn SchedulingPolicy>>,
}

impl EventScheduler {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            next_id: 0,
            policy: None,
        }
    }

    pub fn with_policy(policy: Box<dyn SchedulingPolicy>) -> Self {
        Self {
            queue: BinaryHeap::new(),
            next_id: 0,
            policy: Some(policy),
        }
    }

    pub fn schedule(&mut self, time: u64, node_id: String, event_type: EventType) {
        let actual_time = match &self.policy {
            Some(p) => p.transform_time(time, &event_type),
            None => time,
        };
        let event = ScheduledEvent {
            time: actual_time,
            node_id,
            event_type,
            id: self.next_id,
        };
        self.next_id += 1;
        let should_push = match &self.policy {
            Some(p) => p.filter_event(&event),
            None => true,
        };
        if should_push {
            self.queue.push(event);
        }
    }

    pub fn next_event(&mut self) -> Option<ScheduledEvent> {
        self.queue.pop()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
