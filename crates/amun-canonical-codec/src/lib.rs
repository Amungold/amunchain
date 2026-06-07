#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]
pub mod constants;
pub mod hasher;
pub mod reader;
pub mod writer;

pub use constants::*;
pub use hasher::CanonicalHasher;
pub use reader::CanonicalReader;
pub use writer::CanonicalWriter;

pub const CANONICAL_CODEC_VERSION: u16 = 1;
pub const MAX_CANONICAL_ALLOCATION: u64 = 64 * 1024 * 1024; // 64MB constitutional limit
