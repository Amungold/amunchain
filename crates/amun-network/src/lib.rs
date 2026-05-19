#![no_std]
#![deny(clippy::unwrap_used)]

pub mod peer;
pub mod discovery;
pub mod connection;
pub mod framing;
pub mod heartbeat;
pub mod rate_limit;
pub mod handshake;
pub mod constants;

pub use peer::{Peer, PeerState};
pub use discovery::Discovery;
pub use connection::Connection;
pub use framing::{Frame, MessageType};
pub use heartbeat::Heartbeat;
pub use rate_limit::RateLimiter;
pub use handshake::Handshake;
pub use constants::*;

#[cfg(test)]
mod tests;
