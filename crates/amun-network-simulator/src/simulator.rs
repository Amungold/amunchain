use crate::delivery::DeliverySchedule;
use crate::adversary::AdversaryConfig;
use amun_deterministic_timer::DeterministicTimerWheel;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ScheduledMessage {
    pub id: u64,
    pub delivery_round: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkSimulator {
    pub schedule: DeliverySchedule,
    pub adversary: AdversaryConfig,
    timer: DeterministicTimerWheel,
    message_map: BTreeMap<u64, u64>,
    pub messages_scheduled: u64,
    pub messages_delivered: u64,
    pub messages_dropped: u64,
    next_msg_id: u64,
}

impl NetworkSimulator {
    pub fn new(schedule: DeliverySchedule, adversary: AdversaryConfig) -> Self {
        Self {
            schedule,
            adversary,
            timer: DeterministicTimerWheel::new(),
            message_map: BTreeMap::new(),
            messages_scheduled: 0,
            messages_delivered: 0,
            messages_dropped: 0,
            next_msg_id: 0,
        }
    }

    pub fn schedule_message(&mut self, sender: u64, receiver: u64, round: u64) -> ScheduledMessage {
        let base_delay = self.schedule.latency_for(sender, receiver);
        let adv_delay = self.adversary.additional_delay(sender, receiver, round);
        let delivery_round = round + base_delay + adv_delay;
        let msg_id = self.next_msg_id;
        self.next_msg_id += 1;
        self.messages_scheduled += 1;
        
        let timer_id = self.timer.schedule(delivery_round);
        self.message_map.insert(timer_id, msg_id);

        ScheduledMessage { id: msg_id, delivery_round }
    }

    pub fn tick(&mut self) -> Vec<u64> {
        let fired = self.timer.advance();
        let mut delivered = Vec::new();

        for timer_id in fired {
            if let Some(msg_id) = self.message_map.remove(&timer_id) {
                if self.adversary.should_drop(msg_id) {
                    self.messages_dropped += 1;
                } else {
                    self.messages_delivered += 1;
                    delivered.push(msg_id);
                }
            }
        }

        delivered
    }

    pub fn current_round(&self) -> u64 { self.timer.current_round() }
}
