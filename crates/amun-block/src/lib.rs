#![no_std]
#![cfg_attr(not(test), deny(clippy::unwrap_used))]

pub mod block;
pub mod body;
pub mod header;
pub mod limits;

pub use block::{Block, BlockId};
pub use body::BlockBody;
pub use header::BlockHeader;
pub use limits::BlockLimits;

#[cfg(test)]
mod tests;
