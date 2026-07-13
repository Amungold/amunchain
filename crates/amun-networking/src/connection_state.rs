use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Handshaking,
    Connected,
    Closing,
    Backoff,
    Dead,
}

#[derive(Debug)]
pub struct ConnectionInfo {
    pub state: ConnectionState,
    pub state_changed_at: Instant,
    pub established_at: Option<Instant>,
    pub disconnect_count: u64,
    pub last_error: Option<String>,
}

impl ConnectionInfo {
    pub fn new() -> Self {
        Self {
            state: ConnectionState::Connecting,
            state_changed_at: Instant::now(),
            established_at: None,
            disconnect_count: 0,
            last_error: None,
        }
    }

    pub fn transition(&mut self, new_state: ConnectionState) {
        let old_state = self.state;
        self.state = new_state;
        self.state_changed_at = Instant::now();

        match new_state {
            ConnectionState::Connected => {
                self.established_at = Some(Instant::now());
            }
            ConnectionState::Backoff | ConnectionState::Dead => {
                self.disconnect_count += 1;
            }
            _ => {}
        }

        if old_state != new_state {
            eprintln!(
                "Connection state transition: {:?} -> {:?}",
                old_state, new_state
            );
        }
    }

    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error);
    }
}
