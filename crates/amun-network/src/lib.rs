#![no_std]
#![deny(clippy::unwrap_used)]

pub mod connection;
pub mod constants;
pub mod discovery;
pub mod framing;
pub mod handshake;
pub mod heartbeat;
pub mod peer;
pub mod rate_limit;

pub use connection::Connection;
pub use constants::*;
pub use discovery::Discovery;
pub use framing::{Frame, MessageType};
pub use handshake::Handshake;
pub use heartbeat::Heartbeat;
pub use peer::{Peer, PeerState};
pub use rate_limit::RateLimiter;

#[cfg(test)]
mod tests;
