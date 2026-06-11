#![allow(dead_code)]
use amun_networking::envelope::Envelope;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[derive(Debug, Clone)]
pub struct DeliveryPolicy {
    pub loss_rate: f64,
    pub base_delay_ms: u64,
    pub jitter_ms: u64,
}

impl Default for DeliveryPolicy {
    fn default() -> Self {
        Self {
            loss_rate: 0.0,
            base_delay_ms: 0,
            jitter_ms: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DelayedEnvelope {
    pub delivery_time: u64,
    pub recipient: String,
    pub envelope: Envelope,
}

pub struct MessageDeliveryEngine {
    pub pending: Vec<DelayedEnvelope>,
    policy: DeliveryPolicy,
    rng: StdRng,
}

impl MessageDeliveryEngine {
    pub fn new(policy: DeliveryPolicy, seed: u64) -> Self {
        Self {
            pending: Vec::new(),
            policy,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn schedule(&mut self, delivery_time: u64, recipient: String, envelope: Envelope) {
        self.pending.push(DelayedEnvelope {
            delivery_time,
            recipient,
            envelope,
        });
    }

    pub fn broadcast(
        &mut self,
        sender_id: &str,
        current_time: u64,
        envelope: Envelope,
        include_self: bool,
        node_ids: &[String],
    ) -> u64 {
        let jitter = if self.policy.jitter_ms > 0 {
            self.rng.gen_range(0..=self.policy.jitter_ms)
        } else {
            0
        };
        let delivery_time = current_time + self.policy.base_delay_ms + jitter;

        for recipient_id in node_ids {
            if recipient_id == sender_id && !include_self {
                continue;
            }
            if self.policy.loss_rate > 0.0 && self.rng.gen::<f64>() < self.policy.loss_rate {
                continue;
            }
            self.schedule(delivery_time, recipient_id.clone(), envelope.clone());
            eprintln!("  SCHEDULED to={} at={}", recipient_id, delivery_time);
        }
        delivery_time
    }

    /// Drain pending messages, deliver those whose time has come.
    /// Returns remaining (undelivered) messages.
    pub fn drain_ready(&mut self, current_time: u64) -> Vec<DelayedEnvelope> {
        let mut remaining = Vec::new();
        let mut delivered = Vec::new();
        for msg in self.pending.drain(..) {
            if msg.delivery_time <= current_time {
                delivered.push(msg);
            } else {
                remaining.push(msg);
            }
        }
        self.pending = remaining;
        delivered
    }
}
