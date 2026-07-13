// Sole unsafe boundary for the sovereign kernel.
// No other crate in the system may contain unsafe blocks.
// Every unsafe operation is encapsulated behind a verified safe abstraction.

#![no_std]

pub mod guard;
pub mod slot;

pub use guard::InitGuard;
pub use slot::RawSlot;

// Unsafe marker trait for types with verified deterministic layout.
// This is the ONLY place unsafe traits are permitted.
#[allow(clippy::missing_safety_doc)]
pub unsafe trait StableLayout: Sized {}
unsafe impl StableLayout for u8 {}
unsafe impl StableLayout for u16 {}
unsafe impl StableLayout for u32 {}
unsafe impl StableLayout for u64 {}
unsafe impl StableLayout for [u8; 32] {}
#[cfg(test)]
mod tests;
