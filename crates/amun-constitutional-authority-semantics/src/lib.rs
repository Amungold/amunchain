//! Constitutional Authority Semantics
//!
//! This crate defines what it means to possess authority within
//! the AmunChain constitutional civilisation.  Authority is not
//! a role assignment — it is a witnessed, delegable, revocable,
//! time-bound constitutional capability.

pub mod capability;
pub mod delegation;
pub mod institution;
pub mod revocation;

pub use capability::{AuthorityCapability, CapabilityWitness};
pub use delegation::{DelegationChain, DelegationProof};
pub use institution::{InstitutionalActor, InstitutionalWitness};
pub use revocation::{RevocationRegistry, RevocationWitness};
