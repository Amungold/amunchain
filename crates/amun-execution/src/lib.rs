#![cfg_attr(not(test), deny(clippy::unwrap_used))]
// Deterministic execution runtime.
#![no_std]

pub mod verified_interpreter;
pub mod wasm_deterministic;
pub mod wasm_profile;

pub use verified_interpreter::{ExecutionError, VerifiedInterpreter};
pub use wasm_deterministic::wasm_deterministic_subset;
pub use wasm_profile::{
    DeterministicWasmProfile, InstructionGasTable, CANONICAL_NAN_32, CANONICAL_NAN_64,
};
#[cfg(test)]
mod tests;
